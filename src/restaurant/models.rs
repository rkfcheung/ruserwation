use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Restaurant {
    pub id: u32,
    pub name: String,
    pub max_capacity: u32,
    pub location: String,
    pub active: bool,
}

impl Restaurant {
    pub fn new(id: u32, name: &str, max_capacity: u32, location: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            max_capacity,
            location: location.to_string(),
            active: true,
        }
    }
}
