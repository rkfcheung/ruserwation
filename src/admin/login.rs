use serde_json::json as to_json;
use std::sync::Arc;
use warp::{
    filters::{cookie, reply::headers},
    http::{header, StatusCode},
    reply::{json, with_header, with_status},
    Filter, Rejection, Reply,
};

use crate::utils::env_util::{is_prod, var_as_str};

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
            Ok(session_id) => {
                let response = json(&to_json!(LoginResponse::ok()));
                let secured = if is_prod() { " Secure;" } else { "" };
                let cookie = format!("session_id={}; HttpOnly;{} Path=/", session_id, secured);

                Ok(with_header(
                    with_status(response, StatusCode::OK),
                    header::SET_COOKIE,
                    cookie,
                ))
            }
            Err(err) => Ok(with_header(
                with_status(
                    json(&to_json!(LoginResponse::err(&err.to_string()))),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ),
                header::WARNING,
                StatusCode::INTERNAL_SERVER_ERROR.as_str(),
            )),
        }
    } else {
        // If credentials don't match, return an error response
        Ok(with_header(
            with_status(
                json(&to_json!(LoginResponse::err("Invalid credentials"))),
                StatusCode::UNAUTHORIZED,
            ),
            header::WARNING,
            StatusCode::UNAUTHORIZED.as_str(),
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
