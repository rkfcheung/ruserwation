#[cfg(test)]
mod tests {

    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };
    use ruserwation::admin::models::Admin;
    use std::str::from_utf8;

    #[test]
    fn should_verify_random_password() {
        let rand_pwd: String = Admin::generate_random_password(16);
        assert!(rand_pwd.len() == 16);

        let admin = Admin::new(
            1,
            "admin".to_string(),
            rand_pwd.clone(),
            "admin@localhost".to_string(),
        );
        let parsed_hash = PasswordHash::new(from_utf8(&admin.password).unwrap()).unwrap();
        assert!(Argon2::default()
            .verify_password(rand_pwd.as_bytes(), &parsed_hash)
            .is_ok());
    }
}
