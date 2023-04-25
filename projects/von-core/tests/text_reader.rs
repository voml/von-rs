use von::from_str;

#[test]
#[allow(clippy::bool_assert_comparison)]
fn test_primitives() {
    assert_eq!(from_str::<bool>("true").unwrap(), true);
    assert_eq!(from_str::<bool>("false").unwrap(), false);
    assert_eq!(from_str::<u8>("+1").unwrap(), 1);
    assert_eq!(from_str::<i8>("-1").unwrap(), -1);
}
