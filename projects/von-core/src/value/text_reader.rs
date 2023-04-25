use super::*;

pub struct TextReader<'i> {
    buffer: &'i str,
}

impl<'i> TextReader<'i> {
    pub fn new(input: &'i str) -> Self {
        TextReader { buffer: input }
    }
}

impl<'i, 'de> Deserializer<'de> for TextReader<'i> {
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
        visitor.visit_bool(self.buffer.parse::<bool>()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.buffer.parse::<i8>()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.buffer.parse::<i16>()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.buffer.parse::<i32>()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.buffer.parse::<i64>()?)
    }

    fn deserialize_i128<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i128(self.buffer.parse::<i128>()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.buffer.parse::<u8>()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.buffer.parse::<u16>()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.buffer.parse::<u32>()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.buffer.parse::<u64>()?)
    }

    fn deserialize_u128<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u128(self.buffer.parse::<u128>()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.buffer.parse::<f32>()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> VonResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.buffer.parse::<f64>()?)
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
