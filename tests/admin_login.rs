mod fake;

#[cfg(test)]
mod tests {
    use ruserwation::admin::errors::SessionError;
    use ruserwation::admin::login::{admin_login_form_route, admin_login_route};
    use ruserwation::restaurant::models::Restaurant;
    use serde_json::json as to_json;
    use std::sync::Arc;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::fake::sessions::FakeSessionManager;

    #[tokio::test]
    async fn test_successful_login() {
        let session_manager = FakeSessionManager::ok();
        let filter = admin_login_route(session_manager.into());

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
        let session_manager = FakeSessionManager {
            verify_result: false,
            session_result: None,
        };
        let filter = admin_login_route(session_manager.into());

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
        let session_manager = FakeSessionManager {
            verify_result: true,
            session_result: Some(Err(SessionError::SessionCreationFailed("mock".to_string()))),
        };
        let filter = admin_login_route(session_manager.into());

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
        let session_manager = FakeSessionManager::ok();
        let filter = admin_login_route(session_manager.into());

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
        let session_manager = FakeSessionManager::ok();
        let filter = admin_login_route(session_manager.into());

        let resp = request()
            .method("POST")
            .path("/admin/login")
            .json(&to_json!({"username": "admin"})) // Missing "password" field
            .reply(&filter)
            .await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_admin_login_form_route() {
        // Mock the session manager
        let session_manager = Arc::new(FakeSessionManager::ok());

        // Mock the restaurant
        let restaurant = Arc::new(Restaurant {
            id: 1,
            name: "Test Restaurant".to_string(),
            max_capacity: 32,
            location: "Test City".to_string(),
            active: true,
        });

        // Create the filter
        let filter = admin_login_form_route(session_manager.clone(), restaurant.clone());

        // Test case 1: No session cookie provided
        let res = request()
            .method("GET")
            .path("/admin/login")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), 200);
        let body = String::from_utf8(res.body().to_vec()).unwrap();
        assert!(body.contains("Admin Login"));
        assert!(body.contains("<form method=\"POST\" action=\"/admin/login\">"));

        // Test case 2: Valid session cookie provided
        let res = request()
            .method("GET")
            .path("/admin/login")
            .header("Cookie", "session_id=valid_session_id")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), 200);
        let body = String::from_utf8(res.body().to_vec()).unwrap();
        assert!(body.contains("Logged in already!"));
        assert!(body.contains("Welcome, admin."));

        // Test case 3: Invalid session cookie provided
        let res = request()
            .method("GET")
            .path("/admin/login")
            .header("Cookie", "session_id=invalid_session_id")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), 200);
        let body = String::from_utf8(res.body().to_vec()).unwrap();
        assert!(body.contains("Admin Login"));
        assert!(body.contains("<form method=\"POST\" action=\"/admin/login\">"));
    }
}
