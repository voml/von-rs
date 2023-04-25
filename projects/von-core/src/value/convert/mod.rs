use super::*;

impl From<bool> for VirtualObject {
    fn from(value: bool) -> Self {
        VirtualObject::Boolean(value)
    }
}
