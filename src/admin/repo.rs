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
    // TODO: Change to return Result<String, Error>
    fn create_session(&self, username: &str) -> String;

    fn destroy_session(&self, session_id: &str);

    fn get_session(&self, session_id: &str) -> Option<Session>;
}

pub struct Sessions {
    context: Arc<Mutex<HashMap<String, Session>>>, // Session ID to Session mapping
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

pub struct InMemoryAdminRepo {
    admins: Arc<Mutex<HashMap<String, Admin>>>, // Using username as the key
    sessions: Sessions,                         // Session ID to Session mapping
}

impl InMemoryAdminRepo {
    pub fn new() -> Self {
        Self {
            admins: Arc::new(Mutex::new(HashMap::new())),
            sessions: Sessions::new(),
        }
    }
}

impl EnableSession for InMemoryAdminRepo {
    fn create_session(&self, username: &str) -> String {
        let admins = self.admins.lock().unwrap();
        if admins.contains_key(username) {
            self.sessions.create(username)
        } else {
            panic!("Username not found");
        }
    }

    fn destroy_session(&self, session_id: &str) {
        self.sessions.destroy(session_id);
    }

    fn get_session(&self, session_id: &str) -> Option<Session> {
        self.sessions.get(session_id)
    }
}

impl AdminRepo for InMemoryAdminRepo {
    async fn find_by_id(&self, id: u32) -> Option<Admin> {
        let admins = self.admins.lock().unwrap();
        admins.values().find(|&admin| admin.id == id).cloned()
    }

    async fn find_by_username(&self, username: &str) -> Option<Admin> {
        let admins = self.admins.lock().unwrap();
        admins.get(username).cloned()
    }

    async fn save(&mut self, admin: &mut Admin) -> u32 {
        let mut admins = self.admins.lock().unwrap();
        admins.insert(admin.username.clone(), admin.clone());
        if admin.id == 0 {
            admin.id = admins.len() as u32 + 1;
        }

        admin.id
    }

    async fn verify(&self, username: &str, password: &str) -> bool {
        let admins = self.admins.lock().unwrap();
        if let Some(admin) = admins.get(username) {
            admin.verify_password(password)
        } else {
            false
        }
    }
}
