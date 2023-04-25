#[derive(Debug, Copy, Clone)]
pub enum VonError {
    UnknownError
}

pub type VonResult<T> = std::result::Result<T, VonError>;
