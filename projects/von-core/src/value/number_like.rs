use super::*;
use num::ToPrimitive;

impl ToPrimitive for VirtualObject {
    fn to_isize(&self) -> Option<isize> {
        match self {
            VirtualObject::Default => Some(0),
            VirtualObject::Integer(i) => i.to_isize(),
            _ => None,
        }
    }

    fn to_i8(&self) -> Option<i8> {
        match self {
            VirtualObject::Default => Some(0),
            VirtualObject::Integer(i) => i.to_i8(),
            _ => None,
        }
    }

    fn to_i16(&self) -> Option<i16> {
        match self {
            VirtualObject::Default => Some(0),
            VirtualObject::Integer(i) => i.to_i16(),
            _ => None,
        }
    }

    fn to_i32(&self) -> Option<i32> {
        match self {
            VirtualObject::Default => Some(0),
            VirtualObject::Integer(i) => i.to_i32(),
            _ => None,
        }
    }

    fn to_i64(&self) -> Option<i64> {
        match self {
            VirtualObject::Default => Some(0),
            VirtualObject::Integer(i) => i.to_i64(),
            _ => None,
        }
    }

    fn to_i128(&self) -> Option<i128> {
        match self {
            VirtualObject::Default => Some(0),
            VirtualObject::Integer(i) => i.to_i128(),
            _ => None,
        }
    }

    fn to_usize(&self) -> Option<usize> {
        match self {
            VirtualObject::Default => Some(0),
            VirtualObject::Integer(i) => i.to_usize(),
            _ => None,
        }
    }

    fn to_u8(&self) -> Option<u8> {
        match self {
            VirtualObject::Default => Some(0),
            VirtualObject::Integer(i) => i.to_u8(),
            _ => None,
        }
    }

    fn to_u16(&self) -> Option<u16> {
        match self {
            VirtualObject::Default => Some(0),
            VirtualObject::Integer(i) => i.to_u16(),
            _ => None,
        }
    }

    fn to_u32(&self) -> Option<u32> {
        match self {
            VirtualObject::Default => Some(0),
            VirtualObject::Integer(i) => i.to_u32(),
            _ => None,
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match self {
            VirtualObject::Default => Some(0),
            VirtualObject::Integer(i) => i.to_u64(),
            _ => None,
        }
    }

    fn to_u128(&self) -> Option<u128> {
        match self {
            VirtualObject::Default => Some(0),
            VirtualObject::Integer(i) => i.to_u128(),
            _ => None,
        }
    }

    fn to_f32(&self) -> Option<f32> {
        match self {
            VirtualObject::Default => Some(0.0),
            VirtualObject::Decimal(d) => Some(*d as f32),
            _ => None,
        }
    }

    fn to_f64(&self) -> Option<f64> {
        match self {
            VirtualObject::Default => Some(0.0),
            VirtualObject::Decimal(d) => Some(*d),
            _ => None,
        }
    }
}
