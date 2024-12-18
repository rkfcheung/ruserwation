use serde_json::json as to_json;
use std::sync::Arc;
use warp::{
    http::StatusCode,
    reply::{json, with_status},
    Filter, Rejection, Reply,
};

use super::{
    errors::SessionError,
    models::{LoginRequest, LoginResponse},
    repo::AdminRepo,
    sessions::{EnableSession, SessionManager},
};

// Define the route for login
pub fn admin_login_route<R>(
    session_manager: Arc<SessionManager<R>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: AdminRepo + EnableSession<Error = SessionError> + Send + Sync,
{
    warp::post()
        .and(warp::path!("admin" / "login"))
        .and(warp::body::json())
        .and(with_session_manager(session_manager))
        .and_then(handle_admin_login)
}

// The handler for the admin login
async fn handle_admin_login<R>(
    body: LoginRequest,
    session_manager: Arc<SessionManager<R>>,
) -> Result<impl Reply, Rejection>
where
    R: AdminRepo + EnableSession<Error = SessionError> + Send + Sync,
{
    // If credentials match, return a success response
    if session_manager.verify(&body.username, &body.password).await {
        match session_manager.create_session(&body.username).await {
            Ok(token) => Ok(with_status(
                json(&to_json!(LoginResponse::ok(&token))),
                StatusCode::OK,
            )),
            Err(err) => Ok(with_status(
                json(&to_json!(LoginResponse::err(&err.to_string()))),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    } else {
        // If credentials don't match, return an error response
        Ok(with_status(
            json(&to_json!(LoginResponse::err("Invalid credentials"))),
            StatusCode::UNAUTHORIZED,
        ))
    }
}

// Helper function to attach admin_repo with correct lifetime
fn with_session_manager<R>(
    session_manager: Arc<SessionManager<R>>,
) -> impl Filter<Extract = (Arc<SessionManager<R>>,), Error = std::convert::Infallible> + Clone
where
    R: AdminRepo + EnableSession<Error = SessionError> + Send + Sync,
{
    warp::any().map(move || session_manager.clone())
}
