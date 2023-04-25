use std::error::Error;
use std::fmt::{Display, Formatter};
use serde::de::{Expected, Unexpected};

#[derive(Clone, Debug)]
pub struct VonError {
    kind: VonErrorKind,
}

#[derive(Clone, Debug)]
pub enum VonErrorKind {}

pub type VonResult<T> = Result<T, VonError>;


impl Display for VonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for VonError {}

impl serde::ser::Error for VonError {
    fn custom<T>(msg: T) -> Self where T: Display {
        todo!()
    }
}

impl serde::de::Error for VonError {
    fn custom<T>(msg: T) -> Self where T: Display {
        todo!()
    }

    fn invalid_type(unexp: Unexpected, exp: &Expected) -> Self {
        todo!()
    }

    fn invalid_value(unexp: Unexpected, exp: &Expected) -> Self {
        todo!()
    }

    fn invalid_length(len: usize, exp: &Expected) -> Self {
        todo!()
    }

    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        todo!()
    }

    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        todo!()
    }

    fn missing_field(field: &'static str) -> Self {
        todo!()
    }

    fn duplicate_field(field: &'static str) -> Self {
        todo!()
    }
}