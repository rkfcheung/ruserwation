use log::info;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::error::Error;
use utils::env_util::{var_as_int_or, var_as_str_or};

use crate::utils;

pub async fn init_db() -> Result<Pool<Sqlite>, Box<dyn Error>> {
    // Check the environment
    let max_conn = var_as_int_or("RW_SQLITE_MAX_CONN", 8) as u32; // Adjust the connection limit as needed
    let conn_url = var_as_str_or("RW_SQLITE_URL", "sqlite::memory:".to_string());
    info!("Connecting to {} ...", conn_url);

    // Create the database connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(max_conn)
        .connect(&conn_url)
        .await?;

    Ok(pool)
}

pub async fn migrate_db(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    info!("Migratinh DB ...");
    sqlx::migrate!().run(pool).await?;

    Ok(())
}
