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
                Err(Error::invalid_value(unexpected, &"`true` or `false`"))
            }
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.to_i8() {
            Some(v) => visitor.visit_i8(v),
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_value(unexpected, &"`integer` in range `-128..=127`"))
            }
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.to_i16() {
            Some(v) => visitor.visit_i16(v),
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_value(unexpected, &"`integer` in range `-32768..=32767`"))
            }
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.to_i32() {
            Some(v) => visitor.visit_i32(v),
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_value(unexpected, &"`integer` in range `-2147483648..=2147483647`"))
            }
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.to_i64() {
            Some(v) => visitor.visit_i64(v),
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_value(unexpected, &"`integer` in range `-9223372036854775808..=9223372036854775807`"))
            }
        }
    }

    fn deserialize_i128<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.to_i128() {
            Some(v) => visitor.visit_i128(v),
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_value(
                    unexpected,
                    &"`integer` in range `-170141183460469231731687303715884105728..=170141183460469231731687303715884105727`",
                ))
            }
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.to_u8() {
            Some(v) => visitor.visit_u8(v),
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_value(unexpected, &"`integer` in range `0..=255`"))
            }
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.to_u16() {
            Some(v) => visitor.visit_u16(v),
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_value(unexpected, &"`integer` in range `0..=65535`"))
            }
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.to_u32() {
            Some(v) => visitor.visit_u32(v),
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_value(unexpected, &"`integer` in range `0..=4294967295`"))
            }
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.to_u64() {
            Some(v) => visitor.visit_u64(v),
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_value(unexpected, &"`integer` in range `0..=18446744073709551615`"))
            }
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.to_u128() {
            Some(v) => visitor.visit_u128(v),
            _ => {
                let unexpected = self.as_unexpected_type();
                Err(Error::invalid_value(unexpected, &"`integer` in range `0..=340282366920938463463374607431768211455`"))
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
                Err(Error::invalid_value(unexpected, &"`decimal`"))
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
                Err(Error::invalid_value(unexpected, &"`decimal`"))
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
