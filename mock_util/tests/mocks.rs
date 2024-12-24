use mocks::{ArgumentValue, InvocationTracker, MockVerify};

extern crate mock_util;
use mock_util::mock_invoked;

// Mock Struct to Test
#[derive(Default, mock_util::MockVerify)]
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

    #[test]
    fn test_mock_invoked_increment() {
        let manager = MockSessionManager::default();

        // Invoke the method
        manager.destroy_session("session1");
        manager.destroy_session("session2");

        // Verify the invocation count
        assert_eq!(manager.invocation.get("destroy_session"), 2);
    }

    #[test]
    fn test_mock_verify_invoked() {
        let manager = MockSessionManager::default();

        // Invoke the method
        manager.destroy_session("session1");

        // Verify the invocation count using `verify_once`
        manager.verify_once("destroy_session");

        // Invoke the method again
        manager.destroy_session("session2");

        // Verify the invocation count using `verify_invoked`
        manager.verify_invoked("destroy_session", 2);
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
        assert_eq!(
            map_to_strings(captured_args),
            vec!["session1".to_string(), "session2".to_string()]
        );
    }

    #[test]
    fn test_multiple_methods() {
        let manager = MockSessionManager::default();

        // Invoke different methods
        manager.destroy_session("session1");
        manager.add_session("session2");

        // Verify invocation counts
        assert_eq!(manager.invocation.get("destroy_session"), 1);
        assert_eq!(manager.invocation.get("add_session"), 1);

        // Verify captured arguments
        let destroy_arg = manager
            .invocation
            .first::<String>("destroy_session")
            .unwrap();
        let destroy_arg = destroy_arg.get::<String>().unwrap();
        let add_arg = manager.invocation.last::<String>("add_session").unwrap();
        let add_arg = add_arg.get::<String>().unwrap();

        assert_eq!(destroy_arg, "session1");
        assert_eq!(add_arg, "session2");
    }

    fn map_to_strings(values: Vec<ArgumentValue>) -> Vec<String> {
        values
            .into_iter()
            .filter_map(|value| value.get::<String>().cloned())
            .collect()
    }
}
