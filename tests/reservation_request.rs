#[cfg(test)]
mod tests {
    use chrono::Utc;
    use std::thread::sleep;
    use std::time::Duration;

    use ruserwation::reservation::helper::{generate_ref_check, validate_ref_check};

    #[test]
    fn test_generate_ref_check_success() {
        let secret = "test_secret";
        let ref_check = generate_ref_check(secret);

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
        let timestamp: i64 = parts[0]
            .parse()
            .expect("Timestamp should be a valid integer");
        let current_time = Utc::now().timestamp();
        assert!(
            (current_time - timestamp).abs() < 5,
            "Timestamp should be close to the current time"
        );

        // Validate the signature (not empty)
        assert!(!parts[1].is_empty(), "Signature should not be empty");
    }

    #[test]
    fn test_validate_ref_check_success() {
        let secret = "test_secret";
        let ref_check = generate_ref_check(secret).expect("Failed to generate ref_check");

        let validation_result = validate_ref_check(&ref_check, secret);

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

        let validation_result = validate_ref_check(invalid_ref_check, secret);

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
        let ref_check = generate_ref_check(secret).expect("Failed to generate ref_check");

        // Simulate expiration (wait for over an hour)
        sleep(Duration::from_secs(3601));

        let validation_result = validate_ref_check(&ref_check, secret);

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
        let ref_check = generate_ref_check(secret).expect("Failed to generate ref_check");

        // Tamper with the signature
        let mut parts: Vec<&str> = ref_check.split(':').collect();
        parts[1] = "invalid_signature";
        let tampered_ref_check = parts.join(":");

        let validation_result = validate_ref_check(&tampered_ref_check, secret);

        assert!(
            validation_result.is_err(),
            "Expected validation to fail for tampered signature"
        );
        assert_eq!(
            validation_result.err().unwrap(),
            "Invalid ref_check signature",
            "Expected specific error message for tampered signature"
        );
    }
}
