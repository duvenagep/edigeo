//! Contains custom [`EdigeoError`] and [`EdigeoResult`] types.

/// This Enum Represents all different EdigeoErrors that can occur.
///
/// This enum defines different types of errors that may occur when performing
/// operations such as reading, writing, or processing Edigeo files.
#[derive(Debug, thiserror::Error)]
pub enum EdigeoError {
    /// IO Error fo reading & writing files
    #[error("IO Error {0}")]
    IoError(std::io::Error),
    /// UTF-8 Error converting from raw bytes to UTF-8 Chars
    #[error("UTF-8 Error {0}")]
    Utf8Error(std::str::Utf8Error),
    /// Represents an invalid format field input.
    #[error("Invalid Character for ValueFormat: {0}")]
    InvalidFormat(String),
    /// ParseIntError converting from 03 -> 3
    #[error("ParseInt Error {0}")]
    ParseIntError(std::num::ParseIntError),
    /// ParseFloatError converting from '03.3' -> 3.3
    #[error("ParseFloat Error {0}")]
    ParseFloatError(std::num::ParseFloatError),
}

/// Result Type to be used by the EDIGéO Lib
pub type EdigeoResult<T> = Result<T, EdigeoError>;

impl From<std::io::Error> for EdigeoError {
    fn from(err: std::io::Error) -> Self {
        EdigeoError::IoError(err)
    }
}

impl From<std::str::Utf8Error> for EdigeoError {
    fn from(err: std::str::Utf8Error) -> Self {
        EdigeoError::Utf8Error(err)
    }
}

impl From<std::num::ParseIntError> for EdigeoError {
    fn from(err: std::num::ParseIntError) -> Self {
        EdigeoError::ParseIntError(err)
    }
}

impl From<std::num::ParseFloatError> for EdigeoError {
    fn from(err: std::num::ParseFloatError) -> Self {
        EdigeoError::ParseFloatError(err)
    }
}
