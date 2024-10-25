#[derive(Debug, thiserror::Error)]
pub enum EdigeoError {
    /// IO Error fo reading & writing files
    #[error("IO Error {0}")]
    IoError(std::io::Error),
    // /// UTF-8 Error converting from raw bytes to UTF-8 Chars
    // #[error("UTF-8 Error {0}")]
    // Utf8Error(std::str::Utf8Error),
}

/// Result Type to be used by the EDGIGéO Lib
pub type EdigeoResult<T> = Result<T, EdigeoError>;

impl From<std::io::Error> for EdigeoError {
    fn from(err: std::io::Error) -> Self {
        EdigeoError::IoError(err)
    }
}
