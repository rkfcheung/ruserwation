use ruserwation::restaurant::models::Restaurant;

pub mod sessions;

pub(crate) fn fake_restaurant() -> Restaurant {
    Restaurant {
        id: 1,
        name: "Test Restaurant".to_string(),
        max_capacity: 32,
        location: "Test City".to_string(),
        active: true,
    }
}
