use serde_json::json as to_json;
use std::sync::Arc;
use warp::{
    http::StatusCode,
    reply::{json, with_status},
    Filter, Rejection, Reply,
};

use super::{
    models::{LoginRequest, LoginResponse},
    repo::AdminRepo,
    sessions::EnableSession,
};

type Error = String;

// Define the route for login
pub fn admin_login_route<R>(
    admin_repo: Arc<R>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    R: AdminRepo + EnableSession<Error> + Send + Sync + 'static,
{
    warp::post()
        .and(warp::path!("admin" / "login"))
        .and(warp::body::json())
        .and(with_admin_repo(admin_repo))
        .and_then(handle_admin_login)
}

// The handler for the admin login
async fn handle_admin_login<R>(
    body: LoginRequest,
    admin_repo: Arc<R>,
) -> Result<impl Reply, Rejection>
where
    R: AdminRepo + EnableSession<Error> + Send + Sync + 'static,
{
    // If credentials match, return a success response
    if admin_repo.verify(&body.username, &body.password).await {
        match admin_repo.create_session(&body.username).await {
            Ok(token) => Ok(with_status(
                json(&to_json!(LoginResponse::ok(&token))),
                StatusCode::OK,
            )),
            Err(err) => Ok(with_status(
                json(&to_json!(LoginResponse::err(&err))),
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
fn with_admin_repo<R>(
    admin_repo: Arc<R>,
) -> impl Filter<Extract = (Arc<R>,), Error = std::convert::Infallible> + Clone
where
    R: AdminRepo + EnableSession<Error> + Send + Sync + 'static,
{
    warp::any().map(move || admin_repo.clone())
}
