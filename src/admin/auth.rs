use std::{convert::Infallible, sync::Arc};
use warp::Filter;

use crate::config::models::Context;

pub fn get_cookie_session_id(cookie: Option<String>) -> Option<String> {
    cookie.and_then(|c| {
        c.split(';') // Split by semicolons to handle multiple key-value pairs
            .find_map(|pair| {
                let parts: Vec<_> = pair.trim().splitn(2, '=').collect(); // Split key-value pair by '='
                if parts.len() == 2 && parts[0].trim() == "session_id" {
                    Some(parts[1].to_string()) // Return the value if the key is "session_id"
                } else {
                    None
                }
            })
    })
}

// Helper function to pass `Context` into routes.
pub fn with_context<T: Send + Sync>(
    context: Arc<Context<T>>,
) -> impl Filter<Extract = (Arc<Context<T>>,), Error = Infallible> + Clone {
    warp::any().map(move || context.clone())
}
