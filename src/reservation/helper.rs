use chrono::Utc;

use super::models::Reservation;

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
        .all(|c| c.is_digit(10) || c == '+')
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
