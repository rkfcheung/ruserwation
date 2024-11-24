#[cfg(test)]
mod tests {

    use ruserwation::{
        admin::{helper, models::Admin},
        utils::env_util::var_as_int_or,
    };
    use std::env;

    // Test the Admin::new method
    #[test]
    fn test_admin_new() {
        let username = "admin";
        let password = "password123";
        let email = "admin@localhost";

        let admin = Admin::new(
            1,
            username.to_string(),
            password.to_string(),
            email.to_string(),
        );

        assert_eq!(admin.id, 1);
        assert_eq!(admin.username, username);
        assert_eq!(admin.email, email);
        assert!(admin.root); // Since id is 1, root should be true
        assert!(admin.last_login_time.is_none());

        // Should verify the random password
        assert!(admin.verify_password(&password));
    }

    // Test the random password generation
    #[test]
    fn test_generate_random_password() {
        let length = 16;
        let password = helper::generate_random_password(length);
        assert_eq!(password.len(), length);

        let length = 8;
        let password = helper::generate_random_password(length);
        assert_eq!(password.len(), length);
    }

    // Test for edge cases in random password generation
    #[test]
    fn test_generate_random_password_zero_length() {
        let password = helper::generate_random_password(0);
        assert_eq!(password.len(), 8);
    }

    // Test the random password generation with a default password length
    #[test]
    fn test_generate_password_with_env_length() {
        env::set_var("RW_ADMIN_PWD_LEN", "10");

        let pwd_len = var_as_int_or("RW_ADMIN_PWD_LEN", 0) as usize;
        let random_password = helper::generate_random_password(pwd_len);

        assert_eq!(random_password.len(), 10);

        // Clean up environment variables after the test
        env::remove_var("RW_ADMIN_PWD_LEN");
    }

    // Test the init method with environment variables
    #[test]
    fn test_admin_init_with_env_vars() {
        // Set environment variables for testing
        env::set_var("RW_ADMIN_USERNAME", "env_admin");
        env::set_var("RW_ADMIN_EMAIL", "env_admin@localhost");
        env::set_var("RW_ADMIN_PASSWORD", "env_password123");

        let admin = Admin::init();

        assert_eq!(admin.username, "env_admin");
        assert_eq!(admin.email, "env_admin@localhost");
        assert_eq!(admin.root, true); // Should be root because id is 1

        // Clean up environment variables after the test
        env::remove_var("RW_ADMIN_USERNAME");
        env::remove_var("RW_ADMIN_EMAIL");
        env::remove_var("RW_ADMIN_PASSWORD");
    }

    // Test the init method when password is not set (random password generation)
    #[test]
    fn test_admin_init_with_random_password() {
        // Ensure no password is set in the environment
        env::remove_var("RW_ADMIN_USERNAME");
        env::remove_var("RW_ADMIN_EMAIL");
        env::remove_var("RW_ADMIN_PASSWORD");

        // Test admin initialisation with default/random password
        let admin = Admin::init();

        assert_eq!(admin.username, "admin");
        assert_eq!(admin.email, "admin@localhost");
        assert_eq!(admin.root, true); // Should be root because id is 1
        assert!(admin.password.len() > 0); // Random password is set
    }

    // Test the password verification with valid and invalid passwords
    #[test]
    fn test_verify_password() {
        let username = "admin".to_string();
        let password = "password123";
        let email = "admin@localhost".to_string();
        let admin = Admin::new(1, username, password.to_string(), email);

        // Verify with the correct password
        assert!(admin.verify_password(&password));

        // Verify with an incorrect password
        assert!(!admin.verify_password("wrongpassword"));
    }
}
