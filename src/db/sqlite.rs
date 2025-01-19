use log::info;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite, SqlitePool};
use utils::env_util::{var_as_int_or, var_as_str_or};

use crate::{setup::errors::SetupError, utils};

const SQLITE_IN_MEMORY: &str = "sqlite::memory:";

type StdError = dyn std::error::Error;

impl From<sqlx::Error> for SetupError {
    fn from(err: sqlx::Error) -> SetupError {
        SetupError::Database(err)
    }
}

impl From<Box<StdError>> for SetupError {
    fn from(err: Box<StdError>) -> SetupError {
        SetupError::InvalidConfig(err.to_string())
    }
}

pub fn get_conn_str() -> String {
    let db_url = var_as_str_or("DATABASE_URL", SQLITE_IN_MEMORY);
    var_as_str_or("RW_SQLITE_URL", &db_url)
}

pub async fn init_db() -> Result<SqlitePool, Box<StdError>> {
    // Check the environment
    let max_conn = var_as_int_or("RW_SQLITE_MAX_CONN", 8) as u32; // Adjust the connection limit as needed
    let conn_url = get_conn_str();
    info!("Connecting to {} ...", conn_url);

    if conn_url != SQLITE_IN_MEMORY && !Sqlite::database_exists(&conn_url).await.unwrap_or(false) {
        info!("Creating database {} ...", conn_url);
        Sqlite::create_database(&conn_url).await?;
    }

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
