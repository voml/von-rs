use std::fmt::{Display, Write};
use serde::ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};
use crate::{VonError, VonResult};
use super::*;

pub struct CompactWriter<W: Write> {
    writer: W,
}

pub struct CompactSequence {}

impl<W> Serializer for CompactWriter<W> where W: Write {
    type Ok = ();
    type Error = VonError;
    type SerializeSeq = CompactSequence;
    type SerializeTuple = CompactSequence;
    type SerializeTupleStruct = CompactSequence;
    type SerializeTupleVariant = CompactSequence;
    type SerializeMap = CompactSequence;
    type SerializeStruct = CompactSequence;
    type SerializeStructVariant = CompactSequence;

    fn serialize_bool(mut self, v: bool) -> VonResult<()> {
        let text = if v { "true" } else { "false" };
        Ok(self.writer.write_str(text)?)
    }

    fn serialize_i8(mut self, v: i8) -> VonResult<()> {
        Ok(write!(self.writer, "{}", v)?)
    }

    fn serialize_i16(mut self, v: i16) -> VonResult<()> {
        Ok(write!(self.writer, "{}", v)?)
    }

    fn serialize_i32(mut self, v: i32) -> VonResult<()> {
        Ok(write!(self.writer, "{}", v)?)
    }

    fn serialize_i64(mut self, v: i64) -> VonResult<()> {
        Ok(write!(self.writer, "{}", v)?)
    }

    fn serialize_i128(mut self, v: i128) -> VonResult<()> {
        Ok(write!(self.writer, "{}", v)?)
    }

    fn serialize_u8(mut self, v: u8) -> VonResult<()> {
        Ok(write!(self.writer, "{}", v)?)
    }

    fn serialize_u16(mut self, v: u16) -> VonResult<()> {
        Ok(write!(self.writer, "{}", v)?)
    }

    fn serialize_u32(mut self, v: u32) -> VonResult<()> {
        Ok(write!(self.writer, "{}", v)?)
    }

    fn serialize_u64(mut self, v: u64) -> VonResult<()> {
        Ok(write!(self.writer, "{}", v)?)
    }

    fn serialize_u128(mut self, v: u128) -> VonResult<()> {
        Ok(write!(self.writer, "{}", v)?)
    }

    fn serialize_f32(mut self, v: f32) -> VonResult<()> {
        Ok(write!(self.writer, "{}", v)?)
    }

    fn serialize_f64(mut self, v: f64) -> VonResult<()> {
        Ok(write!(self.writer, "{}", v)?)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }

    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error> where I: IntoIterator, <I as IntoIterator>::Item: Serialize {
        todo!()
    }

    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error> where K: Serialize, V: Serialize, I: IntoIterator<Item=(K, V)> {
        todo!()
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where T: Display {
        todo!()
    }

    fn is_human_readable(&self) -> bool {
        true
    }
}

impl SerializeSeq for CompactSequence {
    type Ok = ();
    type Error = VonError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeTuple for CompactSequence {
    type Ok = ();
    type Error = VonError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeMap for CompactSequence {
    type Ok = ();
    type Error = VonError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> where T: Serialize {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeStruct for CompactSequence {
    type Ok = ();
    type Error = VonError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}


impl SerializeTupleVariant for CompactSequence {
    type Ok = ();
    type Error = VonError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeTupleStruct for CompactSequence {
    type Ok = ();
    type Error = VonError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeStructVariant for CompactSequence {
    type Ok = ();
    type Error = VonError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where T: Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}