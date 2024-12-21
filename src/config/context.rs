use std::{convert::Infallible, sync::Arc};
use warp::Filter;

use super::models::Context;

// Helper function to pass `Context` into routes.
pub fn with_context<T: Send + Sync>(
    context: Arc<Context<T>>,
) -> impl Filter<Extract = (Arc<Context<T>>,), Error = Infallible> + Clone {
    warp::any().map(move || context.clone())
}
