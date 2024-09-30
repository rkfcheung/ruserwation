use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Customer {
    pub id: u32,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub last_reservation_id: u32,
}
