mod restaurant;

use dotenv;
use restaurant::models::Restaurant;

fn main() {
    dotenv::dotenv().ok();

    let restaurant = Restaurant {
        id: 1,
        name: "Hello World".to_string(),
        max_capacity: 64,
        location: "0x0".to_string(),
        active: true,
    };
    println!("{:?}", restaurant);
}
