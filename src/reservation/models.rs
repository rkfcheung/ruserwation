use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::error::BoxDynError;
use sqlx::sqlite::SqliteArguments;
use sqlx::Arguments;
use std::fmt;

/// Enum for Reservation Status.
#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type)]
pub enum ReservationStatus {
    Pending,
    Confirmed,
    Cancelled,
}

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

impl From<&str> for ReservationStatus {
    fn from(status: &str) -> Self {
        match status {
            "Pending" => ReservationStatus::Pending,
            "Confirmed" => ReservationStatus::Confirmed,
            "Cancelled" => ReservationStatus::Cancelled,
            _ => ReservationStatus::Pending, // Default fallback
        }
    }
}

/// Struct for the Reservation table.
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
    pub updated_at: NaiveDateTime,
}

/// Query builder for filtering Reservations.
#[derive(Clone, Debug, Default)]
pub struct ReservationQuery {
    pub id: Option<u32>,
    pub book_ref: Option<String>,
    pub customer_email: Option<String>,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub from_time: Option<NaiveDateTime>,
    pub to_time: Option<NaiveDateTime>,
    pub status: Option<ReservationStatus>,
}

impl ReservationQuery {
    /// Adds a `id` filter.
    pub fn id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }

    /// Adds a `book_ref` filter.
    pub fn book_ref(mut self, book_ref: &str) -> Self {
        self.book_ref = Some(book_ref.to_string());
        self
    }

    /// Adds a `customer_email` filter.
    pub fn customer_email(mut self, email: &str) -> Self {
        self.customer_email = Some(email.to_string());
        self
    }

    /// Adds a `customer_name` filter.
    pub fn customer_name(mut self, name: &str) -> Self {
        self.customer_name = Some(name.to_string());
        self
    }

    /// Adds a `customer_phone` filter.
    pub fn customer_phone(mut self, phone: &str) -> Self {
        self.customer_phone = Some(phone.to_string());
        self
    }

    /// Adds a `from_time` filter.
    pub fn from_time(mut self, from: NaiveDateTime) -> Self {
        self.from_time = Some(from);
        self
    }

    /// Adds a `to_time` filter.
    pub fn to_time(mut self, to: NaiveDateTime) -> Self {
        self.to_time = Some(to);
        self
    }

    /// Adds a `status` filter.
    pub fn status(mut self, status: ReservationStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Checks if the query has any filters.
    pub fn is_none(&self) -> bool {
        self.book_ref.is_none()
            && self.customer_email.is_none()
            && self.customer_name.is_none()
            && self.customer_phone.is_none()
            && self.from_time.is_none()
            && self.to_time.is_none()
            && self.status.is_none()
    }

    /// Constructs a parameterized SQL query for filtering reservations.
    pub fn create(&self) -> Result<(String, SqliteArguments), BoxDynError> {
        let mut query = String::from("SELECT * FROM reservations WHERE 1=1");
        let mut args = SqliteArguments::default();

        if let Some(ref value) = self.id {
            query.push_str(" AND id = ?");
            args.add(value)?;
        }
        if let Some(ref value) = self.book_ref {
            query.push_str(" AND book_ref = ?");
            args.add(value)?;
        }
        if let Some(ref value) = self.customer_email {
            query.push_str(" AND customer_email = ?");
            args.add(value)?;
        }
        if let Some(ref value) = self.customer_name {
            query.push_str(" AND customer_name = ?");
            args.add(value)?;
        }
        if let Some(ref value) = self.customer_phone {
            query.push_str(" AND customer_phone = ?");
            args.add(value)?;
        }
        if let Some(ref value) = self.from_time {
            query.push_str(" AND reservation_time >= ?");
            args.add(value)?;
        }
        if let Some(ref value) = self.to_time {
            query.push_str(" AND reservation_time <= ?");
            args.add(value)?;
        }
        if let Some(ref value) = self.status {
            query.push_str(" AND status = ?");
            args.add(value.to_string())?;
        }

        Ok((query, args))
    }
}
