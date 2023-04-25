use super::*;

pub struct TextReader<F> {
    parser: F,
}

impl<F> TextReader<F>
where
    F: Fn(&str) -> VonResult<VirtualObject>,
{
    pub fn new(parser: F) -> Self {
        TextReader { parser }
    }

    pub fn parse<T>(&self, text: T) -> VonResult<VirtualObject>
    where
        T: AsRef<str>,
    {
        (self.parser)(text.as_ref())
    }
}
