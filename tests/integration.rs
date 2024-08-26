use rand::random;

#[test]
fn it_works() {
    integration_test_helper();
    assert_eq!(1 + 1, 2);
}

/// This is a helper for integration tests
fn integration_test_helper() {
    random::<u32>();
}
