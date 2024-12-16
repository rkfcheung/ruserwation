use log::info;
use ruserwation::db::{self, sqlite::get_conn_str};
use sqlx::SqlitePool;

pub async fn init_conn() -> Result<SqlitePool, sqlx::Error> {
    let conn_url = get_conn_str();
    info!("Connecting to {} ...", conn_url);
    let pool = SqlitePool::connect(&conn_url).await?;

    Ok(pool)
}

pub async fn init_test_db() -> Result<SqlitePool, sqlx::Error> {
    let pool = init_conn().await?;
    db::sqlite::migrate_db(&pool).await?;

    Ok(pool)
}
