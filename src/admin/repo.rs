use chrono::Utc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp_sessions::Session;

use super::models::Admin;

pub trait AdminRepo {
    fn init() -> Self; // Initialise the repository
    fn find_by_username(&self, username: &str) -> Option<Admin>; // Find an Admin by username
    fn save(&mut self, admin: Admin) -> u32; // Save an Admin and return its ID
    fn verify(&self, username: &str, password: &str) -> bool; // Verify username and password
}

pub struct InMemoryAdminRepo {
    admins: Arc<Mutex<HashMap<String, Admin>>>, // Using username as the key
    sessions: Arc<Mutex<HashMap<String, Session>>>, // Session ID to Session mapping
}

impl InMemoryAdminRepo {
    pub fn new() -> Self {
        InMemoryAdminRepo {
            admins: Arc::new(Mutex::new(HashMap::new())),
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_session(&mut self, username: &str) -> String {
        let mut admins = self.admins.lock().unwrap();

        if let Some(mut admin) = admins.get_mut(username) {
            let mut session = Session::new();
            session.insert_raw("user", username.to_string());
            let session_id = session.id().to_owned();

            admin.last_login_time = Some(Utc::now().naive_utc());
            self.save(admin.clone());

            let mut sessions = self.sessions.lock().unwrap();
            sessions.insert(session_id.clone(), session);
            session_id
        } else {
            panic!("Username not found");
        }
    }

    pub fn get_session(&self, session_id: &str) -> Option<Session> {
        let sessions = self.sessions.lock().unwrap();
        sessions.get(session_id).cloned()
    }

    pub fn destroy_session(&self, session_id: &str) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.remove(session_id);
    }
}

impl AdminRepo for InMemoryAdminRepo {
    fn init() -> Self {
        Self::new()
    }

    fn verify(&self, username: &str, password: &str) -> bool {
        let admins = self.admins.lock().unwrap();
        if let Some(admin) = admins.get(username) {
            admin.verify_password(password)
        } else {
            false
        }
    }

    fn find_by_username(&self, username: &str) -> Option<Admin> {
        let admins = self.admins.lock().unwrap();
        admins.get(username).cloned()
    }

    fn save(&mut self, admin: Admin) -> u32 {
        let mut admins = self.admins.lock().unwrap();
        admins.insert(admin.username.clone(), admin.clone());
        admin.id
    }
}
