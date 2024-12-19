use std::{future::Future, sync::Arc, time::Duration};
use warp_sessions::{MemoryStore, Session, SessionStore};

use super::{errors::SessionError, repo::AdminRepo};

const DEFAULT_EXPIRE_IN: u64 = 43_200;

pub trait EnableSession {
    fn create_session(
        &self,
        username: &str,
    ) -> impl Future<Output = Result<String, SessionError>> + Send;

    fn destroy_session(&self, session_id: &str) -> impl Future<Output = ()> + Send;

    fn get_session(
        &self,
        session_id: &str,
    ) -> impl Future<Output = Result<Session, SessionError>> + Send;
}

pub trait VerifyUser {
    // Check if user exists
    fn contains(&self, username: &str) -> impl Future<Output = bool> + Send;

    // Verify username and password
    fn verify(&self, username: &str, password: &str) -> impl Future<Output = bool> + Send;
}

pub struct Sessions {
    context: MemoryStore,
    default_expire_in: Duration,
}

impl Default for Sessions {
    fn default() -> Self {
        Self::new()
    }
}

impl Sessions {
    pub fn new() -> Self {
        Self {
            context: MemoryStore::new(),
            default_expire_in: Duration::from_secs(DEFAULT_EXPIRE_IN),
        }
    }

    pub async fn create(&self, username: &str) -> Option<String> {
        self.create_with_expire_in(username, self.default_expire_in)
            .await
    }

    pub async fn create_with_expire_in(
        &self,
        username: &str,
        expire_in: Duration,
    ) -> Option<String> {
        // Create a new session
        let mut session = Session::new();
        session.insert_raw("user", username.to_string());
        session.expire_in(expire_in);

        self.context
            .store_session(session)
            .await
            .unwrap_or_default()
    }

    pub async fn destroy(&self, session_id: &str) -> bool {
        if let Some(session) = self.get_session(session_id).await {
            match self.context.destroy_session(session).await {
                Ok(_) => {
                    log::info!("Session {} destroyed successfully.", session_id);
                    true
                }
                Err(err) => {
                    log::warn!("Session {} failed to destroy: {}", session_id, err);
                    false
                }
            }
        } else {
            log::warn!("Session {} not found during destruction.", session_id);
            false
        }
    }

    pub async fn get(&self, session_id: &str) -> Option<Session> {
        let session = self.get_session(session_id).await?;

        if session.is_expired() {
            log::info!("Session {} has expired.", session_id);
            let _ = self.context.destroy_session(session).await;
            return None;
        }

        Some(session.clone())
    }

    async fn get_session(&self, session_id: &str) -> Option<Session> {
        self.context
            .load_session(session_id.to_string())
            .await
            .unwrap_or_default()
    }
}

pub struct SessionManager<R>
where
    R: AdminRepo + EnableSession + Send + Sync,
{
    admin_repo: Arc<R>,
}

impl<R> SessionManager<R>
where
    R: AdminRepo + EnableSession + Send + Sync,
{
    pub fn new(admin_repo: Arc<R>) -> Self {
        Self { admin_repo }
    }

    pub async fn verify(&self, username: &str, password: &str) -> bool {
        self.admin_repo.verify(username, password).await
    }
}

impl<R> EnableSession for SessionManager<R>
where
    R: AdminRepo + EnableSession + Send + Sync,
{
    async fn create_session(&self, username: &str) -> Result<String, SessionError> {
        self.admin_repo.create_session(username).await
    }

    async fn destroy_session(&self, session_id: &str) {
        self.admin_repo.destroy_session(session_id).await
    }

    async fn get_session(&self, session_id: &str) -> Result<Session, SessionError> {
        self.admin_repo.get_session(session_id).await
    }
}

impl<R> VerifyUser for SessionManager<R>
where
    R: AdminRepo + EnableSession + Send + Sync,
{
    async fn contains(&self, username: &str) -> bool {
        self.admin_repo.find_by_username(username).await.is_some()
    }

    async fn verify(&self, username: &str, password: &str) -> bool {
        self.admin_repo.verify(username, password).await
    }
}
