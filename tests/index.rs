#[cfg(test)]
mod tests {
    use ruserwation::restaurant::index::index_route;
    use ruserwation::restaurant::models::Restaurant;
    use warp::test::request;

    #[tokio::test]
    async fn test_index_route() {
        let restaurant = Restaurant {
            id: 1,
            name: "Test Restaurant".to_string(),
            max_capacity: 100,
            location: "Test Location".to_string(),
            active: true,
        };

        let index_filter = index_route(restaurant);

        let res = request().method("GET").path("/").reply(&index_filter).await;
        assert_eq!(res.status(), 200);

        let body_str = std::str::from_utf8(res.body()).expect("Response body is not valid UTF-8");
        assert!(body_str.contains("Test Restaurant"));
    }
}
