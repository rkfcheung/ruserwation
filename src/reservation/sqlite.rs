use sqlx::{query, query_as_with, Sqlite, SqlitePool};
use std::sync::Arc;

use super::{
    helper::validate_reservation,
    models::{Reservation, ReservationQuery},
    repo::ReservationRepo,
};
use crate::{common::Repo, db::QueryError};

pub struct SqliteReservationRepo {
    pool: Arc<SqlitePool>, // SQLite connection pool
}

impl SqliteReservationRepo {
    // Create a new repository with a database connection pool
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    async fn insert(&self, reservation: &mut Reservation) -> Result<u32, sqlx::Error> {
        let result = query(
            r#"
            INSERT INTO Reservation (
                book_ref, 
                restaurant_id, 
                customer_email, 
                customer_name, 
                customer_phone, 
                table_size, 
                reservation_time, 
                notes, 
                status,
                updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
            "#,
        )
        .bind(&reservation.book_ref) // Bind book_ref
        .bind(reservation.restaurant_id) // Bind restaurant_id
        .bind(&reservation.customer_email) // Bind customer_email
        .bind(&reservation.customer_name) // Bind customer_name
        .bind(&reservation.customer_phone) // Bind customer_phone
        .bind(reservation.table_size) // Bind table_size
        .bind(reservation.reservation_time) // Bind reservation_time
        .bind(&reservation.notes) // Bind notes (optional)
        .bind(reservation.status.to_string()) // Bind status (convert enum to string)
        .bind(reservation.updated_at) // Bind updated_at
        .execute(self.pool.as_ref())
        .await?;

        reservation.id = result.last_insert_rowid() as u32; // Get the last inserted ID
        Ok(reservation.id)
    }

    async fn update(&self, reservation: &Reservation) -> Result<u32, sqlx::Error> {
        let _ = query(
            r#"
            UPDATE Reservation
            SET
                book_ref = ?,
                restaurant_id = ?,
                customer_email = ?,
                customer_name = ?,
                customer_phone = ?,
                table_size = ?,
                reservation_time = ?,
                notes = ?,
                status = ?,
                assigned_table = ?,
                updated_at = ?
            WHERE id = ?;
            "#,
        )
        .bind(&reservation.book_ref) // Bind book_ref
        .bind(reservation.restaurant_id) // Bind restaurant_id
        .bind(&reservation.customer_email) // Bind customer_email
        .bind(&reservation.customer_name) // Bind customer_name
        .bind(&reservation.customer_phone) // Bind customer_phone
        .bind(reservation.table_size) // Bind table_size
        .bind(reservation.reservation_time) // Bind reservation_time
        .bind(&reservation.notes) // Bind notes (optional)
        .bind(reservation.status.to_string()) // Bind status (convert enum to string)
        .bind(&reservation.assigned_table) // Bind assigned_table (optional)
        .bind(reservation.updated_at) // Bind updated_at
        .bind(reservation.id) // Bind id for the WHERE clause
        .execute(self.pool.as_ref())
        .await?;

        Ok(reservation.id)
    }
}

impl Repo<u32, Reservation> for SqliteReservationRepo {
    async fn find_by_id(&self, id: u32) -> Option<Reservation> {
        self.find_one_by_query(ReservationQuery::default().id(id))
            .await
    }

    async fn save(&self, entity: &mut Reservation) -> Result<u32, QueryError> {
        if let Err(e) = validate_reservation(entity) {
            log::error!("Invalid Reservation found: {e}");
            return Err(QueryError::InvalidQuery(e));
        }

        let result = if entity.id == 0 {
            self.insert(entity).await
        } else {
            self.update(entity).await
        };
        match result {
            Ok(id) => Ok(id),
            Err(e) => {
                log::error!(
                    "Failed to save Reservation with Book Ref '{}': {:?}",
                    entity.book_ref,
                    e
                );
                Err(e.into())
            }
        }
    }
}

impl ReservationRepo for SqliteReservationRepo {
    async fn find_all_by_query(&self, query: ReservationQuery) -> Vec<Reservation> {
        let (sql, args) = match query.create() {
            Ok(result) => result,
            Err(e) => {
                log::error!(
                    "Failed to create ReservationQuery '{:?}' to find all: {e}",
                    query
                );
                return Vec::new();
            }
        };

        match query_as_with::<Sqlite, Reservation, _>(&sql, args)
            .fetch_all(self.pool.as_ref())
            .await
        {
            Ok(reservations) => reservations,
            Err(e) => {
                log::error!("Failed to find Reservations '{:?}': {:?}", query, e);
                Vec::new()
            }
        }
    }

    async fn find_one_by_query(&self, query: ReservationQuery) -> Option<Reservation> {
        let (sql, args) = match query.create() {
            Ok(result) => result,
            Err(e) => {
                log::error!("Failed to find Reservations '{:?}' to find one: {e}", query);
                return None;
            }
        };
        log::debug!("Executing query '{}', args: {:?} ...", sql, args);

        query_as_with::<Sqlite, Reservation, _>(&sql, args)
            .fetch_one(self.pool.as_ref())
            .await
            .ok()
    }
}
