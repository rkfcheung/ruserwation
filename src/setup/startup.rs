use log::{error, info};
use sqlx::Sqlite;
use std::sync::Arc;

use crate::{
    admin::{models::Admin, repo::AdminRepo, sqlite::SqliteAdminRepo},
    config::models::AppState,
    db,
};

use super::errors::SetupError;

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

pub async fn init() -> Result<Arc<AppState<Sqlite, SqliteAdminRepo>>, SetupError> {
    // Initialise logging
    env_logger::init();
    info!("Initialising Ruserwation ...");

    // Initialise the SQLite database connection pool
    let pool = db::sqlite::init_db().await.map_err(SetupError::from)?;

    // Run database migrations
    db::sqlite::migrate_db(&pool)
        .await
        .map_err(SetupError::from)?;

    // Initialise the AdminRepo
    let pool = Arc::new(pool);
    let admin_repo = Arc::new(SqliteAdminRepo::new(pool.clone()));

    // Perform admin initialisation
    init_admin(admin_repo.clone()).await?;

    // Create the AppState
    let app_state = AppState::new(pool, admin_repo);

    Ok(Arc::new(app_state))
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
