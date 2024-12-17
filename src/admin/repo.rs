use log::warn;
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use warp_sessions::Session;

use super::models::Admin;

pub trait AdminRepo {
    // Find an Admin by id
    fn find_by_id(&self, id: u32) -> impl Future<Output = Option<Admin>> + Send;

    // Find an Admin by username
    fn find_by_username(&self, username: &str) -> impl Future<Output = Option<Admin>> + Send;

    // Save an Admin and return its ID
    fn save(&mut self, admin: &mut Admin) -> impl Future<Output = u32> + Send;

    // Verify username and password
    fn verify(&self, username: &str, password: &str) -> impl Future<Output = bool> + Send;
}

pub trait EnableSession<E> {
    fn create_session(
        &self,
        username: &str,
    ) -> impl std::future::Future<Output = Result<String, E>> + Send;
    fn destroy_session(&self, session_id: &str) -> impl std::future::Future<Output = ()> + Send;
    fn get_session(
        &self,
        session_id: &str,
    ) -> impl std::future::Future<Output = Result<Session, E>> + Send;
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
            default_expire_in: Duration::from_secs(43200),
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
