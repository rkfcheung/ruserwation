mod common;

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use ruserwation::common::Repo;
    use ruserwation::reservation::models::*;
    use ruserwation::reservation::repo::ReservationRepo;
    use ruserwation::reservation::sqlite::SqliteReservationRepo;
    use std::sync::Arc;

    use crate::common::db_utils;

    #[tokio::test]
    async fn test_error_handling() {
        if std::env::var("RUST_LOG").is_ok() {
            let _ = env_logger::try_init();
        }
        let repo = prepare_repo().await;

        // Try to find a reservation that doesn't exist
        let result = repo.find_by_id(9999).await;
        assert!(result.is_none());

        // Insert an invalid reservation (e.g., missing required fields)
        let mut invalid_reservation = Reservation {
            id: 0,
            book_ref: "".to_string(),
            restaurant_id: 0,
            customer_email: "".to_string(),
            customer_name: "".to_string(),
            customer_phone: "".to_string(),
            table_size: 0,
            reservation_time: Utc::now().naive_utc(),
            notes: None,
            status: ReservationStatus::Pending,
            updated_at: Utc::now().naive_utc(),
        };

        let result = repo.save(&mut invalid_reservation).await;
        assert_eq!(result, 0);
    }

    #[tokio::test]
    async fn test_find_all_by_query() {
        let repo = prepare_repo().await;

        let reservation_1 = Reservation {
            id: 0,
            book_ref: "REF1".to_string(),
            restaurant_id: 1,
            customer_email: "user1@example.com".to_string(),
            customer_name: "User One".to_string(),
            customer_phone: "1234567891".to_string(),
            table_size: 4,
            reservation_time: Utc::now().naive_utc(),
            notes: Some("First".to_string()),
            status: ReservationStatus::Pending,
            updated_at: Utc::now().naive_utc(),
        };

        let reservation_2 = Reservation {
            id: 0,
            book_ref: "REF2".to_string(),
            restaurant_id: 2,
            customer_email: "user2@example.com".to_string(),
            customer_name: "User Two".to_string(),
            customer_phone: "1234567892".to_string(),
            table_size: 2,
            reservation_time: Utc::now().naive_utc(),
            notes: Some("Second".to_string()),
            status: ReservationStatus::Confirmed,
            updated_at: Utc::now().naive_utc(),
        };

        // Save the reservations
        repo.save(&mut reservation_1.clone()).await;
        repo.save(&mut reservation_2.clone()).await;

        // Find all reservations for customer_name = User One
        let results = repo
            .find_all_by_query(ReservationQuery::default().customer_name("User One"))
            .await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].restaurant_id, 1);

        // Find all confirmed reservations
        let results = repo
            .find_all_by_query(ReservationQuery::default().status(ReservationStatus::Confirmed))
            .await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].status, ReservationStatus::Confirmed);
    }

    #[tokio::test]
    async fn test_find_by_status() {
        let repo = prepare_repo().await;

        let mut pending_reservation = Reservation {
            id: 0,
            book_ref: "PENDING_REF".to_string(),
            restaurant_id: 1,
            customer_email: "pending@example.com".to_string(),
            customer_name: "Pending User".to_string(),
            customer_phone: "1234567890".to_string(),
            table_size: 2,
            reservation_time: Utc::now().naive_utc(),
            notes: None,
            status: ReservationStatus::Pending,
            updated_at: Utc::now().naive_utc(),
        };

        let mut confirmed_reservation = pending_reservation.clone();
        confirmed_reservation.book_ref = "CONFIRMED_REF".to_string();
        confirmed_reservation.status = ReservationStatus::Confirmed;

        repo.save(&mut pending_reservation).await;
        repo.save(&mut confirmed_reservation).await;

        let pending = repo.find_by_status(ReservationStatus::Pending).await;
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].book_ref, "PENDING_REF");

        let confirmed = repo.find_by_status(ReservationStatus::Confirmed).await;
        assert_eq!(confirmed.len(), 1);
        assert_eq!(confirmed[0].book_ref, "CONFIRMED_REF");
    }

    #[tokio::test]
    async fn test_find_by_time() {
        let repo = prepare_repo().await;

        let mut reservation_1 = Reservation {
            id: 0,
            book_ref: "REF1".to_string(),
            restaurant_id: 1,
            customer_email: "time1@example.com".to_string(),
            customer_name: "Time One".to_string(),
            customer_phone: "1234567890".to_string(),
            table_size: 2,
            reservation_time: Utc::now().naive_utc(),
            notes: None,
            status: ReservationStatus::Pending,
            updated_at: Utc::now().naive_utc(),
        };

        let mut reservation_2 = reservation_1.clone();
        reservation_2.book_ref = "REF2".to_string();
        reservation_2.reservation_time = Utc::now().naive_utc() + chrono::Duration::days(1);

        repo.save(&mut reservation_1).await;
        repo.save(&mut reservation_2).await;

        let start_time = Utc::now().naive_utc() - chrono::Duration::hours(1);
        let end_time = Utc::now().naive_utc() + chrono::Duration::hours(1);

        let results = repo.find_by_time(start_time, end_time).await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].book_ref, "REF1");
    }

    #[tokio::test]
    async fn test_save_reservation() {
        let repo = prepare_repo().await;

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

        let saved = repo.find_by_id(id).await;
        assert_eq!(saved.unwrap(), reservation);

        reservation.notes = Some("Updated".to_string());
        reservation.updated_at = Utc::now().naive_utc();
        assert_eq!(repo.save(&mut reservation).await, id);

        let updated = repo.find_by_book_ref(&reservation.book_ref).await;
        assert_eq!(updated.unwrap(), reservation);
    }

    async fn prepare_repo() -> SqliteReservationRepo {
        let pool = db_utils::init_test_db()
            .await
            .expect("Failed to create test DB!");
        SqliteReservationRepo::new(Arc::new(pool))
    }
}
