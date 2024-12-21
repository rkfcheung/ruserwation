#[cfg(test)]
mod tests {
    use ruserwation::admin::auth::get_cookie_session_id;

    #[test]
    fn test_get_cookie_session_id() {
        // Case 1: Valid session cookie with only session_id
        let cookie = Some(
            "session_id=1qmira+uAwsUw53T6i5Itn9bTB0ST1ObU4yrClGbuw9WuMII/uRZsi2EqjQ/".to_string(),
        );
        let session_id = get_cookie_session_id(cookie);
        assert_eq!(
            session_id,
            Some("1qmira+uAwsUw53T6i5Itn9bTB0ST1ObU4yrClGbuw9WuMII/uRZsi2EqjQ/".to_string())
        );

        // Case 2: Cookie with multiple keys, including session_id
        let cookie = Some("Idea-90fc531a=e4ae811b-13c6-443c-ba0d-a2d9795d5302; session_id=1qmira+uAwsUw53T6i5Itn9bTB0ST1ObU4yrClGbuw9WuMII/uRZsi2EqjQ/".to_string());
        let session_id = get_cookie_session_id(cookie);
        assert_eq!(
            session_id,
            Some("1qmira+uAwsUw53T6i5Itn9bTB0ST1ObU4yrClGbuw9WuMII/uRZsi2EqjQ/".to_string())
        );

        // Case 3: Cookie without session_id
        let cookie = Some("Idea-90fc531a=e4ae811b-13c6-443c-ba0d-a2d9795d5302".to_string());
        let session_id = get_cookie_session_id(cookie);
        assert_eq!(session_id, None);

        // Case 4: Invalid cookie format (no '=')
        let cookie = Some("session_id".to_string());
        let session_id = get_cookie_session_id(cookie);
        assert_eq!(session_id, None);

        // Case 5: Empty cookie
        let cookie = Some("".to_string());
        let session_id = get_cookie_session_id(cookie);
        assert_eq!(session_id, None);

        // Case 6: None as the input
        let cookie: Option<String> = None;
        let session_id = get_cookie_session_id(cookie);
        assert_eq!(session_id, None);
    }
}
