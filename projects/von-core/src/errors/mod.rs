use std::error::Error;
use std::fmt::{Display, Formatter};

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
}