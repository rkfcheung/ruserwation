use ruserwation::{
    admin::{models::Admin, sqlite::SqliteAdminRepo},
    common::Repo,
    config::models::AppState,
    db::{self, sqlite::get_conn_str},
    setup::startup::init_app_state,
};
use sqlx::SqlitePool;
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

pub(crate) async fn init_test_app_state() -> Arc<AppState<SqliteAdminRepo>> {
    // Initialise test database
    let pool = init_test_db().await.unwrap();

    // Initialise application state for testing
    env::set_var("RW_SQLITE_URL", "sqlite::memory:");
    env::set_var("RW_ADMIN_PASSWORD", "localtest");

    // Create the AppState
    let app_state = init_app_state(pool.into());

    // Perform admin initialisation
    let mut admin = Admin::init();
    let admin_repo = app_state.admin_repo();
    let _ = &admin_repo.save(&mut admin).await;

    app_state
}
