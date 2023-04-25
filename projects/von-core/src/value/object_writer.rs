use super::*;
use std::io::Write;

pub struct ObjectWriter<'i, W> {
    buffer: &'i mut W,
}

impl<'i, W> ObjectWriter<'i, W>
where
    W: Write,
{
    pub fn new(buffer: &'i mut W) -> Self {
        ObjectWriter { buffer }
    }
}
