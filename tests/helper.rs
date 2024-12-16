#[cfg(test)]
mod tests {
    use ruserwation::admin::helper::{
        generate_random_password, hash_password, validate_email, validate_password,
        validate_username,
    };

    // Test for hash_password function
    #[test]
    fn test_hash_password() {
        let password = "testpassword";
        let hashed_password = hash_password(password);

        assert!(
            !hashed_password.is_empty(),
            "Hashed password should not be empty."
        );
        assert_ne!(
            password.as_bytes(),
            hashed_password,
            "Password and hashed password should be different."
        );
    }

    // Test for generate_random_password function
    #[test]
    fn test_generate_random_password() {
        // Test with a valid length
        let password = generate_random_password(16);
        assert_eq!(
            password.len(),
            16,
            "Generated password length should be 16."
        );

        // Test with a length smaller than the minimum
        let password = generate_random_password(4);
        assert_eq!(
            password.len(),
            8,
            "Generated password length should be at least 8."
        );

        // Test with a length larger than the maximum
        let password = generate_random_password(40);
        assert_eq!(
            password.len(),
            32,
            "Generated password length should be at most 32."
        );
    }

    // Test for validate_email function
    #[test]
    fn test_validate_email() {
        // Valid email
        let valid_email = "user@example.com";
        let result = validate_email(valid_email);
        assert_eq!(
            result, valid_email,
            "Valid email should return the same email."
        );

        // Invalid email
        let invalid_email = "not_an_email";
        let result = validate_email(invalid_email);
        assert_eq!(
            result, "admin@localhost",
            "Invalid email should return the default email."
        );

        // Special case: admin@localhost
        let default_email = "admin@localhost";
        let result = validate_email(default_email);
        assert_eq!(
            result, default_email,
            "admin@localhost should be treated as valid."
        );
    }

    // Test for validate_password function
    #[test]
    fn test_validate_password() {
        // Valid password
        let valid_password = "validpassword";
        let result = validate_password(valid_password);
        assert_eq!(
            result, valid_password,
            "Valid password should return the same password."
        );

        // Invalid password (too short)
        let short_password = "short";
        let result = validate_password(short_password);
        assert!(
            result.len() >= 8,
            "Generated password length for invalid password should be at least 8."
        );

        // Check if random password was generated
        let random_password = validate_password(short_password);
        assert!(
            random_password.len() >= 8,
            "Random password length should be at least 8."
        );

        // Invalid password (SQL Injection)
        let result = validate_password("' OR '1'='1");
        assert!(
            result.len() >= 8,
            "Generated password length for SQL Injection password should be at least 8."
        );
    }

    // Test for validate_username function
    #[test]
    fn test_validate_username() {
        // Valid username
        let valid_username = "valid_user";
        let result = validate_username(valid_username);
        assert_eq!(
            result, valid_username,
            "Valid username should return the same username."
        );

        // Invalid username (too short)
        let short_username = "us";
        let result = validate_username(short_username);
        assert_eq!(
            result, "admin",
            "Invalid username should return the default username."
        );

        // Invalid username (contains special characters)
        let invalid_username = "invalid@user";
        let result = validate_username(invalid_username);
        assert_eq!(
            result, "admin",
            "Invalid username should return the default username."
        );
    }

    // Test for is_valid_email (private function)
    #[test]
    fn test_is_valid_email() {
        // Valid email
        let valid_email = "user@example.com";
        assert_eq!(
            validate_email(valid_email),
            valid_email,
            "Valid email should return true."
        );

        // Invalid email
        let invalid_email = "not_an_email";
        assert_eq!(
            validate_email(invalid_email),
            "admin@localhost",
            "Invalid email should return false."
        );
    }

    // Test for is_valid_username (private function)
    #[test]
    fn test_is_valid_username() {
        // Valid username
        let valid_username = "valid_user";
        assert_eq!(
            validate_username(valid_username),
            valid_username,
            "Valid username should return true."
        );

        // Invalid username (too short)
        let short_username = "us";
        assert_eq!(
            validate_username(short_username),
            "admin",
            "Username too short should return false."
        );

        // Invalid username (contains special characters)
        let invalid_username = "invalid@user";
        assert_eq!(
            validate_username(invalid_username),
            "admin",
            "Username with special characters should return false."
        );

        // Invalid username (SQL Injection)
        let invalid_username = "'; DROP TABLE Admin; --";
        assert_eq!(
            validate_username(invalid_username),
            "admin",
            "Username with SQL Injection should return false."
        );
    }
}
