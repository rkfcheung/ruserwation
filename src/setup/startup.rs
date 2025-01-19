use log::{error, info};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

use crate::{
    admin::{
        models::{Admin, ROOT_ADMIN_ID},
        sqlite::SqliteAdminRepo,
    },
    common::Repo,
    config::models::{AppState, SqliteAppStateBuilder},
    db,
    restaurant::models::Restaurant,
    utils::env_util::{var_as_int_or, var_as_str_or},
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

pub async fn init() -> Result<Arc<AppState<SqliteAdminRepo>>, SetupError> {
    // Initialise logging
    env_logger::init();
    info!("Initialising Ruserwation ...");

    // Initialise the SQLite database connection pool
    let pool = db::sqlite::init_db().await.map_err(SetupError::from)?;

    // Run database migrations
    db::sqlite::migrate_db(&pool)
        .await
        .map_err(SetupError::from)?;

    // Create the AppState
    let app_state = init_app_state(pool.into());

    // Perform admin initialisation
    let admin_repo = &app_state.as_ref().admin_repo();
    init_admin(admin_repo.clone()).await?;

    Ok(app_state)
}

pub fn init_app_state(pool: Arc<Pool<Sqlite>>) -> Arc<AppState<SqliteAdminRepo>> {
    // Perform restaurant initilisation
    let restaurant = Arc::new(init_restaurant());

    let app_state = SqliteAppStateBuilder::default()
        .with_restaurant(restaurant)
        .with_pool(pool)
        .build();

    Arc::new(app_state)
}

async fn init_admin(admin_repo: Arc<SqliteAdminRepo>) -> Result<(), SetupError> {
    let root_user = admin_repo.find_by_id(ROOT_ADMIN_ID).await;
    match root_user {
        Some(admin) => info!(
            "Last login for {}: {:?}",
            admin.username, admin.last_login_time
        ),
        None => {
            info!("Initialising Admin ...");
            let mut admin = Admin::init();
            let result = admin_repo.save(&mut admin).await;
            match result {
                Ok(id) => {
                    info!("Admin [{id}] is created.");
                }
                Err(e) => {
                    let err = "Failed to create Admin";
                    error!("{err} [{:?}]: {:?}", e, admin);

                    return Err(SetupError::Other(err.to_string()));
                }
            }
        }
    }

    Ok(())
}

fn init_restaurant() -> Restaurant {
    let rest_name = var_as_str_or("RW_REST_NAME", "<Name>");
    let rest_max_capacity = var_as_int_or("RW_REST_MAX_CAPACITY", 64) as u32;
    let rest_location = var_as_str_or("RW_REST_LOCATION", "<Location>");

    let restaurant = Restaurant::new(1, &rest_name, rest_max_capacity, &rest_location);
    info!("{:?}", restaurant);

    restaurant
}
