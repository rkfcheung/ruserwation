use maud::html;
use std::{convert::Infallible, sync::Arc};
use warp::{http::HeaderValue, http::Response, Filter, Rejection, Reply};

use super::helper::get_cookie_session_id;
use crate::{
    admin::sessions::EnableSession, restaurant::models::Restaurant, utils::html_util::render_html,
};

/// Defines the route for admin logout.
pub fn admin_logout_route(
    session_manager: Arc<impl EnableSession + Send + Sync>,
    restaurant: Arc<Restaurant>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("admin" / "logout")
        .and(warp::get())
        .and(warp::header::optional::<String>("Cookie"))
        .and(with_session_manager(session_manager))
        .and_then(move |cookie: Option<String>, session_manager| {
            let restaurant = restaurant.clone();
            async move { handle_admin_logout(cookie, session_manager, restaurant.clone()).await }
        })
}

// Handles the admin logout logic.
async fn handle_admin_logout(
    cookie: Option<String>,
    session_manager: Arc<impl EnableSession + Send + Sync>,
    restaurant: Arc<Restaurant>,
) -> Result<impl Reply, Rejection> {
    // Extract the session ID from the cookie (e.g., "session_id=...")
    let mut destroyed = false;
    let content = if let Some(session_id) = get_cookie_session_id(cookie) {
        // Attempt to destroy the session
        session_manager.destroy_session(&session_id).await;
        destroyed = true;

        // HTML content for successful logout
        html! {
            div class="container mt-5" {
                div class="alert alert-success" role="alert" {
                    h4 class="alert-heading" { "Logged out successfully" }
                    p { "You have been logged out successfully. Thank you for visiting!" }
                    hr;
                    a href="/" class="btn btn-primary" { "Go to Homepage" }
                }
            }
        }
    } else {
        // HTML content for no active session
        html! {
            div class="container mt-5" {
                div class="alert alert-warning" role="alert" {
                    h4 class="alert-heading" { "No active session" }
                    p { "You were not logged in or your session has already expired." }
                    hr;
                    a href="/admin/login" class="btn btn-primary" { "Login as Admin" }
                }
            }
        }
    };

    // Render the HTML with the restaurant layout
    let mut response = Response::new(render_html(&restaurant, content).into_string());

    // Set the cookie with the same name and expire it in the past
    if destroyed {
        if let Ok(hdr_val) = HeaderValue::from_str(
            "session_id=; expires=Thu, 01 Jan 1970 00:00:00 GMT; path=/; HttpOnly",
        ) {
            response.headers_mut().insert("Set-Cookie", hdr_val);
        }
    }

    // Return the response with the "Set-Cookie" header to delete the session cookie
    Ok(response)
}

// Helper function to pass `SessionManager` into routes.
fn with_session_manager(
    session_manager: Arc<impl EnableSession + Send + Sync>,
) -> impl Filter<Extract = (Arc<impl EnableSession + Send + Sync>,), Error = Infallible> + Clone {
    warp::any().map(move || session_manager.clone())
}
