use warp::{
    http::StatusCode,
    reply::{json, with_status},
    Filter, Rejection, Reply,
};

use super::{
    models::LoginRequest,
    repo::{AdminRepo, EnableSession},
    sqlite::SqliteAdminRepo,
};

// Define the route for login
pub fn admin_login_route<'a>(
    admin_repo: &'a SqliteAdminRepo<'a>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone + use<'a> {
    warp::post()
        .and(warp::path("admin/login"))
        .and(warp::body::json())
        .and(with_admin_repo(admin_repo))
        .and_then(handle_admin_login)
}

// The handler for the admin login
async fn handle_admin_login(
    body: LoginRequest,
    admin_repo: &SqliteAdminRepo<'_>,
) -> Result<impl Reply, Rejection> {
    // If credentials match, return a success response
    if admin_repo.verify(&body.username, &body.password).await {
        return match admin_repo.create_session(&body.username).await {
            Ok(sid) => Ok(with_status(
                json(&serde_json::json!({ "message": "Login successful", "session_id": sid })),
                StatusCode::OK,
            )),
            Err(err) => Ok(with_status(
                json(&serde_json::json!({ "message": err })),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        };
    }

    // If credentials don't match, return an error response
    Ok(with_status(
        json(&serde_json::json!({ "message": "Invalid credentials" })),
        StatusCode::UNAUTHORIZED,
    ))
}

// Helper function to attach admin_repo with correct lifetime
fn with_admin_repo<'a>(
    admin_repo: &'a SqliteAdminRepo<'a>,
) -> impl Filter<Extract = (&'a SqliteAdminRepo<'a>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || admin_repo)
}
