mod common;

#[cfg(test)]
mod integration_tests {
    use ruserwation::admin::login::admin_login_route;
    use ruserwation::config::models::Context;
    use ruserwation::response::handle_rejections;
    use serde_json::{json as to_json, Value};
    use warp::http::StatusCode;
    use warp::test::request;
    use warp::Filter;

    use crate::common::db_utils::init_test_app_state;

    #[tokio::test]
    async fn test_admin_login_success() {
        // Initialise application state for testing
        let app_state = init_test_app_state().await;

        // Build the login route
        let context = Context::create(app_state.session_manager(), app_state.restaurant());
        let filter = admin_login_route(context);

        // Perform a POST request to the /admin/login endpoint with valid credentials
        let response = request()
            .method("POST")
            .path("/admin/login")
            .json(&to_json!({
                "username": "admin", // Preloaded username from init_test_db()
                "password": "localtest", // Preloaded password from init_test_db()
            }))
            .reply(&filter)
            .await;

        // Assert the status is 200 OK
        assert_eq!(response.status(), StatusCode::OK);

        // Assert the response body
        let body: Value = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(body["message"], "Login successful");

        // Assert the Set-Cookie header exists
        let cookie_header = response.headers().get("set-cookie").unwrap();
        assert!(cookie_header.to_str().unwrap().contains("session_id="));
    }

    #[tokio::test]
    async fn test_admin_login_invalid_credentials() {
        // Initialise application state for testing
        let app_state = init_test_app_state().await;

        // Build the login route
        let context = Context::create(app_state.session_manager(), app_state.restaurant());
        let filter = admin_login_route(context);

        // Perform a POST request to the /admin/login endpoint with invalid credentials
        let response = request()
            .method("POST")
            .path("/admin/login")
            .json(&to_json!({
                "username": "admin",
                "password": "wrongpassword",
            }))
            .reply(&filter)
            .await;

        // Assert the status is 401 Unauthorised
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        // Assert the response body
        let body: Value = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(body["message"], "Invalid credentials");

        // Assert no Set-Cookie header exists
        assert!(response.headers().get("set-cookie").is_none());
    }

    #[tokio::test]
    async fn test_admin_login_missing_fields() {
        // Initialise application state for testing
        let app_state = init_test_app_state().await;

        // Build the login route
        let context = Context::create(app_state.session_manager(), app_state.restaurant());
        let filter = admin_login_route(context).recover(handle_rejections);

        // Perform a POST request with missing fields
        let response = request()
            .method("POST")
            .path("/admin/login")
            .json(&to_json!({
                "username": "admin"
                // Missing "password" field
            }))
            .reply(&filter)
            .await;

        // Assert the status is 400 Bad Request
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // Assert the response body (if your application provides error details)
        let body: Value = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(
            body["message"],
            "Request body deserialize error: missing field `password` at line 1 column 20"
        );
    }
}
