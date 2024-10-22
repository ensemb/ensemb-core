use std::fmt::{Display, Formatter};
use std::io::Error as IoError;
use std::num::ParseIntError;

use serde_json::Error as SerdeError;
use shellish_parse::ParseError;
use tonic::transport::Error as TonicTransportError;

pub type Result<T> = std::result::Result<T, EnsembError>;

#[derive(Debug)]
pub enum EnsembError {
    CustomError(String),
    IoError(IoError),
    ParseError(ParseError),
    ParseIntError(ParseIntError),
    SerdeError(SerdeError),
    StaticError(&'static str),
    TonicTransportError(TonicTransportError),
}

impl Display for EnsembError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EnsembError::CustomError(e) => write!(f, "{}", e),
            EnsembError::IoError(e) => write!(f, "IO error: {}", e),
            EnsembError::ParseError(e) => write!(f, "Parse error: {}", e),
            EnsembError::ParseIntError(e) => write!(f, "Parse int error: {}", e),
            EnsembError::SerdeError(e) => write!(f, "Serde error: {}", e),
            EnsembError::StaticError(e) => write!(f, "{}", e),
            EnsembError::TonicTransportError(e) => write!(f, "Tonic transport error: {}", e),
        }
    }
}

impl From<IoError> for EnsembError {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for EnsembError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl From<ParseError> for EnsembError {
    fn from(e: ParseError) -> Self {
        Self::ParseError(e)
    }
}

impl From<SerdeError> for EnsembError {
    fn from(e: SerdeError) -> Self {
        Self::SerdeError(e)
    }
}

impl From<TonicTransportError> for EnsembError {
    fn from(e: TonicTransportError) -> Self {
        Self::TonicTransportError(e)
    }
}
