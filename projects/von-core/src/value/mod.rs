use crate::{VonError, VonResult};
use num::BigInt;
use serde::{
    de::Visitor,
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct,
        SerializeTupleVariant,
    },
    Deserializer, Serialize, Serializer,
};
use std::fmt::Display;

pub mod binary_writer;
pub mod compact_writer;
mod convert;
mod from_object;
pub mod into_object;
mod number_like;
pub mod text_reader;

pub enum VirtualObject {
    Default,
    Boolean(bool),
    String(String),
    Integer(BigInt),
    Decimal(f64),
}

impl VirtualObject {
    pub fn integer<T>(value: T) -> Self
    where
        T: Into<BigInt>,
    {
        VirtualObject::Integer(value.into())
    }
}
