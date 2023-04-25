use super::*;
use std::io::Write;

pub struct BinaryWriter<'i, W> {
    buffer: &'i mut W,
}

impl<'i, W> BinaryWriter<'i, W>
where
    W: Write,
{
    pub fn new(buffer: &'i mut W) -> Self {
        BinaryWriter { buffer }
    }
}
