mod mock;

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use ruserwation::config::models::Context;
    use ruserwation::reservation::helper::generate_ref_check;
    use ruserwation::reservation::repo::ReservationRepo;
    use ruserwation::reservation::reserve::reserve_route;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::mock::mock_restaurant;
    use crate::mock::repos::MockReservationRepo;

    // Mock context and repository for testing
    fn mock_context() -> Arc<Context<impl ReservationRepo + Send + Sync>> {
        // Create a mock or fake implementation of ReservationRepo
        // This is where you mock the `save` method
        let repo = MockReservationRepo::default().into();
        let restaurant = mock_restaurant().into();
        Context::create(repo, restaurant)
    }

    // Test valid reservation
    #[tokio::test]
    async fn test_reserve_valid_request() {
        let context = mock_context();
        let reserve_filter = reserve_route(context);

        // Create a valid reservation request
        let body = prepare_request("test@example.com");

        let response = request()
            .method("POST")
            .path("/reservations/reserve")
            .header("Content-Type", "application/json")
            .json(&body)
            .reply(&reserve_filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body["status"], "Success");
        assert_eq!(body["message"], "Booked successful");
        assert_eq!(body["book_ref"].as_str().unwrap().len(), 5);
    }

    // Test invalid reservation request (ref_check validation failed)
    #[tokio::test]
    async fn test_reserve_invalid_ref_check() {
        let context = mock_context();
        let reserve_filter = reserve_route(context);

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

        let response = request()
            .method("POST")
            .path("/reservations/reserve")
            .header("Content-Type", "application/json")
            .json(&body)
            .reply(&reserve_filter)
            .await;

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
        let context = mock_context();
        let reserve_filter = reserve_route(context);

        // Mock a failing save by introducing an error in the repository
        let body = prepare_request("save_failure@example.com");

        let response = request()
            .method("POST")
            .path("/reservations/reserve")
            .header("Content-Type", "application/json")
            .json(&body)
            .reply(&reserve_filter)
            .await;

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body: Value = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body["status"], "Error");
        assert_eq!(body["message"], "Error: Failed to save reservation");
    }

    #[tokio::test]
    async fn test_reserve_validation_failure() {
        let context = mock_context();
        let filter = reserve_route(context);

        let request_body = prepare_request_with_name("test@example.com", "");

        let response = warp::test::request()
            .method("POST")
            .path("/reservations/reserve")
            .header("Content-Type", "application/json")
            .body(request_body.to_string())
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body: Value = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body["message"], "Customer name cannot be empty.");
    }

    fn prepare_request(customer_email: &str) -> Value {
        prepare_request_with_name(customer_email, "John Doe")
    }

    fn prepare_request_with_name(customer_email: &str, customer_name: &str) -> Value {
        let reservation_time = (Utc::now() + Duration::hours(1))
            .format("%Y-%m-%dT%H:%M:%S")
            .to_string();
        json!({
            "ref_check": generate_ref_check("ChangeMe", Utc::now().naive_utc().and_utc().timestamp()).unwrap_or("invalid_check".to_string()),
            "customer_email": customer_email,
            "customer_name": customer_name,
            "customer_phone": "1234567890",
            "table_size": 4,
            "reservation_time": reservation_time,
            "notes": "Window seat request"
        })
    }
}
