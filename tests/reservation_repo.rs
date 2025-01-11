mod common;

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use ruserwation::common::Repo;
    use ruserwation::reservation::models::*;
    use ruserwation::reservation::sqlite::SqliteReservationRepo;
    use std::sync::Arc;

    use crate::common::db_utils;

    #[tokio::test]
    async fn test_insert_reservation() {
        let pool = db_utils::init_test_db()
            .await
            .expect("Failed to create test DB!");
        let repo = SqliteReservationRepo::new(Arc::new(pool));

        let mut reservation = Reservation {
            id: 0,
            book_ref: "TEST_REF".to_string(),
            restaurant_id: 1,
            customer_email: "test@example.com".to_string(),
            customer_name: "Test User".to_string(),
            customer_phone: "1234567890".to_string(),
            table_size: 4,
            reservation_time: Utc::now().naive_utc(),
            notes: Some("Test note".to_string()),
            status: ReservationStatus::Pending,
            updated_at: Utc::now().naive_utc(),
        };

        let id = repo.save(&mut reservation).await;
        assert!(id > 0);
    }
}
