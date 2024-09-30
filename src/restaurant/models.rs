use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Restaurant {
    pub id: u32,
    pub name: String,
    pub max_capacity: u32,
    pub location: String,
    pub active: bool,
}
