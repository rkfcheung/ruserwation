use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::app::enums::ReservationStatus;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Customer {
    pub id: u32,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub last_reservation_time: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reservation {
    pub id: u32,
    pub restaurant_id: u32,
    pub customer_id: u32,
    pub table_size: u8,
    pub reservation_time: NaiveDateTime,
    pub notes: Option<String>,
    pub status: ReservationStatus,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Restaurant {
    pub id: u32,
    pub name: String,
    pub max_capacity: u32,
    pub location: String,
    pub active: bool,
}
