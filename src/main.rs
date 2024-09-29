mod app;

use crate::app::models::Restaurant;

fn main() {
    let restaurant = Restaurant {
        id: 1,
        name: "Hello World".to_string(),
        max_capacity: 64,
        location: "0x0".to_string(),
        active: true,
    };
    println!("{:?}", restaurant);
}
