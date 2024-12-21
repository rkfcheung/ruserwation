mod fake;

#[cfg(test)]
mod tests {
    use ruserwation::admin::logout::admin_logout_route;
    use std::sync::Arc;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::fake::fake_restaurant;
    use crate::fake::sessions::FakeSessionManager;

    #[tokio::test]
    async fn test_admin_logout_success() {
        // Mock session manager
        let mock_session_manager = FakeSessionManager::ok();
        let session_manager = Arc::new(mock_session_manager);

        // Create a dummy restaurant
        let restaurant = Arc::new(fake_restaurant());

        // Create the route
        let route = admin_logout_route(session_manager.clone(), restaurant.clone());

        // Simulate a GET request with a valid cookie
        assert!(&session_manager.has_session("valid_session_id"));
        let response = request()
            .method("GET")
            .path("/admin/logout")
            .header("Cookie", "session_id=valid_session_id")
            .reply(&route)
            .await;

        // Assertions
        assert_eq!(response.status(), StatusCode::OK);
        let body = String::from_utf8(response.body().to_vec()).unwrap();
        assert!(body.contains("Logged out successfully"));
        assert!(body.contains("Go to Homepage"));
        assert!(!&session_manager.has_session("valid_session_id"));

        // Check that the Set-Cookie header is set (indicating the session is deleted)
        let set_cookie_header = response.headers().get("Set-Cookie");
        assert!(set_cookie_header.is_some());

        // Ensure that the session_id cookie has been set to expire (in the past)
        if let Some(cookie_header) = set_cookie_header {
            let cookie_value = cookie_header.to_str().unwrap();
            // Assert that the cookie header contains the expiration date in the past
            assert!(cookie_value.contains("expires=Thu, 01 Jan 1970"));
        }
        session_manager.verify_result("destroy_session", 1);
        assert_eq!(
            session_manager.sessiond_id_captor.first(),
            Some("valid_session_id".to_string())
        );
    }

    #[tokio::test]
    async fn test_admin_logout_no_session() {
        // Mock session manager
        let mock_session_manager = FakeSessionManager::ok();
        let session_manager = Arc::new(mock_session_manager);

        // Create a dummy restaurant
        let restaurant = Arc::new(fake_restaurant());

        // Create the route
        let route = admin_logout_route(session_manager.clone(), restaurant.clone());

        // Simulate a GET request without a cookie
        let response = request()
            .method("GET")
            .path("/admin/logout")
            .reply(&route)
            .await;

        // Assertions
        assert_eq!(response.status(), StatusCode::OK);
        let body = String::from_utf8(response.body().to_vec()).unwrap();
        assert!(body.contains("No active session"));
        assert!(body.contains("Login as Admin"));

        // Check that no Set-Cookie header is set (since no session existed)
        let set_cookie_header = response.headers().get("Set-Cookie");
        assert!(set_cookie_header.is_none());
        session_manager.verify_result("destroy_session", 0);
        assert!(session_manager.sessiond_id_captor.last().is_none());
    }
}
