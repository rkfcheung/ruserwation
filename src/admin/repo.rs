use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp_sessions::Session;

use super::models::Admin;

pub trait AdminRepo {
    fn new() -> Self; // Initialise the repository
    fn find_by_username(&self, username: &str) -> Option<Admin>; // Find an Admin by username
    fn save(&mut self, admin: Admin) -> u32; // Save an Admin and return its ID
    fn verify(&self, username: &str, password: &str) -> bool; // Verify username and password
}

pub trait EnableSession {
    fn create_session(&self, username: &str) -> String;

    fn destroy_session(&self, session_id: &str);

    fn get_session(&self, session_id: &str) -> Option<Session>;
}

pub struct Sessions {
    context: Arc<Mutex<HashMap<String, Session>>>, // Session ID to Session mapping
}

impl Sessions {
    pub fn new() -> Self {
        Sessions {
            context: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create(&self, username: &str) -> String {
        // Create a new session
        let mut session = Session::new();
        session.insert_raw("user", username.to_string());

        let session_id = session.id().to_owned();

        // Lock the sessions HashMap and insert the new session
        let mut sessions = self.context.lock().unwrap();
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
        InMemoryAdminRepo {
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
    fn new() -> Self {
        Self::new()
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

    fn verify(&self, username: &str, password: &str) -> bool {
        let admins = self.admins.lock().unwrap();
        if let Some(admin) = admins.get(username) {
            admin.verify_password(password)
        } else {
            false
        }
    }
}
