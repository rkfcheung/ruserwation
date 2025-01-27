use ruserwation::{config::models::Context, restaurant::models::Restaurant};
use serde_json::Value;
use std::sync::Arc;
use warp::http::Response;
use warp::hyper::body::Bytes;
use warp::reply::Reply;
use warp::test::request;
use warp::Filter;

pub(crate) mod repos;
pub(crate) mod sessions;

pub(crate) enum MockBody<'a> {
    Json(&'a Value),
    Text(&'a str),
}

pub(crate) struct MockRoute<T> {
    context: Arc<Context<T>>,
    response: Response<Bytes>,
}

impl<'a> From<&'a str> for MockBody<'a> {
    fn from(value: &'a str) -> Self {
        MockBody::Text(value)
    }
}

impl<'a> From<&'a Value> for MockBody<'a> {
    fn from(value: &'a Value) -> Self {
        MockBody::Json(value)
    }
}

#[cfg(test)]
#[allow(dead_code)]
impl<T> MockRoute<T> {
    pub(crate) fn context(&self) -> Arc<T> {
        self.context.get()
    }

    pub(crate) fn response(&self) -> &Response<Bytes> {
        &self.response
    }

    pub(crate) async fn simulate_request<F>(
        context: Arc<T>,
        route: impl FnOnce(Arc<Context<T>>) -> F,
        method: &str,
        path: &str,
        body: &MockBody<'_>,
    ) -> Self
    where
        F: Filter + 'static,
        F::Extract: Reply + Send,
    {
        // Create the mock context
        let mock_context = mock_context(context.clone());

        // Call the route function with the context
        let filter = route(mock_context.clone());

        // Simulate the HTTP request
        let builder = request()
            .method(method)
            .path(path)
            .header("Content-Type", "application/json");
        let response = match body {
            MockBody::Json(json) => builder.json(json),
            MockBody::Text(text) => builder.body(text),
        }
        .reply(&filter)
        .await;

        Self {
            context: mock_context,
            response,
        }
    }
}

// Mock context for testing
pub(crate) fn mock_context<T>(context: Arc<T>) -> Arc<Context<T>> {
    Context::create(context, mock_restaurant().into())
}

pub(crate) fn mock_restaurant() -> Restaurant {
    Restaurant {
        id: 1,
        name: "Test Restaurant".to_string(),
        max_capacity: 32,
        location: "Test City".to_string(),
        active: true,
    }
}
