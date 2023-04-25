use super::*;
use serde::de::Error;

impl Error for VonError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        let decode = VonErrorKind::DecodeError { message: msg.to_string() };
        VonError { kind: Box::new(decode) }
    }

    fn invalid_type(wrong: Unexpected, needed: &dyn Expected) -> Self {
        Error::custom(format!("invalid type `{wrong}`, expected {needed}"))
    }

    fn invalid_value(wrong: Unexpected, needed: &dyn Expected) -> Self {
        Error::custom(format!("invalid value `{wrong}`, expected {needed}"))
    }

    fn invalid_length(len: usize, exp: &dyn Expected) -> Self {
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

impl From<ParseBoolError> for VonError {
    fn from(value: ParseBoolError) -> Self {
        <Self as Error>::custom(value)
    }
}
impl From<ParseIntError> for VonError {
    fn from(value: ParseIntError) -> Self {
        <Self as Error>::custom(value)
    }
}

impl From<ParseFloatError> for VonError {
    fn from(value: ParseFloatError) -> Self {
        <Self as Error>::custom(value)
    }
}
