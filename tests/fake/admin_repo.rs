use ruserwation::admin::{
    errors::SessionError, models::Admin, repo::AdminRepo, sessions::EnableSession,
};
use warp_sessions::Session;

pub(crate) struct FakeAdminRepo {
    pub(crate) verify_result: bool,
    pub(crate) session_result: Option<Result<String, SessionError>>,
}

impl AdminRepo for FakeAdminRepo {
    async fn verify(&self, _username: &str, _password: &str) -> bool {
        self.verify_result
    }

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
    type Error = SessionError;

    async fn create_session(&self, _username: &str) -> Result<String, Self::Error> {
        self.session_result
            .clone()
            .unwrap_or_else(|| Ok("mock_session_id".into()))
    }

    async fn destroy_session(&self, _session_id: &str) {
        unimplemented!()
    }

    async fn get_session(&self, _session_id: &str) -> Result<Session, Self::Error> {
        unimplemented!()
    }
}
