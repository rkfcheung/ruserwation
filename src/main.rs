mod admin;
mod restaurant;

use dotenv;
use log::info;
use restaurant::{index::index_route, models::Restaurant};
use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    let environment = env::var("APP_ENV").unwrap_or_default();
    match environment.as_str() {
        "prod" => dotenv::from_filename(".env.prod").ok(),
        _ => dotenv::dotenv().ok(),
    };

    env_logger::init();

    let rest_name = env::var("RW_REST_NAME").unwrap_or("<Name>".to_string());
    let rest_max_capacity = env::var("RW_REST_MAX_CAPACITY")
        .unwrap_or("64".to_string())
        .parse()
        .unwrap_or(64);
    let rest_location = env::var("RW_REST_LOCATION").unwrap_or("<Location>".to_string());

    let restaurant = Restaurant {
        id: 1,
        name: rest_name,
        max_capacity: rest_max_capacity,
        location: rest_location,
        active: true,
    };
    info!("{:?}", restaurant);

    let static_route = warp::fs::dir("./static");
    let index_route = index_route(restaurant);

    let routes = warp::get().and(static_route.or(index_route));

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
