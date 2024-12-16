use log::{error, info};

use crate::{
    admin::{models::Admin, repo::AdminRepo, sqlite::SqliteAdminRepo},
    db,
};

#[derive(Debug)]
pub enum SetupError {
    DatabaseError(sqlx::Error), // For sqlx errors
    InvalidConfigError(String), // For invalid configuration errors
    IoError(std::io::Error),    // For I/O related errors
    OtherError(String),         // For any other kind of error
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetupError::DatabaseError(err) => write!(f, "Database error: {}", err),
            SetupError::InvalidConfigError(msg) => write!(f, "Invalid config: {}", msg),
            SetupError::IoError(err) => write!(f, "I/O error: {}", err),
            SetupError::OtherError(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl std::error::Error for SetupError {}

impl From<std::io::Error> for SetupError {
    fn from(err: std::io::Error) -> SetupError {
        SetupError::IoError(err)
    }
}

pub async fn init() -> Result<(), SetupError> {
    env_logger::init();
    info!("Initialising Ruserwation ...");

    let pool = db::sqlite::init_db().await.map_err(SetupError::from)?;
    db::sqlite::migrate_db(&pool)
        .await
        .map_err(SetupError::from)?;

    let mut admin_repo = SqliteAdminRepo::new(&pool);
    init_admin(&mut admin_repo).await?;

    Ok(())
}

async fn init_admin(admin_repo: &mut SqliteAdminRepo<'_>) -> Result<(), SetupError> {
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

                return Err(SetupError::OtherError(err.to_string()));
            }
        }
    }

    Ok(())
}
