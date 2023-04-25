use serde::de::{Expected, Unexpected};
use std::{
    error::Error,
    fmt::{Display, Formatter},
    str::ParseBoolError,
};

mod decode_error;

#[derive(Clone, Debug)]
pub struct VonError {
    kind: Box<VonErrorKind>,
}

#[derive(Clone, Debug)]
pub enum VonErrorKind {
    CustomError { message: String },
    EncodeError { message: String },
    DecodeError { message: String },
}

pub type VonResult<T> = Result<T, VonError>;

impl VonError {
    pub fn get_kind(&self) -> &VonErrorKind {
        &self.kind
    }
    pub fn set_kind(&mut self, kind: VonErrorKind) {
        self.kind = Box::new(kind);
    }
    pub fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        VonError { kind: Box::new(VonErrorKind::CustomError { message: msg.to_string() }) }
    }
}

impl Display for VonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for VonError {}

impl serde::ser::Error for VonError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        let encode = VonErrorKind::EncodeError { message: msg.to_string() };
        VonError { kind: Box::new(encode) }
    }
}

impl From<std::fmt::Error> for VonError {
    fn from(value: std::fmt::Error) -> Self {
        <Self as serde::ser::Error>::custom(value)
    }
}
