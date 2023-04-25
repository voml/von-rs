use crate::{CompactWriter, VirtualObject, VonResult};
use serde::{Deserialize, Serialize};

pub fn to_string<T>(value: &T) -> VonResult<String>
where
    T: Serialize,
{
    let mut buffer = String::with_capacity(128);
    let writer = CompactWriter::new(&mut buffer);
    value.serialize(writer)?;
    Ok(buffer)
}

pub fn from_str<'de, T>(string: &str) -> VonResult<T>
where
    T: Deserialize<'de>,
{
    let reader = VirtualObject::Default;
    T::deserialize(reader)
}
