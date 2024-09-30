mod admin;
mod restaurant;

use admin::models::Admin;
use dotenv;
use log::info;
use restaurant::models::Restaurant;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let restaurant = Restaurant {
        id: 1,
        name: "Hello World".to_string(),
        max_capacity: 64,
        location: "0x0".to_string(),
        active: true,
    };
    info!("{:?}", restaurant);

    let admin = Admin::init();
    info!(
        "Admin [username={}, email={}] is created!",
        admin.username, admin.email
    );
}
