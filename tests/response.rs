#[cfg(test)]
mod tests {
    use ruserwation::response::handle_rejections;
    use serde_json::Value;
    use warp::http::StatusCode;
    use warp::hyper::body::to_bytes;
    use warp::reject::{self, Reject};
    use warp::reply::Reply;

    #[derive(Debug)]
    struct UnknownError;

    impl Reject for UnknownError {}

    #[tokio::test]
    async fn test_handle_rejections_not_found() {
        // Simulate a not found rejection
        let rejection = reject::not_found();
        let response = handle_rejections(rejection).await.unwrap().into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body_bytes = to_bytes(response).await.unwrap();
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(
            body["message"],
            StatusCode::NOT_FOUND.canonical_reason().unwrap()
        );
        assert_eq!(body["status"], "Error");
    }

    #[tokio::test]
    async fn test_handle_rejections_fallback_internal_error() {
        // Simulate an unknown rejection
        let rejection = reject::custom(UnknownError {});
        let response = handle_rejections(rejection).await.unwrap().into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let body_bytes = to_bytes(response).await.unwrap();
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(
            body["message"],
            StatusCode::INTERNAL_SERVER_ERROR
                .canonical_reason()
                .unwrap()
        );
        assert_eq!(body["status"], "Error");
    }
}
