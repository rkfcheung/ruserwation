mod admin;
mod restaurant;

use dotenv;
use log::info;
use restaurant::{index::index_route, models::Restaurant};
use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
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

    let static_files = warp::path("static").and(warp::fs::dir("./static"));
    let index_route = index_route(restaurant);

    let routes = warp::get().and(static_files.or(index_route));

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
