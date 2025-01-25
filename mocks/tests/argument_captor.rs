#[cfg(test)]
mod tests {
    use mocks::*;

    #[test]
    fn test_argument_captor_first_last_values() {
        let mut captor = ArgumentCaptor::default();

        captor.capture(10);
        captor.capture(20);
        captor.capture(30);

        assert_eq!(captor.first(), Some(&10)); // First captured argument
        assert_eq!(captor.last(), Some(&30)); // Last captured argument
        assert_eq!(captor.values(), &vec![10, 20, 30]); // All captured arguments
    }

    #[test]
    fn test_capture_mut_values() {
        let mut captor = ArgumentCaptor::default();
        let values = vec!["hello", "world"];

        for &word in values.iter() {
            let mut mut_val = String::new();
            mut_val.push_str(word);
            captor.capture(ArgumentValue::new(mut_val));
        }

        let captured = captor.values();
        for i in 0..values.len() {
            assert_eq!(captured[i].get_unchecked::<String>(), values[i]);
        }
    }

    #[test]
    fn test_capture_str_values() {
        let mut captor = ArgumentCaptor::default();

        captor.capture("hello");
        captor.capture("world");
        assert_eq!(captor.first(), Some("hello").as_ref());
        assert_eq!(captor.last(), Some("world").as_ref());
    }
}
