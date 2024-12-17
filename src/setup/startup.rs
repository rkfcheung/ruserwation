use log::{error, info};
use std::sync::Arc;

use crate::{
    admin::{models::Admin, repo::AdminRepo, sqlite::SqliteAdminRepo},
    db,
};

#[derive(Debug)]
pub enum SetupError {
    Database(sqlx::Error), // For sqlx errors
    InvalidConfig(String), // For invalid configuration errors
    IO(std::io::Error),    // For I/O related errors
    Other(String),         // For any other kind of error
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetupError::Database(err) => write!(f, "Database error: {}", err),
            SetupError::InvalidConfig(msg) => write!(f, "Invalid config: {}", msg),
            SetupError::IO(err) => write!(f, "I/O error: {}", err),
            SetupError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl std::error::Error for SetupError {}

impl From<std::io::Error> for SetupError {
    fn from(err: std::io::Error) -> SetupError {
        SetupError::IO(err)
    }
}

pub async fn init() -> Result<Arc<SqliteAdminRepo>, SetupError> {
    env_logger::init();
    info!("Initialising Ruserwation ...");

    let pool = db::sqlite::init_db().await.map_err(SetupError::from)?;
    db::sqlite::migrate_db(&pool)
        .await
        .map_err(SetupError::from)?;

    let admin_repo = Arc::new(SqliteAdminRepo::new(Arc::new(pool)));
    init_admin(admin_repo.clone()).await?;

    Ok(admin_repo)
}

async fn init_admin(admin_repo: Arc<SqliteAdminRepo>) -> Result<(), SetupError> {
    let root_user = admin_repo.find_by_id(1).await;
    match root_user {
        Some(admin) => info!(
            "Last login for {}: {:?}",
            admin.username, admin.last_login_time
        ),
        None => {
            info!("Initialising Admin ...");
            let mut admin = Admin::init();
            let id = admin_repo.save(&mut admin).await;
            if id > 0 {
                info!("Admin is created.");
            } else {
                let err = "Failed to create Admin";
                error!("{}: {:?}", err, admin);

                return Err(SetupError::Other(err.to_string()));
            }
        }
    }

    Ok(())
}
