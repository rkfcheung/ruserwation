use std::{future::Future, sync::Arc, time::Duration};
use warp_sessions::{MemoryStore, Session, SessionStore};

use super::{errors::SessionError, repo::VerifyUser};

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

pub struct Sessions {
    context: MemoryStore,
    default_expire_in: Duration,
}

impl Default for Sessions {
    fn default() -> Self {
        Self {
            context: MemoryStore::new(),
            default_expire_in: Duration::from_secs(DEFAULT_EXPIRE_IN),
        }
    }
}

impl Sessions {
    pub async fn create(&self, username: &str) -> Result<String, SessionError> {
        self.create_with_expire_in(username, self.default_expire_in)
            .await
    }

    pub async fn create_with_expire_in(
        &self,
        username: &str,
        expire_in: Duration,
    ) -> Result<String, SessionError> {
        // Create a new session
        let mut session = Session::new();
        session.insert_raw("user", username.to_string());
        session.expire_in(expire_in);

        let result = self
            .context
            .store_session(session)
            .await
            .map_err(|err| SessionError::StoreException(err.to_string()))?;
        if let Some(session_id) = result {
            Ok(session_id)
        } else {
            Err(SessionError::SessionCreationFailed(username.to_string()))
        }
    }

    pub async fn destroy(&self, session_id: &str) -> bool {
        if let Ok(session) = self.get_session(session_id).await {
            match self.context.destroy_session(session).await {
                Ok(_) => {
                    log::info!("Session {} destroyed successfully.", session_id);
                    true
                }
                Err(err) => {
                    log::warn!("Failed to destroy session {}: {}", session_id, err);
                    false
                }
            }
        } else {
            log::warn!("Session {} not found during destruction.", session_id);
            false
        }
    }

    pub async fn get(&self, session_id: &str) -> Result<Session, SessionError> {
        let session = self.get_session(session_id).await?;

        if session.is_expired() {
            log::info!("Session {} has expired and will be destroyed.", session_id);
            let _ = self.context.destroy_session(session).await;

            Err(SessionError::SessionExpired(session_id.to_string()))
        } else {
            Ok(session)
        }
    }

    async fn get_session(&self, session_id: &str) -> Result<Session, SessionError> {
        let result = self
            .context
            .load_session(session_id.to_string())
            .await
            .map_err(|err| SessionError::StoreException(err.to_string()))?;
        if let Some(session) = result {
            Ok(session)
        } else {
            Err(SessionError::SessionNotFound(session_id.to_string()))
        }
    }
}

pub struct SessionManager<R>
where
    R: VerifyUser + Send + Sync,
{
    user_store: Arc<R>,
    sessions: Sessions,
}

impl<R> SessionManager<R>
where
    R: VerifyUser + Send + Sync,
{
    pub fn new(user_store: Arc<R>) -> Self {
        Self {
            user_store,
            sessions: Sessions::default(),
        }
    }
}

impl<R> EnableSession for SessionManager<R>
where
    R: VerifyUser + Send + Sync,
{
    async fn create_session(&self, username: &str) -> Result<String, SessionError> {
        if self.contains(username).await {
            self.sessions.create(username).await
        } else {
            Err(SessionError::UserNotFound(username.to_string()))
        }
    }

    async fn destroy_session(&self, session_id: &str) {
        self.sessions.destroy(session_id).await;
    }

    async fn get_session(&self, session_id: &str) -> Result<Session, SessionError> {
        self.sessions.get(session_id).await
    }
}

impl<R> VerifyUser for SessionManager<R>
where
    R: VerifyUser + Send + Sync,
{
    async fn contains(&self, username: &str) -> bool {
        self.user_store.contains(username).await
    }

    async fn verify(&self, username: &str, password: &str) -> bool {
        self.user_store.verify(username, password).await
    }
}
