#[cfg(test)]
mod tests {
    use mocks::*;

    #[test]
    fn test_increment_invocation_count() {
        let tracker = InvocationTracker::default();

        tracker.increment("method1");
        tracker.increment("method1");
        tracker.increment("method2");

        assert_eq!(tracker.get("method1"), 2); // method1 called twice
        assert_eq!(tracker.get("method2"), 1); // method2 called once
        assert_eq!(tracker.get("method3"), 0); // method3 never called
    }

    #[test]
    fn test_capture_arguments() {
        let tracker = InvocationTracker::default();

        tracker.capture("method1", 42);
        tracker.capture("method1", 24);

        let first = tracker.first("method1").unwrap();
        let last = tracker.last("method1").unwrap();

        assert_eq!(first.get::<i32>().unwrap(), &42); // First captured argument
        assert_eq!(last.get::<i32>().unwrap(), &24); // Last captured argument
    }

    #[test]
    fn test_capture_string_arguments() {
        let tracker = InvocationTracker::default();

        tracker.capture("method2", "hello".to_string());
        tracker.capture("method2", "world".to_string());

        let first = tracker.first("method2").unwrap();
        let last = tracker.last("method2").unwrap();

        assert_eq!(first.get::<String>().unwrap(), "hello"); // First captured argument
        assert_eq!(last.get::<String>().unwrap(), "world"); // Last captured argument
    }

    #[test]
    fn test_argument_captor_first_last_values() {
        let captor = ArgumentCaptor::default();

        captor.capture(10);
        captor.capture(20);
        captor.capture(30);

        assert_eq!(captor.first(), Some(10)); // First captured argument
        assert_eq!(captor.last(), Some(30)); // Last captured argument
        assert_eq!(captor.values(), vec![10, 20, 30]); // All captured arguments
    }

    #[test]
    fn test_argument_value_downcast() {
        let value = ArgumentValue::new(100);
        assert_eq!(value.get::<i32>().unwrap(), &100);

        let string_value = ArgumentValue::new("test".to_string());
        assert_eq!(string_value.get::<String>().unwrap(), "test");
    }

    #[test]
    fn test_mock_default_behavior() {
        let default_value = ArgumentValue::default();
        assert!(default_value.get::<MockDefault>().is_some());
    }

    #[test]
    fn test_thread_safety_with_send_sync() {
        use std::sync::Arc;

        let tracker = Arc::new(InvocationTracker::default());
        let tracker_clone = tracker.clone();

        let handle = std::thread::spawn(move || {
            tracker_clone.increment("method_in_thread");
        });

        handle.join().unwrap();
        assert_eq!(tracker.get("method_in_thread"), 1);
    }

    #[test]
    fn test_verify_invoked() {
        let tracker = InvocationTracker::default();

        // Simulate calls to methods
        tracker.increment("method1");
        tracker.increment("method1");
        tracker.increment("method2");

        // Verify exact count
        assert!(tracker.verify_invoked("method1", MockCheck::Eq, 2).passed);
        assert!(!tracker.verify_invoked("method1", MockCheck::Eq, 1).passed);

        // Verify at least
        assert!(tracker.verify_invoked("method1", MockCheck::Gte, 1).passed);
        assert!(!tracker.verify_invoked("method1", MockCheck::Gte, 3).passed);

        // Verify at most
        assert!(tracker.verify_invoked("method1", MockCheck::Lte, 2).passed);
        assert!(!tracker.verify_invoked("method1", MockCheck::Lte, 1).passed);

        // Verify a method never called
        let answer = tracker.verify_invoked("method3", MockCheck::Eq, 0);
        assert!(answer.passed);
        assert_eq!(
            answer.reason,
            "Verification passed: method 'method3' was called 0 times as expected."
        );
    }

    #[test]
    fn test_capture_no_arguments() {
        let tracker = InvocationTracker::default();

        assert!(tracker.first("method").is_none()); // No arguments captured
        assert!(tracker.last("method").is_none()); // No arguments captured
        assert!(tracker.values("method").is_empty()); // No arguments captured
    }

    #[test]
    #[should_panic(
        expected = "Failed to downcast `ArgumentValue` to 'alloc::string::String'. Ensure the types match."
    )]
    fn test_argument_value_invalid_downcast() {
        let value = ArgumentValue::new(42 as usize); // Stores an `usize`
        assert_eq!(*value.downcast_ref::<usize>().unwrap(), 42);
        assert_eq!(value.get::<usize>().unwrap(), &42);
        assert_eq!(value.unwrap::<usize>(), &42);
        value.unwrap::<String>(); // Attempting to get `String` should panic
    }

    #[test]
    fn test_verify_invoked_edge_cases() {
        let tracker = InvocationTracker::default();

        tracker.increment("method1");

        // Verify exactly once
        assert!(tracker.verify_invoked("method1", MockCheck::Eq, 1).passed);

        // Verify at least once
        assert!(tracker.verify_invoked("method1", MockCheck::Gte, 1).passed);

        // Verify at most once
        assert!(tracker.verify_invoked("method1", MockCheck::Lte, 1).passed);

        // Verify at least twice (should fail)
        assert!(!tracker.verify_invoked("method1", MockCheck::Gte, 2).passed);

        // Verify at most zero times (should fail)
        assert!(!tracker.verify_invoked("method1", MockCheck::Lte, 0).passed);

        // Verify a method never called with non-zero times
        assert!(!tracker.verify_invoked("method2", MockCheck::Eq, 1).passed);
    }

    #[test]
    fn test_multiple_methods_capturing() {
        let tracker = InvocationTracker::default();

        // Capture arguments for multiple methods
        tracker.capture("method1", 42);
        tracker.capture("method2", "hello".to_string());

        // Verify capturing for method1
        assert_eq!(tracker.first("method1").unwrap().get::<i32>().unwrap(), &42);
        assert!(tracker.first("method2").unwrap().get::<String>().is_some());
    }

    #[test]
    fn test_verify_never_called() {
        let tracker = InvocationTracker::default();

        let answer = tracker.verify_invoked("method1", MockCheck::Eq, 0);
        assert!(answer.passed);
        assert_eq!(
            answer.reason,
            "Verification passed: method 'method1' was called 0 times as expected."
        );
    }

    #[test]
    fn test_concurrent_captures_simulated() {
        let tracker = InvocationTracker::default();

        tracker.capture("method1", 1);
        tracker.capture("method1", 2);

        // Simulate another context capturing for a different method
        tracker.capture("method2", "value".to_string());

        assert_eq!(tracker.values("method1").len(), 2);
        assert_eq!(tracker.values("method2").len(), 1);
    }

    #[test]
    fn test_mock_default_integration() {
        let default_value = ArgumentValue::default();

        // Ensure the default value is of type `MockDefault`
        assert!(default_value.get::<MockDefault>().is_some());
        assert!(default_value.get::<String>().is_none());
    }
}
