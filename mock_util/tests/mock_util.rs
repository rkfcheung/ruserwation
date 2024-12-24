use mocks::{ArgumentValue, InvocationTracker, MockVerify};

extern crate mock_util;
use mock_util::*;

// Mock Struct to Test
#[derive(Default, MockVerify)]
struct MockSessionManager {
    invocation: InvocationTracker,
}

impl MockSessionManager {
    #[mock_invoked]
    fn destroy_session(&self, session_id: &str) {
        self.invocation
            .capture("destroy_session", session_id.to_string());
    }

    #[mock_invoked]
    fn add_session(&self, session_id: &str) {
        self.invocation
            .capture("add_session", session_id.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to map captured arguments to Strings
    fn map_to_strings(values: Vec<ArgumentValue>) -> Vec<String> {
        values
            .into_iter()
            .filter_map(|value| value.get::<String>().cloned())
            .collect()
    }

    #[test]
    fn test_mock_invoked_increment() {
        let manager = MockSessionManager::default();

        // Invoke the method
        manager.destroy_session("session1");
        manager.destroy_session("session2");

        // Verify the invocation count
        assert_eq!(
            manager.invocation.get("destroy_session"),
            2,
            "Expected destroy_session to be invoked twice"
        );
    }

    #[test]
    fn test_mock_verify_invoked() {
        let manager = MockSessionManager::default();

        // Invoke the method
        manager.destroy_session("session1");

        // Verify the invocation count using `verify_exactly_once`
        manager.verify_exactly_once("destroy_session");

        // Invoke the method again
        manager.destroy_session("session2");

        // Verify the invocation count using `verify_exactly`
        manager.verify_exactly("destroy_session", 2);
    }

    #[test]
    fn test_argument_capture() {
        let manager = MockSessionManager::default();

        // Capture arguments
        manager.destroy_session("session1");
        manager.destroy_session("session2");

        // Retrieve captured arguments
        let captured_args = manager.invocation.values("destroy_session");

        // Verify captured arguments
        let captured_strings = map_to_strings(captured_args);
        assert_eq!(
            captured_strings,
            vec!["session1".to_string(), "session2".to_string()],
            "Expected captured arguments to match the session ids"
        );
    }

    #[test]
    fn test_multiple_methods() {
        let manager = MockSessionManager::default();

        // Invoke different methods
        manager.destroy_session("session1");
        manager.add_session("session2");

        // Verify invocation counts
        assert_eq!(
            manager.invocation.get("destroy_session"),
            1,
            "Expected destroy_session to be called once"
        );
        assert_eq!(
            manager.invocation.get("add_session"),
            1,
            "Expected add_session to be called once"
        );

        // Verify captured arguments
        let destroy_arg = manager.invocation.first("destroy_session").unwrap();
        let destroy_arg = destroy_arg.get::<String>().unwrap();
        let add_arg = manager.invocation.last("add_session").unwrap();
        let add_arg = add_arg.get::<String>().unwrap();

        assert_eq!(
            destroy_arg, "session1",
            "Expected first destroy_session argument to be 'session1'"
        );
        assert_eq!(
            add_arg, "session2",
            "Expected add_session argument to be 'session2'"
        );
    }

    #[test]
    fn test_zero_invocations() {
        let manager = MockSessionManager::default();

        // Verify no invocations
        assert_eq!(
            manager.invocation.get("destroy_session"),
            0,
            "Expected no calls to destroy_session"
        );

        manager.verify_exactly("destroy_session", 0);
    }

    #[test]
    #[should_panic(
        expected = "Expected method 'destroy_session' to be called Eq 2 times, but it was called 1 times."
    )]
    fn test_invalid_invocation_count() {
        let manager = MockSessionManager::default();

        // Test an invalid invocation count for a method
        manager.destroy_session("session1");

        // Verify the method was invoked exactly once
        assert_eq!(
            manager.invocation.get("destroy_session"),
            1,
            "Expected destroy_session to be invoked once"
        );

        // This should panic as we verify it was invoked twice
        manager.verify_exactly("destroy_session", 2);
    }

    #[test]
    fn test_edge_case_empty_session_id() {
        let manager = MockSessionManager::default();

        // Invoke with empty session_id
        manager.destroy_session("");

        // Verify invocation and captured argument
        assert_eq!(manager.invocation.get("destroy_session"), 1);
        let captured_args = manager.invocation.values("destroy_session");
        let captured_strings = map_to_strings(captured_args);
        assert_eq!(captured_strings, vec!["".to_string()]);
    }

    #[test]
    fn test_multiple_invocations_same_method() {
        let manager = MockSessionManager::default();

        // Multiple invocations with the same method
        manager.add_session("session1");
        manager.add_session("session2");

        // Verify invocation counts
        assert_eq!(manager.invocation.get("add_session"), 2);

        // Verify captured arguments
        let captured_args = manager.invocation.values("add_session");
        let captured_strings = map_to_strings(captured_args);
        assert_eq!(
            captured_strings,
            vec!["session1".to_string(), "session2".to_string()]
        );
    }
}
