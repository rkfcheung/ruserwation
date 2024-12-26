use mocks::{ArgumentValue, InvocationTracker, MockVerify};

extern crate mock_derive;
use mock_derive::*;

// Mock Struct to Test
#[derive(Default, MockVerify)]
struct MockSessionManager {
    invocation: InvocationTracker,
}

struct MyMock {
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

    #[mock_captured_arguments]
    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    #[mock_captured_arguments]
    fn greet(&self, message: &str) {
        // No-op
        // println!("{}", message);
    }

    #[mock_captured_arguments]
    fn hi(&self) {
        // No-op
        // println!("Hi!");
    }
}

impl MyMock {
    #[mock_track("Increment")]
    pub fn increment_only(&self) {
        // No-op
    }

    #[mock_track("Capture")]
    pub fn capture_arguments(&self, arg1: i32, arg2: String) {
        // No-op
    }

    #[mock_track("Increment, Capture")]
    pub fn increment_and_capture(&self, arg1: i32) {
        // No-op
    }

    #[mock_track]
    pub fn imply_increment_only(&self) {
        // No-op
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

    #[test]
    fn test_argument_captured_arguments() {
        let mock = MockSessionManager::default();

        // Test add function
        mock.add(1, 2);
        assert_eq!(mock.invocation.values("add").len(), 1);
        let captured_values = mock.invocation.first("add").unwrap();
        assert_eq!(*captured_values.get::<(i32, i32)>().unwrap(), (1, 2));

        // Test greet function
        mock.greet("hello");
        assert_eq!(mock.invocation.values("greet").len(), 1);
        let captured_values = mock.invocation.first("greet").unwrap();
        assert_eq!(captured_values.get::<String>().unwrap(), "hello");

        // Test hi function
        mock.hi();
        assert_eq!(mock.invocation.values("hi").len(), 0); // Should ignore `hi` because it has no arguments
    }

    #[test]
    fn test_increment_only() {
        let mock = MyMock {
            invocation: InvocationTracker::default(),
        };

        mock.increment_only();
        mock.increment_only();

        assert_eq!(mock.invocation.get("increment_only"), 2); // Invoked twice
    }

    #[test]
    fn test_capture_arguments() {
        let mock = MyMock {
            invocation: InvocationTracker::default(),
        };

        mock.capture_arguments(42, "Hello".to_string());

        let first_arg = mock.invocation.first("capture_arguments").unwrap();
        assert_eq!(
            first_arg.unwrap::<(i32, String)>(),
            &(42, "Hello".to_string())
        );
    }

    #[test]
    fn test_increment_and_capture() {
        let mock = MyMock {
            invocation: InvocationTracker::default(),
        };

        mock.increment_and_capture(10);

        assert_eq!(mock.invocation.get("increment_and_capture"), 1); // Invoked once

        let first_arg = mock.invocation.first("increment_and_capture").unwrap();
        assert_eq!(first_arg.unwrap::<i32>(), &10);
    }

    #[test]
    fn test_imply_increment_only() {
        let mock = MyMock {
            invocation: InvocationTracker::default(),
        };

        // Call the method multiple times
        mock.imply_increment_only();
        mock.imply_increment_only();
        mock.imply_increment_only();

        // Verify the invocation count
        assert_eq!(mock.invocation.get("imply_increment_only"), 3); // Invoked three times
    }
}
