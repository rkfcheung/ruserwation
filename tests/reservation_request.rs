#[cfg(test)]
mod tests {
    use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
    use ruserwation::reservation::{
        helper::{generate_ref_check, validate_ref_check},
        models::{Reservation, ReservationRequest},
    };

    #[test]
    fn test_reservation_request_new() {
        let customer_email = "test@example.com";
        let customer_name = "John Doe";
        let customer_phone = "1234567890";
        let table_size = 4;
        let reservation_date = NaiveDate::from_ymd_opt(2025, 1, 20).unwrap();
        let reservation_time = NaiveTime::from_hms_opt(18, 30, 0).unwrap();
        let reservation_datetime = NaiveDateTime::new(reservation_date, reservation_time);
        let notes = Some("Window seat, please".to_string());
        let ref_check = "valid_ref_check";

        let request = ReservationRequest::new(
            customer_email,
            customer_name,
            customer_phone,
            table_size,
            reservation_datetime,
            notes.clone(),
            ref_check,
        );

        // Validate fields
        assert_eq!(request.ref_check(), ref_check);
        let reservation: Reservation = request.into();
        assert_eq!(reservation.customer_email, customer_email);
        assert_eq!(reservation.customer_name, customer_name);
        assert_eq!(reservation.customer_phone, customer_phone);
        assert_eq!(reservation.table_size, table_size);
        assert_eq!(reservation.reservation_time, reservation_datetime);
        assert_eq!(reservation.notes, notes);
    }

    #[test]
    fn test_reservation_request_ref_check() {
        let customer_email = "test@example.com";
        let customer_name = "John Doe";
        let customer_phone = "1234567890";
        let table_size = 4;
        let reservation_time = Utc::now().naive_utc();
        let notes = None;
        let ref_check =
            generate_ref_check("test_ref_check", reservation_time.and_utc().timestamp()).unwrap();

        let request = ReservationRequest::new(
            customer_email,
            customer_name,
            customer_phone,
            table_size,
            reservation_time,
            notes,
            &ref_check,
        );

        // Validate ref_check method
        assert_eq!(request.ref_check(), ref_check);
    }

    #[test]
    fn test_reservation_request_with_no_notes() {
        let customer_email = "test@example.com";
        let customer_name = "John Doe";
        let customer_phone = "1234567890";
        let table_size = 4;
        let reservation_time = Utc::now().naive_utc();
        let notes = None;
        let ref_check = "test_ref_check";

        let request = ReservationRequest::new(
            customer_email,
            customer_name,
            customer_phone,
            table_size,
            reservation_time,
            notes,
            ref_check,
        );

        // Validate fields with None for notes
        assert_eq!(request.ref_check(), ref_check);
        let reservation: Reservation = request.into();
        assert_eq!(reservation.customer_email, customer_email);
        assert_eq!(reservation.customer_name, customer_name);
        assert_eq!(reservation.customer_phone, customer_phone);
        assert_eq!(reservation.table_size, table_size);
        assert_eq!(reservation.reservation_time, reservation_time);
        assert!(reservation.notes.is_none());
    }

    #[test]
    fn test_generate_ref_check_success() {
        let secret = "test_secret";
        let timestamp = Utc::now().timestamp();

        let ref_check = generate_ref_check(secret, timestamp);

        assert!(
            ref_check.is_ok(),
            "Expected ref_check to be generated successfully"
        );
        let ref_check = ref_check.unwrap();
        let parts: Vec<&str> = ref_check.split(':').collect();

        // Validate the format
        assert_eq!(
            parts.len(),
            2,
            "ref_check should have two parts separated by ':'"
        );

        // Validate the timestamp
        let parsed_timestamp: i64 = parts[0]
            .parse()
            .expect("Timestamp should be a valid integer");
        assert_eq!(
            parsed_timestamp, timestamp,
            "Timestamp should match the one provided"
        );

        // Validate the signature (not empty)
        assert!(!parts[1].is_empty(), "Signature should not be empty");
    }

    #[test]
    fn test_validate_ref_check_success() {
        let secret = "test_secret";
        let timestamp = Utc::now().timestamp();
        let ref_check =
            generate_ref_check(secret, timestamp).expect("Failed to generate ref_check");

        let validation_result = validate_ref_check(&ref_check, secret, 3600);

        assert!(
            validation_result.is_ok(),
            "Expected ref_check to be valid, got error: {:?}",
            validation_result.err()
        );
    }

    #[test]
    fn test_validate_ref_check_invalid_format() {
        let secret = "test_secret";
        let invalid_ref_check = "invalid_format";

        let validation_result = validate_ref_check(invalid_ref_check, secret, 3600);

        assert!(
            validation_result.is_err(),
            "Expected validation to fail for invalid format"
        );
        assert_eq!(
            validation_result.err().unwrap(),
            "Invalid ref_check format",
            "Expected specific error message for invalid format"
        );
    }

    #[test]
    fn test_validate_ref_check_expired() {
        let secret = "test_secret";
        let timestamp = (Utc::now() - Duration::hours(2)).timestamp(); // Simulate an expired timestamp
        let ref_check =
            generate_ref_check(secret, timestamp).expect("Failed to generate ref_check");

        let validation_result = validate_ref_check(&ref_check, secret, 3600);

        assert!(
            validation_result.is_err(),
            "Expected validation to fail for expired ref_check"
        );
        assert_eq!(
            validation_result.err().unwrap(),
            "ref_check expired",
            "Expected specific error message for expired ref_check"
        );
    }

    #[test]
    fn test_validate_ref_check_invalid_signature() {
        let secret = "test_secret";
        let timestamp = Utc::now().timestamp();
        let ref_check =
            generate_ref_check(secret, timestamp).expect("Failed to generate ref_check");

        // Tamper with the signature
        let mut parts: Vec<&str> = ref_check.split(':').collect();
        parts[1] = "invalid_signature";
        let tampered_ref_check = parts.join(":");

        let validation_result = validate_ref_check(&tampered_ref_check, secret, 3600);

        assert!(
            validation_result.is_err(),
            "Expected validation to fail for tampered signature"
        );
        assert_eq!(
            validation_result.err().unwrap(),
            "Invalid signature in ref_check",
            "Expected specific error message for tampered signature"
        );
    }
}
