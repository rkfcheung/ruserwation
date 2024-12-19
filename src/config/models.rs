use sqlx::Pool;
use std::sync::Arc;

use crate::{
    admin::{
        repo::{AdminRepo, VerifyUser},
        sessions::{EnableSession, SessionManager},
    },
    restaurant::models::Restaurant,
};

pub struct AppState<DB, R>
where
    DB: sqlx::Database,
    R: AdminRepo + VerifyUser + Send + Sync,
{
    restaurant: Arc<Restaurant>,
    pool: Arc<Pool<DB>>,
    admin_repo: Arc<R>,
    session_manager: Arc<SessionManager<R>>,
}

pub struct AppStateBuilder<DB, R>
where
    DB: sqlx::Database,
    R: AdminRepo + VerifyUser + Send + Sync,
{
    restaurant: Option<Arc<Restaurant>>,
    pool: Option<Arc<Pool<DB>>>,
    admin_repo: Option<Arc<R>>,
}

impl<DB, R> AppState<DB, R>
where
    DB: sqlx::Database,
    R: AdminRepo + VerifyUser + Send + Sync,
{
    pub fn new(
        restaurant: Arc<Restaurant>,
        pool: Arc<Pool<DB>>,
        admin_repo: Arc<R>,
        session_manager: Arc<SessionManager<R>>,
    ) -> Self {
        Self {
            restaurant,
            pool,
            admin_repo,
            session_manager,
        }
    }

    pub fn admin_repo(&self) -> Arc<R> {
        self.admin_repo.clone()
    }

    pub fn pool(&self) -> Arc<Pool<DB>> {
        self.pool.clone()
    }

    pub fn session_manager(&self) -> Arc<impl EnableSession + VerifyUser + Send + Sync> {
        self.session_manager.clone()
    }

    pub fn restaurant(&self) -> Arc<Restaurant> {
        self.restaurant.clone()
    }
}

impl<DB, R> AppStateBuilder<DB, R>
where
    DB: sqlx::Database,
    R: AdminRepo + VerifyUser + Send + Sync,
{
    /// Create a new builder instance
    pub fn new() -> Self {
        Self {
            restaurant: None,
            pool: None,
            admin_repo: None,
        }
    }

    /// Set the restaurant
    pub fn with_restaurant(mut self, restaurant: Arc<Restaurant>) -> Self {
        self.restaurant = Some(restaurant);
        self
    }

    /// Set the database pool
    pub fn with_pool(mut self, pool: Arc<Pool<DB>>) -> Self {
        self.pool = Some(pool);
        self
    }

    /// Set the admin repository
    pub fn with_admin_repo(mut self, admin_repo: Arc<R>) -> Self {
        self.admin_repo = Some(admin_repo);
        self
    }

    /// Build the `AppState`
    pub fn build(self) -> AppState<DB, R> {
        let restaurant = self.restaurant.expect("Restaurant is required");
        let pool = self.pool.expect("Database pool is required");
        let admin_repo = self.admin_repo.expect("Admin repo is required");
        let session_manager = Arc::new(SessionManager::new(admin_repo.clone()));

        AppState::new(restaurant, pool, admin_repo, session_manager)
    }
}
