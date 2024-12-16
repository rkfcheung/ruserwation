use ruserwation::db::{self, sqlite::SQLITE_IN_MEMORY};
use sqlx::SqlitePool;

pub async fn init_conn() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePool::connect(SQLITE_IN_MEMORY).await?;

    Ok(pool)
}

pub async fn init_test_db() -> Result<SqlitePool, sqlx::Error> {
    let pool = init_conn().await?;
    db::sqlite::migrate_db(&pool).await?;

    Ok(pool)
}
