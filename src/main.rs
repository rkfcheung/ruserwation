mod admin;
mod restaurant;
mod utils;

use dotenv;
use log::info;
use restaurant::{index::index_route, models::Restaurant};
use utils::env_util::{var_as_int_or, var_as_str, var_as_str_or};
use warp::Filter;

#[tokio::main]
async fn main() {
    let app_env = var_as_str("APP_ENV");
    match app_env.as_str() {
        "prod" => dotenv::from_filename(".env.prod").ok(),
        _ => dotenv::dotenv().ok(),
    };

    env_logger::init();

    let rest_name = var_as_str_or("RW_REST_NAME", "<Name>".to_string());
    let rest_max_capacity = var_as_int_or("RW_REST_MAX_CAPACITY", 64) as u32;
    let rest_location = var_as_str_or("RW_REST_LOCATION", "<Location>".to_string());

    let restaurant = Restaurant {
        id: 1,
        name: rest_name,
        max_capacity: rest_max_capacity,
        location: rest_location,
        active: true,
    };
    info!("{:?}", restaurant);

    let static_route = warp::path("static").and(warp::fs::dir("./static"));
    let index_route = index_route(restaurant);

    let routes = warp::get().and(static_route).or(index_route);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
