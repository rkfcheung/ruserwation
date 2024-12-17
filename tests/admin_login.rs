mod common;

#[cfg(test)]
mod tests {
    use ruserwation::admin::login::admin_login_route;
    use ruserwation::admin::models::Admin;
    use ruserwation::admin::repo::{AdminRepo, EnableSession};
    use serde_json::json as to_json;
    use std::sync::Arc;
    use warp::http::StatusCode;
    use warp::test::request;
    use warp_sessions::Session;

    type Error = String;
    struct MockAdminRepo {
        verify_result: bool,
        session_result: Option<Result<String, Error>>,
    }

    impl AdminRepo for MockAdminRepo {
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

    impl EnableSession<Error> for MockAdminRepo {
        async fn create_session(&self, _username: &str) -> Result<String, Error> {
            self.session_result
                .clone()
                .unwrap_or_else(|| Ok("mock_token".into()))
        }

        async fn destroy_session(&self, _session_id: &str) {
            unimplemented!()
        }

        async fn get_session(&self, _session_id: &str) -> Result<Session, Error> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn test_successful_login() {
        let repo = Arc::new(MockAdminRepo {
            verify_result: true,
            session_result: Some(Ok("mock_token".into())),
        });
        let filter = admin_login_route(repo.clone());

        let resp = request()
            .method("POST")
            .path("/admin/login")
            .json(&to_json!({"username": "admin", "password": "password"}))
            .reply(&filter)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(body["message"], "Login successful");
        assert_eq!(body["token"], "mock_token");
    }

    #[tokio::test]
    async fn test_invalid_credentials() {
        let repo = Arc::new(MockAdminRepo {
            verify_result: false,
            session_result: None,
        });
        let filter = admin_login_route(repo.clone());

        let resp = request()
            .method("POST")
            .path("/admin/login")
            .json(&to_json!({"username": "wrong", "password": "wrong"}))
            .reply(&filter)
            .await;

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(body["message"], "Invalid credentials");
    }

    #[tokio::test]
    async fn test_session_creation_failure() {
        let repo = Arc::new(MockAdminRepo {
            verify_result: true,
            session_result: Some(Err("Session creation failed".into())),
        });
        let filter = admin_login_route(repo.clone());

        let resp = request()
            .method("POST")
            .path("/admin/login")
            .json(&to_json!({"username": "admin", "password": "password"}))
            .reply(&filter)
            .await;

        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(body["message"], "Session creation failed");
    }

    #[tokio::test]
    async fn test_malformed_json_body() {
        let repo = Arc::new(MockAdminRepo {
            verify_result: true,
            session_result: Some(Ok("mock_token".into())),
        });
        let filter = admin_login_route(repo.clone());

        let resp = request()
            .method("POST")
            .path("/admin/login")
            .body("{ malformed_json: }")
            .reply(&filter)
            .await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_missing_fields() {
        let repo = Arc::new(MockAdminRepo {
            verify_result: true,
            session_result: Some(Ok("mock_token".into())),
        });
        let filter = admin_login_route(repo.clone());

        let resp = request()
            .method("POST")
            .path("/admin/login")
            .json(&to_json!({"username": "admin"})) // Missing "password" field
            .reply(&filter)
            .await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
