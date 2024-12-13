mod admin;
mod config;
mod db;
mod response;
mod restaurant;
mod setup;
mod utils;

use warp::Filter;

use admin::login::admin_login_route;
use log::info;
use response::handle_rejections;
use restaurant::{index::index_route, models::Restaurant};
use setup::{errors::SetupError, startup::init};
use utils::env_util::{is_prod, var_as_int_or, var_as_str_or};

#[tokio::main]
async fn main() -> Result<(), SetupError> {
    if is_prod() {
        dotenv::from_filename(".env.prod").ok();
    } else {
        dotenv::dotenv().ok();
    }

    let app_state = init().await?;

    let rest_name = var_as_str_or("RW_REST_NAME", "<Name>".to_string());
    let rest_max_capacity = var_as_int_or("RW_REST_MAX_CAPACITY", 64) as u32;
    let rest_location = var_as_str_or("RW_REST_LOCATION", "<Location>".to_string());

    let restaurant = Restaurant::new(1, &rest_name, rest_max_capacity, &rest_location);
    info!("{:?}", restaurant);

    let static_route = warp::path("static").and(warp::fs::dir("./static"));
    let index_route = index_route(restaurant);
    let admin_login_route = admin_login_route(app_state.session_manager());

    let routes = warp::get()
        .and(static_route)
        .or(index_route)
        .or(admin_login_route)
        .recover(handle_rejections);

    let rest_port = var_as_int_or("RW_REST_PORT", 3030) as u16;
    warp::serve(routes).run(([0, 0, 0, 0], rest_port)).await;

    Ok(())
}
