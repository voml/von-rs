use super::*;
use std::io::Write;

pub struct ObjectWriter {
    object: VirtualObject,
}

impl Default for ObjectWriter {
    fn default() -> Self {
        ObjectWriter { object: VirtualObject::Default }
    }
}

impl ObjectWriter {
    pub fn new(object: VirtualObject) -> Self {
        ObjectWriter { object }
    }
}
