use mocks::{ArgumentCaptor, InvocationTracker};
use ruserwation::admin::{errors::SessionError, repo::VerifyUser, sessions::EnableSession};
use std::{collections::HashSet, sync::Mutex};
use test_utils::{mock_invoked, MockVerify};
use warp_sessions::Session;

#[derive(Default, MockVerify)]
pub struct FakeSessionManager {
    pub(crate) verify_result: bool,
    pub(crate) session_result: Option<Result<String, SessionError>>,
    pub(crate) sessions: Mutex<HashSet<String>>,
    pub(crate) invocation: InvocationTracker,
    pub(crate) sessiond_id_captor: ArgumentCaptor<String>,
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

    #[mock_invoked]
    async fn destroy_session(&self, session_id: &str) {
        self.sessiond_id_captor.capture(session_id.to_string());
        let mut sessions = self.sessions.lock().unwrap();
        sessions.remove(session_id);
    }

    async fn get_session(&self, session_id: &str) -> Result<Session, SessionError> {
        if self.has_session(session_id) {
            let mut session = Session::new();
            let _ = session.insert("user", "admin".to_string());
            Ok(session)
        } else {
            Err(SessionError::SessionNotFound(session_id.to_string()))
        }
    }
}

impl FakeSessionManager {
    pub(crate) fn new(
        verify_result: bool,
        session_result: Option<Result<String, SessionError>>,
    ) -> Self {
        Self {
            verify_result,
            session_result,
            sessions: Mutex::default(),
            invocation: InvocationTracker::default(),
            sessiond_id_captor: ArgumentCaptor::default(),
        }
    }

    pub(crate) fn ok() -> Self {
        let session_id = "valid_session_id";
        let mut sessions = HashSet::new();
        sessions.insert(session_id.to_string());

        Self {
            verify_result: true,
            session_result: Some(Ok(session_id.to_string())),
            sessions: Mutex::new(sessions),
            invocation: InvocationTracker::default(),
            sessiond_id_captor: ArgumentCaptor::default(),
        }
    }

    pub(crate) fn has_session(&self, session_id: &str) -> bool {
        self.sessions.lock().unwrap().contains(session_id)
    }
}
