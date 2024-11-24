use log::info;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use utils::env_util::{var_as_int_or, var_as_str_or};

use crate::{setup::init::SetupError, utils};

type StdError = dyn std::error::Error;

impl From<sqlx::Error> for SetupError {
    fn from(err: sqlx::Error) -> SetupError {
        SetupError::DatabaseError(err)
    }
}

impl From<Box<StdError>> for SetupError {
    fn from(err: Box<StdError>) -> SetupError {
        SetupError::InvalidConfigError(err.to_string())
    }
}

pub async fn init_db() -> Result<SqlitePool, Box<StdError>> {
    // Check the environment
    let max_conn = var_as_int_or("RW_SQLITE_MAX_CONN", 8) as u32; // Adjust the connection limit as needed
    let conn_url = var_as_str_or("RW_SQLITE_URL", "sqlite::memory:".to_string());
    info!("Connecting to {} ...", conn_url);

    // Create the database connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(max_conn)
        .connect(&conn_url)
        .await?;
    info!("Connected: size={}", pool.size());

    Ok(pool)
}

pub async fn migrate_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    info!("Migrating DB ...");
    sqlx::migrate!().run(pool).await?;
    info!("Migrated");

    Ok(())
}
