mod admin;
mod config;
mod db;
mod response;
mod restaurant;
mod setup;
mod utils;

use config::models::Context;
use warp::Filter;

use admin::{
    login::{admin_login_form_route, admin_login_route},
    logout::admin_logout_route,
};
use response::handle_rejections;
use restaurant::index::index_route;
use setup::{errors::SetupError, startup::init};
use utils::env_util::{is_prod, var_as_int_or};

#[tokio::main]
async fn main() -> Result<(), SetupError> {
    if is_prod() {
        dotenv::from_filename(".env.prod").ok();
    } else {
        dotenv::dotenv().ok();
    }

    let app_state = init().await?;
    let session_manager = app_state.session_manager();
    let restaurant = app_state.restaurant();

    let static_route = warp::path("static").and(warp::fs::dir("./static"));
    let routes = warp::get()
        .and(static_route)
        .or(index_route(restaurant.clone()))
        .or(admin_login_route(session_manager.clone()))
        .or(admin_login_form_route(
            session_manager.clone(),
            restaurant.clone(),
        ))
        .or(admin_logout_route(Context::create(
            session_manager.clone(),
            restaurant,
        )))
        .recover(handle_rejections);

    let rest_port = var_as_int_or("RW_REST_PORT", 3030) as u16;
    warp::serve(routes).run(([0, 0, 0, 0], rest_port)).await;

    Ok(())
}
