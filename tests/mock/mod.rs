use ruserwation::{config::models::Context, restaurant::models::Restaurant};
use std::sync::Arc;

pub(crate) mod repos;
pub(crate) mod sessions;

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
