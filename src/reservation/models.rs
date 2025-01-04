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
    pub book_ref: String,
    pub restaurant_id: u32,
    pub customer_email: String,
    pub customer_name: String,
    pub customer_phone: String,
    pub table_size: u8,
    pub reservation_time: NaiveDateTime,
    pub notes: Option<String>,
    pub status: ReservationStatus,
}

#[derive(Clone, Debug, Default)]
pub struct ReservationQuery {
    pub book_ref: Option<String>,
    pub customer_email: Option<String>,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub from_time: Option<NaiveDateTime>,
    pub to_time: Option<NaiveDateTime>,
    pub status: Option<ReservationStatus>,
    has_some: bool,
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

impl ReservationQuery {
    pub fn book_ref(mut self, book_ref: &str) -> Self {
        self.book_ref = Some(book_ref.to_string());
        self.has_some = true;
        self
    }

    pub fn customer_email(mut self, customer_email: &str) -> Self {
        self.customer_email = Some(customer_email.to_string());
        self.has_some = true;
        self
    }

    pub fn customer_name(mut self, customer_name: &str) -> Self {
        self.customer_name = Some(customer_name.to_string());
        self.has_some = true;
        self
    }

    pub fn customer_phone(mut self, customer_phone: &str) -> Self {
        self.customer_phone = Some(customer_phone.to_string());
        self.has_some = true;
        self
    }

    pub fn from_time(mut self, from_time: NaiveDateTime) -> Self {
        self.from_time = Some(from_time);
        self.has_some = true;
        self
    }

    pub fn to_time(mut self, to_time: NaiveDateTime) -> Self {
        self.to_time = Some(to_time);
        self.has_some = true;
        self
    }

    pub fn status(mut self, status: ReservationStatus) -> Self {
        self.status = Some(status);
        self.has_some = true;
        self
    }

    pub fn is_none(&self) -> bool {
        !self.has_some
    }

    pub fn create(&self) -> String {
        if self.is_none() {
            "".to_string()
        } else {
            let mut qry = String::new();
            qry.push_str("SELECT * FROM reservations WHERE 1 = 1 ");

            if let Some(value) = &self.book_ref {
                qry.push_str(&format!("AND book_ref = '{}' ", value));
            }
            if let Some(value) = &self.customer_email {
                qry.push_str(&format!("AND customer_email = '{}' ", value));
            }
            if let Some(value) = &self.customer_name {
                qry.push_str(&format!("AND customer_name = '{}' ", value));
            }
            if let Some(value) = &self.customer_phone {
                qry.push_str(&format!("AND customer_phone = '{}' ", value));
            }
            if let Some(value) = &self.from_time {
                qry.push_str(&format!("AND reservation_time >= '{}' ", value));
            }
            if let Some(value) = &self.to_time {
                qry.push_str(&format!("AND reservation_time <= '{}' ", value));
            }
            if let Some(value) = &self.status {
                qry.push_str(&format!("AND status = '{}' ", value));
            }

            qry
        }
    }
}
