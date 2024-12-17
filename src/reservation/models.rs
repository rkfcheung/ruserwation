use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt;

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

// Convert to String for DB
impl fmt::Display for ReservationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match self {
            ReservationStatus::Pending => "Pending",
            ReservationStatus::Confirmed => "Confirmed",
            ReservationStatus::Cancelled => "Cancelled",
        };
        write!(f, "{}", status)
    }
}

// Convert from String when reading from DB
impl From<&str> for ReservationStatus {
    fn from(status: &str) -> Self {
        match status {
            "Pending" => ReservationStatus::Pending,
            "Confirmed" => ReservationStatus::Confirmed,
            "Cancelled" => ReservationStatus::Cancelled,
            _ => ReservationStatus::Pending, // Default to Pending for unknown status
        }
    }
}
