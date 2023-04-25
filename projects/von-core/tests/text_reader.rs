use serde::Deserialize;
use von::VirtualObject;

#[test]
#[allow(clippy::bool_assert_comparison)]
fn test_primitives() {
    assert_eq!(bool::deserialize(VirtualObject::Boolean(true)), Ok(true));
    assert_eq!(bool::deserialize(VirtualObject::Boolean(false)), Ok(false));
    assert_eq!(i8::deserialize(VirtualObject::integer(-128)), Ok(-128));
    assert_eq!(u8::deserialize(VirtualObject::integer(255)), Ok(255));
    assert_eq!(i16::deserialize(VirtualObject::integer(-32768)), Ok(-32768));
    assert_eq!(u16::deserialize(VirtualObject::integer(65535)), Ok(65535));
    assert_eq!(i32::deserialize(VirtualObject::integer(-2147483648)), Ok(-2147483648));
    assert_eq!(u32::deserialize(VirtualObject::integer(4294967295u32)), Ok(4294967295));
    assert_eq!(i64::deserialize(VirtualObject::integer(-9223372036854775808i64)), Ok(-9223372036854775808));
    assert_eq!(u64::deserialize(VirtualObject::integer(18446744073709551615u64)), Ok(18446744073709551615));
    assert_eq!(
        i128::deserialize(VirtualObject::integer(-170141183460469231731687303715884105728i128)),
        Ok(-170141183460469231731687303715884105728)
    );
    assert_eq!(
        u128::deserialize(VirtualObject::integer(340282366920938463463374607431768211455u128)),
        Ok(340282366920938463463374607431768211455)
    );
    assert_eq!(f32::deserialize(VirtualObject::decimal(1.0)), Ok(1.0));
    assert_eq!(f64::deserialize(VirtualObject::decimal(1.0)), Ok(1.0));
}

fn test_table() {
    let table = VirtualObject::table();

    assert_eq!(bool::deserialize(VirtualObject::Boolean(true)), Ok(true));
    assert_eq!(bool::deserialize(VirtualObject::Boolean(false)), Ok(false));
    assert_eq!(i8::deserialize(VirtualObject::integer(-128)), Ok(-128));
    assert_eq!(u8::deserialize(VirtualObject::integer(255)), Ok(255));
    assert_eq!(i16::deserialize(VirtualObject::integer(-32768)), Ok(-32768));
    assert_eq!(u16::deserialize(VirtualObject::integer(65535)), Ok(65535));
    assert_eq!(i32::deserialize(VirtualObject::integer(-2147483648)), Ok(-2147483648));
    assert_eq!(u32::deserialize(VirtualObject::integer(4294967295u32)), Ok(4294967295));
    assert_eq!(i64::deserialize(VirtualObject::integer(-9223372036854775808i64)), Ok(-9223372036854775808));
    assert_eq!(u64::deserialize(VirtualObject::integer(18446744073709551615u64)), Ok(18446744073709551615));
    assert_eq!(
        i128::deserialize(VirtualObject::integer(-170141183460469231731687303715884105728i128)),
        Ok(-170141183460469231731687303715884105728)
    );
    assert_eq!(
        u128::deserialize(VirtualObject::integer(340282366920938463463374607431768211455u128)),
        Ok(340282366920938463463374607431768211455)
    );
    assert_eq!(f32::deserialize(VirtualObject::decimal(1.0)), Ok(1.0));
    assert_eq!(f64::deserialize(VirtualObject::decimal(1.0)), Ok(1.0));
}
