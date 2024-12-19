mod fake;

#[cfg(test)]
mod tests {
    use ruserwation::admin::errors::SessionError;
    use ruserwation::admin::login::admin_login_route;
    use ruserwation::admin::sessions::SessionManager;
    use serde_json::json as to_json;
    use std::sync::Arc;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::fake::admin_repo::FakeAdminRepo;

    #[tokio::test]
    async fn test_successful_login() {
        let admin_repo = Arc::new(FakeAdminRepo::ok());
        let session_manager = Arc::new(SessionManager::new(admin_repo));
        let filter = admin_login_route(session_manager);

        let resp = request()
            .method("POST")
            .path("/admin/login")
            .json(&to_json!({"username": "admin", "password": "password"}))
            .reply(&filter)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(body["message"], "Login successful");

        // Validate the Set-Cookie header
        let cookie_header = resp.headers().get("set-cookie").unwrap();
        assert!(cookie_header
            .to_str()
            .unwrap()
            .contains("session_id=mock_session_id"));
        assert!(cookie_header.to_str().unwrap().contains("HttpOnly"));
    }

    #[tokio::test]
    async fn test_invalid_credentials() {
        let admin_repo = Arc::new(FakeAdminRepo {
            verify_result: false,
            session_result: None,
        });
        let session_manager = Arc::new(SessionManager::new(admin_repo));
        let filter = admin_login_route(session_manager);

        let resp = request()
            .method("POST")
            .path("/admin/login")
            .json(&to_json!({"username": "wrong", "password": "wrong"}))
            .reply(&filter)
            .await;

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(body["message"], "Invalid credentials");
        assert_eq!(
            resp.headers().get("warning").unwrap(),
            StatusCode::UNAUTHORIZED.as_str()
        );

        // Validate there is no Set-Cookie header
        assert!(resp.headers().get("set-cookie").is_none());
    }

    #[tokio::test]
    async fn test_session_creation_failure() {
        let admin_repo = Arc::new(FakeAdminRepo {
            verify_result: true,
            session_result: Some(Err(SessionError::SessionCreationFailed("mock".to_string()))),
        });
        let session_manager = Arc::new(SessionManager::new(admin_repo));
        let filter = admin_login_route(session_manager);

        let resp = request()
            .method("POST")
            .path("/admin/login")
            .json(&to_json!({"username": "admin", "password": "password"}))
            .reply(&filter)
            .await;

        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(body["message"], "Failed to create session for 'mock'");
        assert_eq!(
            resp.headers().get("warning").unwrap(),
            StatusCode::INTERNAL_SERVER_ERROR.as_str()
        );

        // Validate there is no Set-Cookie header
        assert!(resp.headers().get("set-cookie").is_none());
    }

    #[tokio::test]
    async fn test_malformed_json_body() {
        let admin_repo = Arc::new(FakeAdminRepo::ok());
        let session_manager = Arc::new(SessionManager::new(admin_repo));
        let filter = admin_login_route(session_manager);

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
        let admin_repo = Arc::new(FakeAdminRepo::ok());
        let session_manager = Arc::new(SessionManager::new(admin_repo));
        let filter = admin_login_route(session_manager);

        let resp = request()
            .method("POST")
            .path("/admin/login")
            .json(&to_json!({"username": "admin"})) // Missing "password" field
            .reply(&filter)
            .await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
