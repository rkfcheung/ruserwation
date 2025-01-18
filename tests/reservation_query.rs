#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use ruserwation::{db::QueryError, reservation::models::ReservationQuery};

    // Helper function to create a NaiveDateTime
    fn create_naive_datetime(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    ) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, 0)
            .unwrap()
    }

    #[test]
    fn test_create_query_with_no_conditions() {
        let query = ReservationQuery::default(); // No filters applied

        // Call the create method and expect an error
        let result = query.create();
        assert_eq!(result, Err(QueryError::NoConditionsProvided));
    }

    #[test]
    fn test_create_query_with_id_filter() {
        let query = ReservationQuery::default().id(1); // Apply id filter

        // Generate the SQL query and arguments
        let (sql, args) = query.create().expect("Expected a valid query");

        // Check the generated SQL query and arguments
        assert_eq!(sql, "SELECT * FROM Reservation WHERE id = ?");
        assert_eq!(args.len(), 1); // One argument for the id filter
    }

    #[test]
    fn test_create_query_with_multiple_filters() {
        let query = ReservationQuery::default()
            .id(1)
            .book_ref("ABC123")
            .customer_email("test@example.com");

        // Generate the SQL query and arguments
        let (sql, args) = query.create().expect("Expected a valid query");

        // Check the generated SQL query and arguments
        assert_eq!(
            sql,
            "SELECT * FROM Reservation WHERE id = ? AND book_ref = ? AND customer_email = ?"
        );
        assert_eq!(args.len(), 3); // Three arguments: id, book_ref, customer_email
    }

    #[test]
    fn test_create_query_with_datetime_filters() {
        let from_time = create_naive_datetime(2025, 1, 11, 10, 30);
        let to_time = create_naive_datetime(2025, 1, 11, 12, 0);

        let query = ReservationQuery::default()
            .from_time(from_time)
            .to_time(to_time);

        // Generate the SQL query and arguments
        let (sql, args) = query.create().expect("Expected a valid query");

        // Check the generated SQL query and arguments
        assert_eq!(
            sql,
            "SELECT * FROM Reservation WHERE reservation_time >= ? AND reservation_time <= ?"
        );
        assert_eq!(args.len(), 2); // Two arguments: from_time, to_time
    }

    #[test]
    fn test_create_query_with_status_filter() {
        let query = ReservationQuery::default().status(ReservationStatus::Confirmed);

        // Generate the SQL query and arguments
        let (sql, args) = query.create().expect("Expected a valid query");

        // Check the generated SQL query and arguments
        assert_eq!(sql, "SELECT * FROM Reservation WHERE status = ?");
        assert_eq!(args.len(), 1); // One argument for the status
    }

    #[test]
    fn test_create_query_with_all_filters() {
        let from_time = create_naive_datetime(2025, 1, 11, 10, 30);
        let to_time = create_naive_datetime(2025, 1, 11, 12, 0);

        let query = ReservationQuery::default()
            .id(1)
            .book_ref("ABC123")
            .customer_email("test@example.com")
            .customer_name("John Doe")
            .customer_phone("1234567890")
            .from_time(from_time)
            .to_time(to_time)
            .status(ReservationStatus::Confirmed);

        // Generate the SQL query and arguments
        let (sql, args) = query.create().expect("Expected a valid query");

        // Check the generated SQL query and arguments
        assert_eq!(
            sql,
            "SELECT * FROM Reservation WHERE id = ? AND book_ref = ? AND customer_email = ? AND customer_name = ? AND customer_phone = ? AND reservation_time >= ? AND reservation_time <= ? AND status = ?"
        );
        assert_eq!(args.len(), 8); // Eight arguments: all the filters applied
    }
}
