use von::to_string;

#[test]
fn test_primitives() {
    assert_eq!(to_string(&true).unwrap(), "true");
    assert_eq!(to_string(&false).unwrap(), "false");
    assert_eq!(to_string(&1).unwrap(), "1");
    assert_eq!(to_string(&1.0).unwrap(), "1");
    assert_eq!(to_string(&"hello").unwrap(), "\"hello\"");
}
