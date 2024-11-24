use log::info;

use crate::db;

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

    Ok(())
}
