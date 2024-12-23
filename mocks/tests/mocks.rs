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

        let first = tracker.first::<i32>("method1").unwrap();
        let last = tracker.last::<i32>("method1").unwrap();

        assert_eq!(first.get::<i32>().unwrap(), &42); // First captured argument
        assert_eq!(last.get::<i32>().unwrap(), &24); // Last captured argument
    }

    #[test]
    fn test_capture_string_arguments() {
        let tracker = InvocationTracker::default();

        tracker.capture("method2", "hello".to_string());
        tracker.capture("method2", "world".to_string());

        let first = tracker.first::<String>("method2").unwrap();
        let last = tracker.last::<String>("method2").unwrap();

        assert_eq!(first.get::<String>().unwrap(), "hello"); // First captured argument
        assert_eq!(last.get::<String>().unwrap(), "world"); // Last captured argument
    }

    #[test]
    fn test_argument_captor_first_last() {
        let captor = ArgumentCaptor::default();

        captor.capture(10);
        captor.capture(20);
        captor.capture(30);

        assert_eq!(captor.first(), Some(10)); // First captured argument
        assert_eq!(captor.last(), Some(30)); // Last captured argument
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
}
