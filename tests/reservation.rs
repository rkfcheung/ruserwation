#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use ruserwation::reservation::{
        helper::validate_reservation,
        models::{Reservation, ReservationStatus},
    };

    // Helper function to create a sample reservation
    fn valid_reservation() -> Reservation {
        Reservation {
            id: 1,
            book_ref: "1234567890".to_string(),
            restaurant_id: 1,
            customer_email: "customer@example.com".to_string(),
            customer_name: "John Doe".to_string(),
            customer_phone: "+123456789".to_string(),
            table_size: 4,
            reservation_time: Utc::now().naive_utc(),
            notes: Some("This is a test note.".to_string()),
            status: ReservationStatus::Pending,
            updated_at: Utc::now().naive_utc(),
        }
    }

    #[test]
    fn test_valid_reservation() {
        let reservation = valid_reservation();
        let result = validate_reservation(&reservation);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_book_ref() {
        let mut reservation = valid_reservation();
        reservation.book_ref = "".to_string();
        let result = validate_reservation(&reservation);
        assert_eq!(result, Err("Book reference cannot be empty.".to_string()));
    }

    #[test]
    fn test_long_book_ref() {
        let mut reservation = valid_reservation();
        reservation.book_ref = "A".repeat(17); // 17 characters
        let result = validate_reservation(&reservation);
        assert_eq!(
            result,
            Err("Book reference cannot exceed 16 characters.".to_string())
        );
    }

    #[test]
    fn test_empty_email() {
        let mut reservation = valid_reservation();
        reservation.customer_email = "".to_string();
        let result = validate_reservation(&reservation);
        assert_eq!(result, Err("Customer email cannot be empty.".to_string()));
    }

    #[test]
    fn test_invalid_email() {
        let mut reservation = valid_reservation();
        reservation.customer_email = "invalid_email".to_string();
        let result = validate_reservation(&reservation);
        assert_eq!(result, Err("Customer email must contain '@'.".to_string()));
    }

    #[test]
    fn test_empty_customer_name() {
        let mut reservation = valid_reservation();
        reservation.customer_name = "".to_string();
        let result = validate_reservation(&reservation);
        assert_eq!(result, Err("Customer name cannot be empty.".to_string()));
    }

    #[test]
    fn test_long_customer_name() {
        let mut reservation = valid_reservation();
        reservation.customer_name = "A".repeat(257); // 257 characters
        let result = validate_reservation(&reservation);
        assert_eq!(
            result,
            Err("Customer name cannot exceed 256 characters.".to_string())
        );
    }

    #[test]
    fn test_empty_customer_phone() {
        let mut reservation = valid_reservation();
        reservation.customer_phone = "".to_string();
        let result = validate_reservation(&reservation);
        assert_eq!(
            result,
            Err("Customer phone cannot be empty or exceed 32 characters.".to_string())
        );
    }

    #[test]
    fn test_long_customer_phone() {
        let mut reservation = valid_reservation();
        reservation.customer_phone = "1".repeat(33); // 33 characters
        let result = validate_reservation(&reservation);
        assert_eq!(
            result,
            Err("Customer phone cannot be empty or exceed 32 characters.".to_string())
        );
    }

    #[test]
    fn test_invalid_customer_phone() {
        let mut reservation = valid_reservation();
        reservation.customer_phone = "invalidphone".to_string();
        let result = validate_reservation(&reservation);
        assert_eq!(
            result,
            Err("Customer phone must contain only digits or '+'.".to_string())
        );
    }

    #[test]
    fn test_invalid_table_size() {
        let mut reservation = valid_reservation();
        reservation.table_size = 0; // Invalid table size
        let result = validate_reservation(&reservation);
        assert_eq!(
            result,
            Err("Table size must be between 1 and 20.".to_string())
        );
    }

    #[test]
    fn test_large_table_size() {
        let mut reservation = valid_reservation();
        reservation.table_size = 21; // Invalid table size
        let result = validate_reservation(&reservation);
        assert_eq!(
            result,
            Err("Table size must be between 1 and 20.".to_string())
        );
    }

    #[test]
    fn test_past_reservation_time() {
        let mut reservation = valid_reservation();
        reservation.reservation_time = DateTime::from_timestamp(1_000_000_000, 0)
            .unwrap()
            .naive_utc(); // A date in the past
        let result = validate_reservation(&reservation);
        assert_eq!(
            result,
            Err("Reservation time cannot be in the past.".to_string())
        );
    }

    #[test]
    fn test_long_notes() {
        let mut reservation = valid_reservation();
        reservation.notes = Some("A".repeat(513)); // 513 characters
        let result = validate_reservation(&reservation);
        assert_eq!(
            result,
            Err("Notes cannot exceed 512 characters.".to_string())
        );
    }

    #[test]
    fn test_valid_notes() {
        let reservation = valid_reservation();
        let result = validate_reservation(&reservation);
        assert!(result.is_ok());
    }

    #[test]
    fn test_random_book_ref() {
        let reservation = Reservation::new(
            "customer@example.com",
            "John Doe",
            "123456789",
            4,
            Utc::now().naive_utc(),
            None,
        );
        assert_eq!(reservation.book_ref.len(), 5);
        assert_eq!(reservation.status, ReservationStatus::Pending);

        let result = validate_reservation(&reservation);
        assert!(result.is_ok());
    }
}
