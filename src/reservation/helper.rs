use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::Utc;
use hmac::{Hmac, Mac};
use rand::{distributions::Alphanumeric, Rng};

use super::models::Reservation;

type HmacSha256 = Hmac<sha2::Sha256>;

pub fn generate_random_book_ref(ref_len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(ref_len)
        .map(char::from)
        .collect()
}

pub fn generate_ref_check(secret: &str) -> Result<String, String> {
    let timestamp = Utc::now().timestamp();
    let payload = format!("{}:{}", secret, timestamp);

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| "Failed to create HMAC instance".to_string())?;
    mac.update(payload.as_bytes());

    let signature = mac.finalize().into_bytes();
    let encoded_signature = BASE64_STANDARD.encode(signature);

    Ok(format!("{}:{}", timestamp, encoded_signature))
}

pub fn validate_ref_check(ref_check: &str, secret: &str) -> Result<(), String> {
    let parts: Vec<&str> = ref_check.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid ref_check format".to_string());
    }

    let timestamp: i64 = parts[0]
        .parse()
        .map_err(|_| "Invalid timestamp in ref_check")?;
    let signature = BASE64_STANDARD
        .decode(parts[1])
        .map_err(|_| "Invalid signature in ref_check")?;

    // Ensure the timestamp is within the allowed range (1 hour)
    let current_time = Utc::now().timestamp();
    if (current_time - timestamp).abs() > 3600 {
        return Err("ref_check expired".to_string());
    }

    // Recreate the signature and validate
    let payload = format!("{}:{}", secret, timestamp);
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| "Failed to create HMAC instance")?;
    mac.update(payload.as_bytes());

    mac.verify_slice(&signature)
        .map_err(|_| "Invalid ref_check signature".to_string())
}

pub fn validate_reservation(reservation: &Reservation) -> Result<(), String> {
    if reservation.book_ref.trim().is_empty() {
        return Err("Book reference cannot be empty.".to_string());
    }

    if reservation.book_ref.len() > 16 {
        return Err("Book reference cannot exceed 16 characters.".to_string());
    }

    if reservation.customer_email.trim().is_empty() {
        return Err("Customer email cannot be empty.".to_string());
    }

    if !reservation.customer_email.contains('@') {
        return Err("Customer email must contain '@'.".to_string());
    }

    if reservation.customer_name.trim().is_empty() {
        return Err("Customer name cannot be empty.".to_string());
    }

    if reservation.customer_name.len() > 256 {
        return Err("Customer name cannot exceed 256 characters.".to_string());
    }

    if reservation.customer_phone.trim().is_empty() || reservation.customer_phone.len() > 32 {
        return Err("Customer phone cannot be empty or exceed 32 characters.".to_string());
    }

    if !reservation
        .customer_phone
        .chars()
        .all(|c| c.is_ascii_digit() || c == '+')
    {
        return Err("Customer phone must contain only digits or '+'.".to_string());
    }

    if reservation.table_size < 1 || reservation.table_size > 20 {
        return Err("Table size must be between 1 and 20.".to_string());
    }

    if reservation.reservation_time < Utc::now().naive_utc() - chrono::Duration::minutes(3) {
        return Err("Reservation time cannot be in the past.".to_string());
    }

    if let Some(notes) = &reservation.notes {
        if notes.len() > 512 {
            return Err("Notes cannot exceed 512 characters.".to_string());
        }
    }

    Ok(())
}
