use ruserwation::{
    admin::{models::Admin, repo::AdminRepo, sqlite::SqliteAdminRepo},
    config::models::AppState,
    db::{self, sqlite::get_conn_str},
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
    let pool = init_test_db().await.unwrap();

    // Initialise application state for testing
    env::set_var("RW_SQLITE_URL", "sqlite::memory:");
    env::set_var("RW_ADMIN_PASSWORD", "localtest");

    // Initialise the AdminRepo
    let pool = Arc::new(pool);
    let admin_repo = Arc::new(SqliteAdminRepo::new(pool.clone()));

    // Perform admin initialisation
    let mut admin = Admin::init();
    let _ = &admin_repo.save(&mut admin).await;

    // Create the AppState
    let app_state = AppState::new(pool, admin_repo);

    Arc::new(app_state)
}
