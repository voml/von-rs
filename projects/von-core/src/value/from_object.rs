use super::*;
use num::ToPrimitive;
use serde::de::{Error, Unexpected};

impl VirtualObject {
    fn as_unexpected_type(&self) -> Unexpected {
        match self {
            VirtualObject::Default => Unexpected::Other("default"),
            VirtualObject::Boolean(v) => Unexpected::Bool(*v),
            VirtualObject::String(v) => Unexpected::Str(v),
            VirtualObject::Integer(_) => Unexpected::Other("integer"),
            VirtualObject::Decimal(_) => Unexpected::Other("decimal"),
        }
    }
}

impl<'de> Deserializer<'de> for VirtualObject {
    type Error = VonError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self {
            VirtualObject::Boolean(v) => visitor.visit_bool(*v),
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_type(unexpected, &"`true` or `false`"))
            }
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self {
            VirtualObject::Integer(v) => match v.to_i8() {
                Some(v) => visitor.visit_i8(v),
                None => {
                    Err(Error::custom(format!("integer `{v}` does not fit into the type `i8` whose range is `-128..=127`")))
                }
            },
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_type(unexpected, &"`integer` in range `-128..=127`"))
            }
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self {
            VirtualObject::Integer(v) => match v.to_i16() {
                Some(v) => visitor.visit_i16(v),
                None => Err(Error::custom(format!(
                    "integer `{v}` does not fit into the type `i16` whose range is `-32768..=32767`"
                ))),
            },
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_type(unexpected, &"`integer`"))
            }
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self {
            VirtualObject::Integer(v) => match v.to_i32() {
                Some(v) => visitor.visit_i32(v),
                None => Err(Error::custom(format!(
                    "integer `{v}` does not fit into the type `i32` whose range is `-2147483648..=2147483647`"
                ))),
            },
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_type(unexpected, &"`integer`"))
            }
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self {
            VirtualObject::Integer(v) => match v.to_i64() {
                Some(v) => visitor.visit_i64(v),
                None => Err(Error::custom(format!(
                    "integer `{v}` does not fit into the type `i64` whose range is `-9223372036854775808..=9223372036854775807`"
                ))),
            },
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_type(unexpected, &"`integer`"))
            }
        }
    }

    fn deserialize_i128<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self {
            VirtualObject::Integer(v) => match v.to_i128() {
                Some(v) => visitor.visit_i128(v),
                None => Err(Error::custom(format!(
                    "integer `{v}` does not fit into the type `i128` whose range is `-170141183460469231731687303715884105728..=170141183460469231731687303715884105727`"
                ))),
            },
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_type(unexpected, &"`integer`"))
            }
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self {
            VirtualObject::Integer(v) => match v.to_u8() {
                Some(v) => visitor.visit_u8(v),
                None => Err(Error::custom(format!("integer `{v}` does not fit into the type `u8` whose range is `0..=255`"))),
            },
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_type(unexpected, &"`integer`"))
            }
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self {
            VirtualObject::Integer(v) => match v.to_u16() {
                Some(v) => visitor.visit_u16(v),
                None => {
                    Err(Error::custom(format!("integer `{v}` does not fit into the type `u16` whose range is `0..=65535`")))
                }
            },
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_type(unexpected, &"`integer`"))
            }
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self {
            VirtualObject::Integer(v) => match v.to_u32() {
                Some(v) => visitor.visit_u32(v),
                None => Err(Error::custom(format!(
                    "integer `{v}` does not fit into the type `u32` whose range is `0..=4294967295`"
                ))),
            },
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_type(unexpected, &"`integer`"))
            }
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self {
            VirtualObject::Integer(v) => match v.to_u64() {
                Some(v) => visitor.visit_u64(v),
                None => Err(Error::custom(format!(
                    "integer `{v}` does not fit into the type `u64` whose range is `0..=18446744073709551615`"
                ))),
            },
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_type(unexpected, &"`integer`"))
            }
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self {
            VirtualObject::Integer(v) => match v.to_u128() {
                Some(v) => visitor.visit_u128(v),
                None => Err(Error::custom(format!(
                    "integer `{v}` does not fit into the type `u128` whose range is `0..=340282366920938463463374607431768211455`"
                ))),
            },
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_type(unexpected, &"`integer`"))
            }
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self {
            VirtualObject::Decimal(v) => visitor.visit_f32(*v as f32),
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_type(unexpected, &"`decimal`"))
            }
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self {
            VirtualObject::Decimal(v) => visitor.visit_f64(*v),
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_type(unexpected, &"`decimal`"))
            }
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn is_human_readable(&self) -> bool {
        todo!()
    }
}
