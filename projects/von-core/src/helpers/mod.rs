use serde::Serialize;
use crate::{CompactWriter, VonResult};



pub fn to_string<T>(value: &T) -> VonResult<String> where T: Serialize {
    let mut buffer = String::with_capacity(128);
    let writer = CompactWriter::new(&mut buffer);
    value.serialize(writer)?;
    Ok(buffer)
}