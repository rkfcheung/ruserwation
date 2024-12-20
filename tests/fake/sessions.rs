use ruserwation::admin::{errors::SessionError, repo::VerifyUser, sessions::EnableSession};

use warp_sessions::Session;

pub struct FakeSessionManager {
    pub(crate) verify_result: bool,
    pub(crate) session_result: Option<Result<String, SessionError>>,
}

impl VerifyUser for FakeSessionManager {
    async fn contains(&self, _username: &str) -> bool {
        self.verify_result
    }

    async fn verify(&self, _username: &str, _password: &str) -> bool {
        self.verify_result
    }
}

impl EnableSession for FakeSessionManager {
    async fn create_session(&self, _username: &str) -> Result<String, SessionError> {
        self.session_result.clone().unwrap()
    }

    async fn destroy_session(&self, _session_id: &str) {
        unimplemented!()
    }

    async fn get_session(&self, session_id: &str) -> Result<Session, SessionError> {
        if session_id == "valid_session_id" {
            let mut session = Session::new();
            let _ = session.insert("user", "admin".to_string());
            Ok(session)
        } else {
            Err(SessionError::SessionNotFound(session_id.to_string()))
        }
    }
}

impl FakeSessionManager {
    pub(crate) fn ok() -> Self {
        Self {
            verify_result: true,
            session_result: Some(Ok("mock_session_id".to_string())),
        }
    }
}
