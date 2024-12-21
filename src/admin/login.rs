use maud::html;
use serde_json::json as to_json;
use std::sync::Arc;
use warp::{
    filters::body,
    http::{header, StatusCode},
    reply::{json, with_header, with_status},
    Filter, Rejection, Reply,
};

use crate::{
    admin::auth::get_cookie_session_id,
    config::{context::with_context, models::Context},
    utils::{env_util::is_prod, html_util::render_html},
};

use super::{
    models::{LoginRequest, LoginResponse},
    repo::VerifyUser,
    sessions::EnableSession,
};

// Define the route for login
pub fn admin_login_route(
    context: Arc<Context<impl EnableSession + VerifyUser + Send + Sync>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let json_body = warp::post()
        .and(warp::path!("admin" / "login"))
        .and(warp::header::exact("Content-Type", "application/json"))
        .and(warp::body::json())
        .and(with_context(context.clone()))
        .and_then(handle_admin_login);

    let form_body = warp::post()
        .and(warp::path!("admin" / "login"))
        .and(warp::header::exact(
            "Content-Type",
            "application/x-www-form-urlencoded",
        ))
        .and(body::form::<LoginRequest>()) // Parse form body
        .and(with_context(context))
        .and_then(handle_admin_login);

    json_body.or(form_body)
}

pub fn admin_login_form_route(
    context: Arc<Context<impl EnableSession + VerifyUser + Send + Sync>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path!("admin" / "login"))
        .and(warp::header::optional::<String>("Cookie"))
        .and(with_context(context.clone()))
        .and_then(move |cookie: Option<String>, context| async move {
            render_admin_login(cookie, context).await
        })
}

// The handler for the admin login
async fn handle_admin_login(
    body: LoginRequest,
    context: Arc<Context<impl EnableSession + VerifyUser + Send + Sync>>,
) -> Result<impl Reply, Rejection> {
    // If credentials match, return a success response
    let session_manager = context.get();
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

// Generates HTML content for the login form
fn login_form_content() -> maud::Markup {
    html! {
        div class="container mt-5" {
            div class="row justify-content-center" {
                div class="col-md-6" {
                    div class="card shadow-lg" {
                        div class="card-header bg-primary text-white" {
                            h4 class="mb-0" { "Admin Login" }
                        }
                        div class="card-body" {
                            form method="POST" action="/admin/login" enctype="application/x-www-form-urlencoded" {
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
    }
}

// Generates HTML content for logged-in users
fn logged_in_content(username: &str) -> maud::Markup {
    html! {
        div class="container mt-5" {
            div class="alert alert-success" role="alert" {
                h4 class="alert-heading" { "Logged in already!" }
                p { "Welcome, " (username) "." }
                hr;
                p { "You're already logged in as an admin." }
            }
        }
    }
}

async fn render_admin_login(
    cookie: Option<String>,
    context: Arc<Context<impl EnableSession + VerifyUser + Send + Sync>>,
) -> Result<impl Reply, Rejection> {
    let restaurant = context.restaurant();

    // Retrieve session ID from the cookie
    if let Some(session_id) = get_cookie_session_id(cookie) {
        let session_manager = context.get();
        match session_manager.get_session(&session_id).await {
            Ok(session) => {
                let username = session
                    .get::<String>("user")
                    .unwrap_or_else(|| "unknown".to_string());
                return Ok(warp::reply::html(
                    render_html(&restaurant, logged_in_content(&username)).into_string(),
                ));
            }
            Err(err) => {
                log::warn!("Failed to retrieve session: {:?}", err);
            }
        }
    } else {
        log::debug!("No session_id found in cookies.");
    }

    // Render login form if session is not found or invalid
    Ok(warp::reply::html(
        render_html(&restaurant, login_form_content()).into_string(),
    ))
}
