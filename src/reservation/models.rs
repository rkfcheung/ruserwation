use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ReservationStatus {
    Pending,
    Confirmed,
    Cancelled,
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
