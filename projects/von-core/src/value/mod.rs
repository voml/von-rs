use crate::{VonError, VonResult};
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
pub mod object_writer;
pub mod text_reader;

pub enum VirtualObject {
    Default,
    Bool(bool),
    String(String),
}
