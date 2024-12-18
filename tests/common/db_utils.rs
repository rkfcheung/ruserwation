use ruserwation::{
    admin::sqlite::SqliteAdminRepo,
    config::models::AppState,
    db::{self, sqlite::get_conn_str},
    setup::startup::init,
};
use sqlx::{Sqlite, SqlitePool};
use std::{env, sync::Arc};

pub(crate) async fn init_conn() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePool::connect(&get_conn_str()).await?;

    Ok(pool)
}

pub(crate) async fn init_test_db() -> Result<SqlitePool, sqlx::Error> {
    let pool = init_conn().await?;
    db::sqlite::migrate_db(&pool).await?;

    Ok(pool)
}

pub(crate) async fn init_test_app_state() -> Arc<AppState<Sqlite, SqliteAdminRepo>> {
    // Initialise test database
    init_test_db().await.unwrap();

    // Initialise application state for testing
    env::set_var("RW_ADMIN_PASSWORD", "localtest");
    init().await.unwrap()
}
