use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ReservationStatus {
    Pending,
    Confirmed,
    Cancelled,
}

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Reservation {
    pub id: u32,
    pub restaurant_id: u32,
    pub customer_id: u32,
    pub table_size: u8,
    pub reservation_time: NaiveDateTime,
    pub notes: Option<String>,
    pub status: ReservationStatus,
}

impl ReservationStatus {
    // Convert to String for DB
    pub fn to_string(&self) -> String {
        match self {
            ReservationStatus::Pending => "Pending".to_string(),
            ReservationStatus::Confirmed => "Confirmed".to_string(),
            ReservationStatus::Cancelled => "Cancelled".to_string(),
        }
    }

    // Convert from String when reading from DB
    pub fn from_string(status: &str) -> ReservationStatus {
        match status {
            "Pending" => ReservationStatus::Pending,
            "Confirmed" => ReservationStatus::Confirmed,
            "Cancelled" => ReservationStatus::Cancelled,
            _ => panic!("Unknown status: {}", status),
        }
    }
}
