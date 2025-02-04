mod mock;

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use mocks::MockVerify;
    use ruserwation::reservation::helper::generate_ref_check;
    use ruserwation::reservation::models::Reservation;
    use ruserwation::reservation::reserve::reserve_route;
    use serde_json::{json, Value};
    use warp::http::StatusCode;

    use crate::mock::repos::MockReservationRepo;
    use crate::mock::MockBody;
    use crate::mock::MockRoute;

    // Test valid reservation
    #[tokio::test]
    async fn test_reserve_valid_request() {
        // Create a valid reservation request
        let body = prepare_request();

        let route = post_request(&body).await;
        let repo = route.context();
        let response = route.response();

        repo.verify_exactly_once("save");

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body["status"], "Success");
        assert_eq!(body["message"], "Booked successfully");
        assert_eq!(body["book_ref"].as_str().unwrap().len(), 5);
    }

    // Test invalid reservation request (ref_check validation failed)
    #[tokio::test]
    async fn test_reserve_invalid_ref_check() {
        // Create a request with an invalid ref_check
        let body = json!({
            "ref_check": "invalid_check",
            "customer": json!({
                "email": "test@example.com",
                "name": "John Doe",
                "phone": "1234567890"
            }),
            "table_size": 4,
            "reservation_time": "2025-01-19T18:30:00",
            "notes": "Window seat request"
        });

        let route = post_request(&body).await;
        let repo = route.context();
        let response = route.response();

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
        body["customer"]["email"] = "save_failure@example.com".into();

        let route = post_request(&body).await;
        let repo = route.context();
        let response = route.response();

        repo.verify_exactly_once("save");

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body: Value = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body["status"], "Error");
        assert_eq!(body["message"], "Error: Failed to save reservation");
    }

    #[tokio::test]
    async fn test_reserve_validation_failure() {
        let mut body = prepare_request();
        body["customer"]["name"] = "".into();

        let route = post_request(&body).await;
        let repo = route.context();
        let response = route.response();

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

        let route = post_request(&body).await;
        let repo = route.context();
        let response = route.response();

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
        body["customer"]["email"] = "invalid_email".into();

        let route = post_request(&body).await;
        let repo = route.context();
        let response = route.response();

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

        let route = post_request(&body).await;
        let repo = route.context();
        let response = route.response();

        repo.verify_never("save");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body: Value = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body["status"], "Error");
        assert_eq!(body["message"], "Table size must be between 1 and 20.");
    }

    #[tokio::test]
    async fn test_reserve_invalid_http_method() {
        let route = simulate_request("GET", &MockBody::None).await;
        let repo = route.context();
        let response = route.response();

        repo.verify_never("save");

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(response.body(), "HTTP method not allowed");
    }

    #[tokio::test]
    async fn test_reserve_empty_body() {
        let route = simulate_request("POST", &MockBody::Text("")).await;
        let repo = route.context();
        let response = route.response();

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

        let route = post_request(&body).await;
        let repo = route.context();
        let response = route.response();

        repo.verify_never("save");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_update_reservation_invalid_book_ref() {
        let mut body = prepare_request();
        body["book_ref"] = "valid_book_ref".into();

        let route = simulate_update("invalid_book_ref", &(&body).into()).await;
        let repo = route.context();
        let response = route.response();

        repo.verify_never("save");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body: Value = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body["status"], "Error");
        assert_eq!(body["book_ref"], "invalid_book_ref");
        assert_eq!(body["message"], "The reservation request is invalid.");
    }

    #[tokio::test]
    async fn test_update_reservation_valid() {
        let mut body = prepare_request();
        body["book_ref"] = "valid_book_ref".into();
        body["table_size"] = 5.into();

        let route = simulate_update("valid_book_ref", &(&body).into()).await;
        let repo = route.context();

        repo.verify_exactly_once("find_by_book_ref");
        repo.verify_exactly_once("save");
        assert_eq!(
            repo.invocation
                .first("find_by_book_ref")
                .unwrap()
                .get_unchecked::<String>(),
            "valid_book_ref"
        );
        let saved = repo.invocation.last("save").unwrap();
        let saved = saved.get_unchecked::<Reservation>();
        assert_eq!(saved.table_size, 5);

        let response = route.response();

        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body["status"], "Success");
        assert_eq!(body["book_ref"], "valid_book_ref");
        assert_eq!(body["message"], "Updated successfully");
    }

    #[tokio::test]
    async fn test_update_reservation_not_found() {
        let book_ref = "non_existent_ref";
        let mut body = prepare_request();
        body["book_ref"] = book_ref.into();

        let route = simulate_update(book_ref, &(&body).into()).await;
        let repo = route.context();

        assert_eq!(
            repo.invocation
                .first("find_by_book_ref")
                .unwrap()
                .get_unchecked::<String>(),
            book_ref
        );

        let response = route.response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body: Value = serde_json::from_slice(&response.body()).unwrap();
        assert_eq!(body["status"], "Error");
        assert_eq!(body["book_ref"], book_ref);
        assert_eq!(body["message"], "Not found for the query: Invalid Book Ref");
    }

    fn prepare_request() -> Value {
        let reservation_time = (Utc::now() + Duration::hours(1))
            .format("%Y-%m-%dT%H:%M:%S")
            .to_string();
        json!({
            "ref_check": generate_ref_check("ChangeMe", Utc::now().naive_utc().and_utc().timestamp()).unwrap_or("invalid_check".to_string()),
            "customer": json!({
                "email": "test@example.com",
                "name": "John Doe",
                "phone": "1234567890"
            }),
            "table_size": 4,
            "reservation_time": reservation_time,
            "notes": "Window seat request"
        })
    }

    async fn post_request(body: &Value) -> MockRoute<MockReservationRepo> {
        simulate_request("POST", &body.into()).await
    }

    async fn simulate_request(method: &str, body: &MockBody<'_>) -> MockRoute<MockReservationRepo> {
        MockRoute::simulate_request(
            MockReservationRepo::default().into(),
            reserve_route,
            method,
            "/reservations/reserve",
            body,
        )
        .await
    }

    async fn simulate_update(
        book_ref: &str,
        body: &MockBody<'_>,
    ) -> MockRoute<MockReservationRepo> {
        MockRoute::simulate_request(
            MockReservationRepo::default().into(),
            reserve_route,
            "PUT",
            &format!("/reservations/update/{book_ref}"),
            body,
        )
        .await
    }
}
