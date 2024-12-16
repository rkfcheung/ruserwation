use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex};
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

pub trait EnableSession {
    fn create_session(
        &self,
        username: &str,
    ) -> impl std::future::Future<Output = Result<String, String>> + Send;
    fn destroy_session(&self, session_id: &str) -> impl std::future::Future<Output = ()> + Send;
    fn get_session(
        &self,
        session_id: &str,
    ) -> impl std::future::Future<Output = Result<Session, String>> + Send;
}

pub struct Sessions {
    // Session ID to Session mapping
    context: Arc<Mutex<HashMap<String, Session>>>,
}

impl Sessions {
    pub fn new() -> Self {
        Self {
            context: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create(&self, username: &str) -> String {
        // Create a new session
        let mut session = Session::new();
        session.insert_raw("user", username.to_string());

        // Lock the sessions HashMap and insert the new session
        let mut sessions = self.context.lock().unwrap();
        let session_id = session.id().to_owned();
        sessions.insert(session_id.clone(), session);

        // Return the session ID
        session_id
    }

    pub fn destroy(&self, session_id: &str) {
        let mut sessions = self.context.lock().unwrap();
        sessions.remove(session_id);
    }

    pub fn get(&self, session_id: &str) -> Option<Session> {
        let sessions = self.context.lock().unwrap();
        sessions.get(session_id).cloned()
    }
}
