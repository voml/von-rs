use super::*;

impl serde::de::Error for VonError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        let decode = VonErrorKind::DecodeError { message: msg.to_string() };
        VonError { kind: Box::new(decode) }
    }

    fn invalid_type(unexp: Unexpected, exp: &dyn Expected) -> Self {
        todo!()
    }

    fn invalid_value(unexp: Unexpected, exp: &dyn Expected) -> Self {
        todo!()
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
        <Self as serde::de::Error>::custom(value)
    }
}
