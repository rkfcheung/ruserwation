use sqlx::SqlitePool;
use std::sync::Arc;

use crate::{
    admin::{
        repo::{AdminRepo, VerifyUser},
        sessions::{EnableSession, SessionManager},
        sqlite::SqliteAdminRepo,
    },
    reservation::sqlite::SqliteReservationRepo,
    restaurant::models::Restaurant,
};

pub struct AppState<ADMIN>
where
    ADMIN: AdminRepo + VerifyUser + Send + Sync,
{
    restaurant: Arc<Restaurant>,
    admin_repo: Arc<ADMIN>,
    reservation_repo: Arc<SqliteReservationRepo>,
    session_manager: Arc<SessionManager<ADMIN>>,
}

#[derive(Clone)]
pub struct Context<T> {
    context: Arc<T>,
    restaurant: Arc<Restaurant>,
}

#[derive(Default)]
pub struct SqliteAppStateBuilder {
    restaurant: Option<Arc<Restaurant>>,
    pool: Option<Arc<SqlitePool>>,
}

impl<ADMIN> AppState<ADMIN>
where
    ADMIN: AdminRepo + VerifyUser + Send + Sync,
{
    pub fn admin_repo(&self) -> Arc<ADMIN> {
        self.admin_repo.clone()
    }

    pub fn reservation_repo(&self) -> Arc<SqliteReservationRepo> {
        self.reservation_repo.clone()
    }

    pub fn restaurant(&self) -> Arc<Restaurant> {
        self.restaurant.clone()
    }

    pub fn session_manager(&self) -> Arc<impl EnableSession + VerifyUser + Send + Sync> {
        self.session_manager.clone()
    }
}

impl<T> Context<T> {
    pub fn create(context: Arc<T>, restaurant: Arc<Restaurant>) -> Arc<Self> {
        Arc::new(Self {
            context,
            restaurant,
        })
    }

    pub fn get(&self) -> Arc<T> {
        self.context.clone()
    }

    pub fn restaurant(&self) -> Arc<Restaurant> {
        self.restaurant.clone()
    }
}

impl SqliteAppStateBuilder {
    /// Set the restaurant
    pub fn with_restaurant(mut self, restaurant: Arc<Restaurant>) -> Self {
        self.restaurant = Some(restaurant);
        self
    }

    /// Set the database pool
    pub fn with_pool(mut self, pool: Arc<SqlitePool>) -> Self {
        self.pool = Some(pool);
        self
    }

    /// Build the `AppState`
    pub fn build(self) -> AppState<SqliteAdminRepo> {
        let restaurant = self.restaurant.expect("Restaurant is required");
        let pool = self.pool.expect("Database pool is required");
        let admin_repo = Arc::new(SqliteAdminRepo::new(pool.clone()));
        let reservation_repo = Arc::new(SqliteReservationRepo::new(pool.clone()));
        let session_manager = Arc::new(SessionManager::new(admin_repo.clone()));

        AppState {
            restaurant,
            admin_repo,
            reservation_repo,
            session_manager,
        }
    }
}
