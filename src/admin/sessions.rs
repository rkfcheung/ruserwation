use log::warn;
use std::{
    collections::HashMap,
    future::Future,
    sync::{Arc, Mutex},
    time::Duration,
};
use warp_sessions::Session;

const DEFAULT_EXPIRE_IN: u64 = 43_200;

pub trait EnableSession {
    type Error;

    fn create_session(
        &self,
        username: &str,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send;

    fn destroy_session(&self, session_id: &str) -> impl Future<Output = ()> + Send;

    fn get_session(
        &self,
        session_id: &str,
    ) -> impl Future<Output = Result<Session, Self::Error>> + Send;
}

#[derive(Default)]
pub struct Sessions {
    // Session ID to Session mapping
    context: Arc<Mutex<HashMap<String, Session>>>,
    default_expire_in: Duration,
}

impl Sessions {
    pub fn new() -> Self {
        Self {
            context: Arc::new(Mutex::new(HashMap::new())),
            default_expire_in: Duration::from_secs(DEFAULT_EXPIRE_IN),
        }
    }

    pub fn create(&self, username: &str) -> Option<String> {
        self.create_with_expire_in(username, self.default_expire_in)
    }

    pub fn create_with_expire_in(&self, username: &str, expire_in: Duration) -> Option<String> {
        // Create a new session
        let mut session = Session::new();
        session.insert_raw("user", username.to_string());
        session.expire_in(expire_in);

        // Lock the sessions HashMap and insert the new session
        let mut sessions = self.context.lock().ok()?;
        let session_id = session.id().to_owned();

        // Insert the session into the HashMap and return session_id
        sessions.insert(session_id.clone(), session);
        Some(session_id) // Return the session_id if successful
    }

    pub fn destroy(&self, session_id: &str) -> bool {
        // Attempt to lock the sessions HashMap
        if let Ok(mut sessions) = self.context.lock() {
            // Successfully locked, so proceed to remove the session
            sessions.remove(session_id);
            true
        } else {
            // Locking failed, handle the error (e.g., log or return)
            warn!(
                "Failed to lock sessions while destroying session: {}",
                session_id
            );
            false
        }
    }

    pub fn get(&self, session_id: &str) -> Option<Session> {
        let mut sessions = self.context.lock().ok()?;
        if let Some(session) = sessions.get(session_id) {
            if session.is_expired() {
                sessions.remove(session_id);

                None
            } else {
                Some(session.clone())
            }
        } else {
            None
        }
    }
}
