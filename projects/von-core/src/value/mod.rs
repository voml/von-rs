use serde::{Serialize, Serializer};

pub mod conpact_writer;
pub mod text_reader;

pub enum VirtualObject {
    Default,
    Bool(bool),
    String(String),
}




