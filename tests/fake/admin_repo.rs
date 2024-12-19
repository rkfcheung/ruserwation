use ruserwation::admin::{
    errors::SessionError,
    models::Admin,
    repo::{AdminRepo, VerifyUser},
    sessions::EnableSession,
};
use warp_sessions::Session;

pub(crate) struct FakeAdminRepo {
    pub(crate) verify_result: bool,
    pub(crate) session_result: Option<Result<String, SessionError>>,
}

impl AdminRepo for FakeAdminRepo {
    async fn find_by_id(&self, _id: u32) -> Option<Admin> {
        unimplemented!()
    }

    async fn find_by_username(&self, _username: &str) -> Option<Admin> {
        unimplemented!()
    }

    async fn save(&self, _admin: &mut ruserwation::admin::models::Admin) -> u32 {
        unimplemented!()
    }
}

impl EnableSession for FakeAdminRepo {
    async fn create_session(&self, _username: &str) -> Result<String, SessionError> {
        self.session_result.clone().unwrap()
    }

    async fn destroy_session(&self, _session_id: &str) {
        unimplemented!()
    }

    async fn get_session(&self, _session_id: &str) -> Result<Session, SessionError> {
        unimplemented!()
    }
}

impl VerifyUser for FakeAdminRepo {
    async fn contains(&self, _username: &str) -> bool {
        self.verify_result
    }

    async fn verify(&self, _username: &str, _password: &str) -> bool {
        self.verify_result
    }
}

impl FakeAdminRepo {
    pub(crate) fn ok() -> Self {
        Self {
            verify_result: true,
            session_result: Some(Ok("mock_session_id".to_string())),
        }
    }
}
