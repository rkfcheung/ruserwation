#[cfg(test)]
mod tests {
    use mocks::*;

    #[test]
    fn test_argument_value_downcast() {
        let int_value = ArgumentValue::new(100);
        assert!(int_value.is::<i32>());
        assert_eq!(int_value.get::<i32>().unwrap(), &100);

        let string_value = ArgumentValue::new("test".to_string());
        assert!(string_value.is::<String>());
        assert_eq!(string_value.get::<String>().unwrap(), "test");
    }

    #[test]
    #[should_panic(
        expected = "Failed to downcast `ArgumentValue` to 'alloc::string::String'. Ensure the types match."
    )]
    fn test_argument_value_invalid_downcast() {
        let value = ArgumentValue::new(42 as usize); // Stores an `usize`
        assert!(value.is::<usize>());
        assert_eq!(*value.downcast_ref::<usize>().unwrap(), 42);
        assert_eq!(value.get::<usize>().unwrap(), &42);
        assert_eq!(value.get_unchecked::<usize>(), &42);
        assert!(!value.is::<String>());
        value.get_unchecked::<String>(); // Attempting to get `String` should panic
    }

    #[test]
    fn test_mock_default_behavior() {
        let default_value = ArgumentValue::default();

        // Ensure the default value is of type `MockDefault`
        assert!(default_value.get::<MockDefault>().is_some());
        assert!(default_value.is::<MockDefault>());
        assert!(default_value.get::<String>().is_none());
    }
}
