use serde::{Serialize, Serializer};

pub mod serializer;
pub mod deserializer;

pub enum VirtualObject {
    Default,
    Bool(bool),
    String(String),
}

pub struct TextReader {

}


