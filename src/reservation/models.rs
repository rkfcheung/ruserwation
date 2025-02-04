use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteArguments;
use sqlx::Arguments;
use std::fmt;
use warp::reject::Reject;

use super::helper::generate_random_book_ref;
use crate::db::QueryError;
use crate::response::Response;

#[derive(Deserialize)]
pub struct Customer {
    pub email: String,
    pub name: String,
    pub phone: String,
}

impl Customer {
    pub fn new(email: &str, name: &str, phone: &str) -> Self {
        Self {
            email: email.to_string(),
            name: name.to_string(),
            phone: phone.to_string(),
        }
    }
}

/// Enum for Reservation Status.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, sqlx::Type)]
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
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, sqlx::FromRow)]
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
    pub assigned_table: Option<String>,
    pub updated_at: NaiveDateTime,
}

impl Reservation {
    pub fn new(
        customer_email: &str,
        customer_name: &str,
        customer_phone: &str,
        table_size: u8,
        reservation_time: NaiveDateTime,
        notes: Option<String>,
    ) -> Self {
        Self::new_with_book_ref(
            &generate_random_book_ref(5),
            customer_email,
            customer_name,
            customer_phone,
            table_size,
            reservation_time,
            notes,
        )
    }

    pub fn new_with_book_ref(
        book_ref: &str,
        customer_email: &str,
        customer_name: &str,
        customer_phone: &str,
        table_size: u8,
        reservation_time: NaiveDateTime,
        notes: Option<String>,
    ) -> Self {
        Self {
            id: 0,
            book_ref: book_ref.to_string(),
            restaurant_id: 1,
            customer_email: customer_email.to_string(),
            customer_name: customer_name.to_string(),
            customer_phone: customer_phone.to_string(),
            table_size,
            reservation_time,
            notes,
            status: ReservationStatus::Pending,
            assigned_table: None,
            updated_at: Utc::now().naive_utc(),
        }
    }
}

impl From<ReservationRequest> for Reservation {
    fn from(value: ReservationRequest) -> Self {
        match value.book_ref {
            Some(book_ref) => Reservation::new_with_book_ref(
                &book_ref,
                &value.customer.email,
                &value.customer.name,
                &value.customer.phone,
                value.table_size,
                value.reservation_time,
                value.notes,
            ),
            None => Reservation::new(
                &value.customer.email,
                &value.customer.name,
                &value.customer.phone,
                value.table_size,
                value.reservation_time,
                value.notes,
            ),
        }
    }
}

#[derive(Deserialize)]
pub struct ReservationRequest {
    book_ref: Option<String>,
    customer: Customer,
    table_size: u8,
    reservation_time: NaiveDateTime,
    notes: Option<String>,
    ref_check: String,
}

impl ReservationRequest {
    pub fn new(
        customer: Customer,
        table_size: u8,
        reservation_time: NaiveDateTime,
        notes: Option<String>,
        ref_check: &str,
    ) -> Self {
        Self::new_with_book_ref(
            Option::None,
            customer,
            table_size,
            reservation_time,
            notes,
            ref_check,
        )
    }

    pub fn new_with_book_ref(
        book_ref: Option<String>,
        customer: Customer,
        table_size: u8,
        reservation_time: NaiveDateTime,
        notes: Option<String>,
        ref_check: &str,
    ) -> Self {
        Self {
            book_ref,
            customer,
            table_size,
            reservation_time,
            notes,
            ref_check: ref_check.to_string(),
        }
    }

    pub fn has_book_ref(&self) -> bool {
        self.book_ref.is_some()
    }

    pub fn ref_check(&self) -> &str {
        &self.ref_check
    }
}

#[derive(Debug, Serialize)]
pub struct ReservationResponse {
    book_ref: String,
    #[serde(flatten)]
    response: Response,
}

/// Query builder for filtering Reservations.
#[derive(Clone, Debug, Default)]
pub struct ReservationQuery {
    pub id: Option<u32>,
    pub book_ref: Option<String>,
    pub customer_email: Option<String>,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
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
    pub fn start_time(mut self, from: NaiveDateTime) -> Self {
        self.start_time = Some(from);
        self
    }

    /// Adds a `to_time` filter.
    pub fn end_time(mut self, to: NaiveDateTime) -> Self {
        self.end_time = Some(to);
        self
    }

    /// Adds a `status` filter.
    pub fn status(mut self, status: ReservationStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Constructs a parameterised SQL query for filtering reservations.
    pub fn create(&self) -> Result<(String, SqliteArguments), QueryError> {
        let mut conditions = Vec::new();
        let mut args = SqliteArguments::default();

        if let Some(ref value) = self.id {
            conditions.push("id = ?");
            args.add(value)?;
        }
        if let Some(ref value) = self.book_ref {
            conditions.push("book_ref = ?");
            args.add(value)?;
        }
        if let Some(ref value) = self.customer_email {
            conditions.push("customer_email = ?");
            args.add(value)?;
        }
        if let Some(ref value) = self.customer_name {
            conditions.push("customer_name = ?");
            args.add(value)?;
        }
        if let Some(ref value) = self.customer_phone {
            conditions.push("customer_phone = ?");
            args.add(value)?;
        }
        if let Some(ref value) = self.start_time {
            conditions.push("reservation_time >= ?");
            args.add(value)?;
        }
        if let Some(ref value) = self.end_time {
            conditions.push("reservation_time <= ?");
            args.add(value)?;
        }
        if let Some(ref value) = self.status {
            conditions.push("status = ?");
            args.add(value.to_string())?;
        }

        if conditions.is_empty() {
            return Err(QueryError::NoConditionsProvided);
        }

        let query = format!(
            "SELECT * FROM Reservation WHERE {}",
            conditions.join(" AND ")
        );

        Ok((query, args))
    }
}

impl ReservationResponse {
    pub fn ok(book_ref: &str) -> Self {
        Self {
            book_ref: book_ref.to_string(),
            response: Response::ok("Booked successful"),
        }
    }

    pub fn err(msg: &str) -> Self {
        Self::err_with_book_ref("<Failed to Book>", msg)
    }

    pub fn err_with_book_ref(book_ref: &str, msg: &str) -> Self {
        Self {
            book_ref: book_ref.to_string(),
            response: Response::err(msg),
        }
    }
}

impl Reject for ReservationResponse {}
