use sqlx::Pool;
use std::sync::Arc;

use crate::admin::{
    repo::AdminRepo,
    sessions::{EnableSession, SessionManager},
};

pub struct AppState<DB, R>
where
    DB: sqlx::Database,
    R: AdminRepo + EnableSession + Send + Sync,
{
    pool: Arc<Pool<DB>>,
    admin_repo: Arc<R>,
    session_manager: Arc<SessionManager<R>>,
}

impl<DB, R> AppState<DB, R>
where
    DB: sqlx::Database,
    R: AdminRepo + EnableSession + Send + Sync,
{
    pub fn new(pool: Arc<Pool<DB>>, admin_repo: Arc<R>) -> Self {
        let session_manager = Arc::new(SessionManager::new(admin_repo.clone()));

        Self {
            pool,
            admin_repo,
            session_manager,
        }
    }

    pub fn session_manager(&self) -> Arc<SessionManager<R>> {
        self.session_manager.clone()
    }
}
