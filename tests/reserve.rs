mod mock;

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use mocks::MockVerify;
    use ruserwation::config::models::Context;
    use ruserwation::reservation::helper::generate_ref_check;
    use ruserwation::reservation::reserve::reserve_route;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::mock::mock_restaurant;
    use crate::mock::repos::MockContext;
    use crate::mock::repos::MockReservationRepo;

    // Mock context and repository for testing
    fn mock_context() -> Arc<Context<MockReservationRepo>> {
        // Create a mock or fake implementation of ReservationRepo
        // This is where you mock the `save` method
        let repo = MockReservationRepo::default().into();
        let restaurant = mock_restaurant().into();
        Context::create(repo, restaurant)
    }

    // Test valid reservation
    #[tokio::test]
    async fn test_reserve_valid_request() {
        // Create a valid reservation request
        let body = prepare_request();

        let context = prepare_response(&body).await;
        let repo = context.repo;
        let response = context.response;

        repo.verify_exactly_once("save");

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body["status"], "Success");
        assert_eq!(body["message"], "Booked successful");
        assert_eq!(body["book_ref"].as_str().unwrap().len(), 5);
    }

    // Test invalid reservation request (ref_check validation failed)
    #[tokio::test]
    async fn test_reserve_invalid_ref_check() {
        // Create a request with an invalid ref_check
        let body = json!({
            "ref_check": "invalid_check",
            "customer_email": "test@example.com",
            "customer_name": "John Doe",
            "customer_phone": "1234567890",
            "table_size": 4,
            "reservation_time": "2025-01-19T18:30:00",
            "notes": "Window seat request"
        });

        let context = prepare_response(&body).await;
        let repo = context.repo;
        let response = context.response;

        repo.verify_never("save");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body: Value = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body["status"], "Error");
        assert_eq!(
            body["message"],
            "The reservation request is either invalid or has expired."
        );
    }

    // Test reservation failure (e.g., database save failure)
    #[tokio::test]
    async fn test_reserve_save_failure() {
        // Mock a failing save by introducing an error in the repository
        let mut body = prepare_request();
        body["customer_email"] = "save_failure@example.com".into();

        let context = prepare_response(&body).await;
        let repo = context.repo;
        let response = context.response;

        repo.verify_exactly_once("save");

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body: Value = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body["status"], "Error");
        assert_eq!(body["message"], "Error: Failed to save reservation");
    }

    #[tokio::test]
    async fn test_reserve_validation_failure() {
        let mut body = prepare_request();
        body["customer_name"] = "".into();

        let context = prepare_response(&body).await;
        let repo = context.repo;
        let response = context.response;

        repo.verify_never("save");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body: Value = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body["message"], "Customer name cannot be empty.");
    }

    #[tokio::test]
    async fn test_reserve_time_in_past() {
        // Create a reservation request with a past reservation time
        let reservation_time = (Utc::now() - Duration::hours(1))
            .format("%Y-%m-%dT%H:%M:%S")
            .to_string();
        let mut body = prepare_request();
        body["reservation_time"] = reservation_time.into();

        let context = prepare_response(&body).await;
        let repo = context.repo;
        let response = context.response;

        repo.verify_never("save");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body: Value = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body["status"], "Error");
        assert_eq!(body["message"], "Reservation time cannot be in the past.");
    }

    #[tokio::test]
    async fn test_reserve_invalid_email() {
        // Create a reservation request with an invalid email
        let mut body = prepare_request();
        body["customer_email"] = "invalid_email".into();

        let context = prepare_response(&body).await;
        let repo = context.repo;
        let response = context.response;

        repo.verify_never("save");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body: Value = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body["status"], "Error");
        assert_eq!(body["message"], "Customer email is invalid.");
    }

    #[tokio::test]
    async fn test_reserve_invalid_table_size() {
        // Create a reservation request with an invalid table size
        let mut body = prepare_request();
        body["table_size"] = 0.into();

        let context = prepare_response(&body).await;
        let repo = context.repo;
        let response = context.response;

        repo.verify_never("save");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body: Value = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body["status"], "Error");
        assert_eq!(body["message"], "Table size must be between 1 and 20.");
    }

    #[tokio::test]
    async fn test_reserve_invalid_http_method() {
        let context = mock_context();
        let reserve_filter = reserve_route(context.clone());

        let repo = context.get();
        let response = request()
            .method("GET")
            .path("/reservations/reserve")
            .reply(&reserve_filter)
            .await;

        repo.verify_never("save");

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_reserve_empty_body() {
        let context = mock_context();
        let reserve_filter = reserve_route(context.clone());

        let repo = context.get();
        let response = request()
            .method("POST")
            .path("/reservations/reserve")
            .header("Content-Type", "application/json")
            .body("")
            .reply(&reserve_filter)
            .await;

        repo.verify_never("save");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            response.body(),
            "Request body deserialize error: EOF while parsing a value at line 1 column 0"
        );
    }

    #[tokio::test]
    async fn test_not_found_reservation() {
        let mut body = prepare_request();
        body["book_ref"] = "NOT_FOUND".into();

        let context = prepare_response(&body).await;
        let repo = context.repo;
        let response = context.response;

        repo.verify_never("save");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    fn prepare_request() -> Value {
        let reservation_time = (Utc::now() + Duration::hours(1))
            .format("%Y-%m-%dT%H:%M:%S")
            .to_string();
        json!({
            "ref_check": generate_ref_check("ChangeMe", Utc::now().naive_utc().and_utc().timestamp()).unwrap_or("invalid_check".to_string()),
            "customer_email": "test@example.com",
            "customer_name": "John Doe",
            "customer_phone": "1234567890",
            "table_size": 4,
            "reservation_time": reservation_time,
            "notes": "Window seat request"
        })
    }

    async fn prepare_response(body: &Value) -> MockContext<MockReservationRepo> {
        let context = mock_context();
        let reserve_filter = reserve_route(context.clone());

        let response = request()
            .method("POST")
            .path("/reservations/reserve")
            .header("Content-Type", "application/json")
            .json(body)
            .reply(&reserve_filter)
            .await;

        MockContext {
            repo: context.get(),
            response,
        }
    }
}
