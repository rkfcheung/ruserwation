use std::collections::HashMap;
use std::sync::Mutex;

extern crate test_utils;
use test_utils::{mock_invoked, MockVerify};

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

// InvocationTracker (copied from your existing implementation)
#[derive(Default)]
pub struct InvocationTracker {
    invoked_count: Mutex<HashMap<String, usize>>,
    captors: Mutex<HashMap<String, Vec<String>>>,
}

impl InvocationTracker {
    pub fn increment(&self, method: &str) {
        let mut invoked_count = self.invoked_count.lock().unwrap();
        *invoked_count.entry(method.to_string()).or_insert(0) += 1;
    }

    pub fn get(&self, method: &str) -> usize {
        *self.invoked_count.lock().unwrap().get(method).unwrap_or(&0)
    }

    pub fn capture(&self, method: &str, argument: String) {
        let mut captors = self.captors.lock().unwrap();
        captors
            .entry(method.to_string())
            .or_insert_with(Vec::new)
            .push(argument);
    }

    pub fn get_captured_arguments(&self, method: &str) -> Vec<String> {
        self.captors
            .lock()
            .unwrap()
            .get(method)
            .cloned()
            .unwrap_or_default()
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
        manager.destroy_session("session2");

        // Verify the invocation count using `verify_invoked`
        manager.verify_invoked("destroy_session", 2);

        // Negative test case (should panic if uncommented)
        // manager.verify_invoked("destroy_session", 1);
    }

    #[test]
    fn test_argument_capture() {
        let manager = MockSessionManager::default();

        // Capture arguments
        manager.destroy_session("session1");
        manager.destroy_session("session2");

        // Retrieve captured arguments
        let captured_args = manager.invocation.get_captured_arguments("destroy_session");

        // Verify captured arguments
        assert_eq!(
            captured_args,
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
        let destroy_args = manager.invocation.get_captured_arguments("destroy_session");
        let add_args = manager.invocation.get_captured_arguments("add_session");

        assert_eq!(destroy_args, vec!["session1".to_string()]);
        assert_eq!(add_args, vec!["session2".to_string()]);
    }
}
