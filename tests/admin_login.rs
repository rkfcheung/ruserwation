mod mock;

#[cfg(test)]
mod tests {
    use ruserwation::admin::errors::SessionError;
    use ruserwation::admin::login::{admin_login_form_route, admin_login_route};
    use serde_json::json as to_json;
    use std::sync::Arc;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::mock::sessions::MockSessionManager;
    use crate::mock::{mock_context, MockBody, MockRoute};

    #[tokio::test]
    async fn test_successful_login() {
        let session_manager = MockSessionManager::ok();
        let body = &to_json!({"username": "admin", "password": "password"});
        let route = simulate_request(session_manager, &body.into()).await;
        let response = route.response();

        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(body["message"], "Login successful");

        // Validate the Set-Cookie header
        let cookie_header = response.headers().get("set-cookie").unwrap();
        assert!(cookie_header
            .to_str()
            .unwrap()
            .contains("session_id=valid_session_id"));
        assert!(cookie_header.to_str().unwrap().contains("HttpOnly"));
    }

    #[tokio::test]
    async fn test_invalid_credentials() {
        let session_manager = MockSessionManager::default();
        let body = &to_json!({"username": "wrong", "password": "wrong"});
        let route = simulate_request(session_manager, &body.into()).await;
        let response = route.response();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(body["message"], "Invalid credentials");
        assert_eq!(
            response.headers().get("warning").unwrap(),
            StatusCode::UNAUTHORIZED.as_str()
        );

        // Validate there is no Set-Cookie header
        assert!(response.headers().get("set-cookie").is_none());
    }

    #[tokio::test]
    async fn test_session_creation_failure() {
        let session_manager = MockSessionManager::new(
            true,
            Some(Err(SessionError::SessionCreationFailed("mock".to_string()))),
        );
        let body = &to_json!({"username": "admin", "password": "password"});
        let route = simulate_request(session_manager, &body.into()).await;
        let response = route.response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(body["message"], "Failed to create session for 'mock'");
        assert_eq!(
            response.headers().get("warning").unwrap(),
            StatusCode::INTERNAL_SERVER_ERROR.as_str()
        );

        // Validate there is no Set-Cookie header
        assert!(response.headers().get("set-cookie").is_none());
    }

    #[tokio::test]
    async fn test_malformed_json_body() {
        let session_manager = MockSessionManager::ok();
        let body = "{ malformed_json: }";
        let route = simulate_request(session_manager, &body.into()).await;
        let response = route.response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_missing_fields() {
        let session_manager = MockSessionManager::ok();
        let body = &to_json!({"username": "admin"});
        let route = simulate_request(session_manager, &body.into()).await;
        let response = route.response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_admin_login_form_route() {
        // Mock the session manager
        let session_manager = MockSessionManager::ok();

        // Create the filter
        let context = mock_context(session_manager.into());
        let filter = admin_login_form_route(context);

        // Test case 1: No session cookie provided
        let response = request()
            .method("GET")
            .path("/admin/login")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), 200);
        let body = String::from_utf8(response.body().to_vec()).unwrap();
        let form_tag = "<form method=\"POST\" action=\"/admin/login\" enctype=\"application/x-www-form-urlencoded\">";
        assert!(body.contains("Admin Login"));
        assert!(body.contains(form_tag));

        // Test case 2: Valid session cookie provided
        let response = request()
            .method("GET")
            .path("/admin/login")
            .header("Cookie", "session_id=valid_session_id")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), 200);
        let body = String::from_utf8(response.body().to_vec()).unwrap();
        assert!(body.contains("Logged in already!"));
        assert!(body.contains("Welcome, admin."));

        // Test case 3: Invalid session cookie provided
        let response = request()
            .method("GET")
            .path("/admin/login")
            .header("Cookie", "session_id=invalid_session_id")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), 200);
        let body = String::from_utf8(response.body().to_vec()).unwrap();
        assert!(body.contains("Admin Login"));
        assert!(body.contains(form_tag));
    }

    #[tokio::test]
    async fn test_form_body_login() {
        // Create a mock session manager
        let session_manager = MockSessionManager::ok();

        // Prepare the form data
        let form_data = "username=test_user&password=test_pass";

        // Perform the request
        let route = simulate_request_with_header(
            session_manager.into(),
            "POST",
            "Content-Type",
            "application/x-www-form-urlencoded",
            &form_data.into(),
        )
        .await;
        let response = route.response();

        // Assert the response
        assert_eq!(response.status(), 200); // Replace with expected status code
        let body = String::from_utf8(response.body().to_vec()).unwrap();
        assert_eq!(
            body,
            "{\"message\":\"Login successful\",\"status\":\"Success\"}"
        );
    }

    async fn simulate_request(
        session_manager: MockSessionManager,
        body: &MockBody<'_>,
    ) -> MockRoute<MockSessionManager> {
        MockRoute::simulate_request(
            session_manager.into(),
            admin_login_route,
            "POST",
            "/admin/login",
            body,
        )
        .await
    }

    async fn simulate_request_with_header(
        session_manager: Arc<MockSessionManager>,
        method: &str,
        header_key: &str,
        header_value: &str,
        body: &MockBody<'_>,
    ) -> MockRoute<MockSessionManager> {
        MockRoute::simulate_request_with_header(
            session_manager,
            admin_login_route,
            method,
            "/admin/login",
            header_key,
            header_value,
            body,
        )
        .await
    }
}
