mod value;
mod errors;

pub use errors::{VonError, VonResult};

pub use value::{conpact_writer::CompactWriter, text_reader::TextReader, VirtualObject};