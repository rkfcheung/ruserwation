use sqlx::SqlitePool;
use std::sync::Arc;

use crate::common::Repo;

use super::{
    models::{Reservation, ReservationQuery},
    repo::ReservationRepo,
};

pub struct SqliteReservationRepo {
    pool: Arc<SqlitePool>, // SQLite connection pool
}

impl SqliteReservationRepo {
    // Create a new repository with a database connection pool
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

impl Repo<u32, Reservation> for SqliteReservationRepo {
    async fn find_by_id(&self, id: u32) -> Option<Reservation> {
        todo!()
    }

    async fn save(&self, entity: &mut Reservation) -> u32 {
        todo!()
    }
}

impl ReservationRepo for SqliteReservationRepo {
    async fn find_by_query(&self, query: ReservationQuery) -> Vec<Reservation> {
        todo!()
    }

    async fn find_by_book_ref(&self, book_ref: &str) -> Option<Reservation> {
        if let Some(value) = self
            .find_by_query(ReservationQuery::default().book_ref(book_ref))
            .await
            .first()
        {
            Some(value.clone())
        } else {
            None
        }
    }
}
