use maud::html;
use serde_json::json as to_json;
use std::{convert::Infallible, sync::Arc};
use warp::{
    http::{header, StatusCode},
    reply::{json, with_header, with_status},
    Filter, Rejection, Reply,
};

use crate::{
    restaurant::models::Restaurant,
    utils::{env_util::is_prod, html_util::render_html},
};

use super::{
    models::{LoginRequest, LoginResponse},
    repo::VerifyUser,
    sessions::EnableSession,
};

// Define the route for login
pub fn admin_login_route(
    session_manager: Arc<impl EnableSession + VerifyUser + Send + Sync>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path!("admin" / "login"))
        .and(warp::body::json())
        .and(with_session_manager(session_manager))
        .and_then(handle_admin_login)
}

pub fn admin_login_form_route(
    session_manager: Arc<impl EnableSession + VerifyUser + Send + Sync>,
    restaurant: Arc<Restaurant>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path!("admin" / "login"))
        .and(warp::header::optional::<String>("Cookie"))
        .and(with_session_manager(session_manager.clone()))
        .and_then(move |cookie: Option<String>, session_manager| {
            let restaurant = restaurant.clone();
            async move { render_admin_login(cookie, session_manager, restaurant.clone()).await }
        })
}

async fn render_admin_login(
    cookie: Option<String>,
    session_manager: Arc<impl EnableSession + VerifyUser + Send + Sync>,
    restaurant: Arc<Restaurant>,
) -> Result<impl Reply, Rejection> {
    let session_id = cookie
        .and_then(|c| c.split('=').nth(1).map(|s| s.to_string()))
        .unwrap_or_default();

    if let Ok(session) = session_manager.get_session(&session_id).await {
        let username = session
            .get::<String>("user")
            .unwrap_or("unknown".to_string());
        let content = html! {
            div class="container mt-5" {
                div class="alert alert-success" role="alert" {
                    h4 class="alert-heading" { "Logged in already!" }
                    p { "Welcome, " (username) "." }
                    hr;
                    p { "You're already logged in as an admin." }
                }
            }
        };
        return Ok(warp::reply::html(
            render_html(&restaurant, content).into_string(),
        ));
    }

    let content = html! {
        div class="container mt-5" {
            div class="row justify-content-center" {
                div class="col-md-6" {
                    div class="card shadow-lg" {
                        div class="card-header bg-primary text-white" {
                            h4 class="mb-0" { "Admin Login" }
                        }
                        div class="card-body" {
                            form method="POST" action="/admin/login" {
                                div class="mb-3" {
                                    label for="username" class="form-label" { "Username" }
                                    input type="text" class="form-control" id="username" name="username" required="true" placeholder="Enter your username";
                                }
                                div class="mb-3" {
                                    label for="password" class="form-label" { "Password" }
                                    input type="password" class="form-control" id="password" name="password" required="true" placeholder="Enter your password";
                                }
                                div class="d-grid gap-2" {
                                    button type="submit" class="btn btn-primary btn-block" { "Login" }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    Ok(warp::reply::html(
        render_html(&restaurant, content).into_string(),
    ))
}

// The handler for the admin login
async fn handle_admin_login(
    body: LoginRequest,
    session_manager: Arc<impl EnableSession + VerifyUser + Send + Sync>,
) -> Result<impl Reply, Rejection> {
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
fn with_session_manager(
    session_manager: Arc<impl EnableSession + VerifyUser + Send + Sync>,
) -> impl Filter<Extract = (Arc<impl EnableSession + VerifyUser + Send + Sync>,), Error = Infallible>
       + Clone {
    warp::any().map(move || session_manager.clone())
}
