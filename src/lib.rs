#[cfg(test)]
mod tests {
    use rand::random;

    #[test]
    fn it_works() {
        unit_test_helper();
        assert_eq!(1 + 1, 2);
    }

    /// This is a helper for unit tests
    fn unit_test_helper() {
        random::<u32>();
    }
}
