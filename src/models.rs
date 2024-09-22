use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ReservationStatus {
    Pending,
    Confirmed,
    Cancelled,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub last_reservation_time: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reservation {
    pub id: i32,
    pub restaurant_id: i32,
    pub customer_id: i32,
    pub table_size: u8,
    pub reservation_time: NaiveDateTime,
    pub notes: Option<String>,
    pub status: ReservationStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Restaurant {
    pub id: i32,
    pub name: String,
    pub max_capacity: u32,
    pub location: String,
    pub active: bool,
}
