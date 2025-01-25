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
}
