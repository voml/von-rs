use serde::Deserialize;
use von::{from_str, VirtualObject};

#[test]
#[allow(clippy::bool_assert_comparison)]
fn test_primitives() {
    assert_eq!(bool::deserialize(VirtualObject::Boolean(true)), Ok(true));
    assert_eq!(bool::deserialize(VirtualObject::Boolean(false)), Ok(false));
    assert_eq!(i8::deserialize(VirtualObject::integer(256)), Ok(127));
}
