use std::fmt::{Display, Formatter};
use std::io::Error as IoError;
use std::num::ParseIntError;

use serde_json::Error as SerdeError;
use shellish_parse::ParseError;
use tonic::transport::Error as TonicTransportError;

pub type Result<T> = std::result::Result<T, DomeRedError>;

#[derive(Debug)]
pub enum DomeRedError {
    CustomError(String),
    IoError(IoError),
    ParseError(ParseError),
    ParseIntError(ParseIntError),
    SerdeError(SerdeError),
    StaticError(&'static str),
    TonicTransportError(TonicTransportError),
}

impl Display for DomeRedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DomeRedError::CustomError(e) => write!(f, "{}", e),
            DomeRedError::IoError(e) => write!(f, "IO error: {}", e),
            DomeRedError::ParseError(e) => write!(f, "Parse error: {}", e),
            DomeRedError::ParseIntError(e) => write!(f, "ParseIntError error: {}", e),
            DomeRedError::SerdeError(e) => write!(f, "Serde error: {}", e),
            DomeRedError::StaticError(e) => write!(f, "{}", e),
            DomeRedError::TonicTransportError(e) => write!(f, "Tonic Transport error: {}", e),
        }
    }
}

impl From<IoError> for DomeRedError {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for DomeRedError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl From<ParseError> for DomeRedError {
    fn from(e: ParseError) -> Self {
        Self::ParseError(e)
    }
}

impl From<SerdeError> for DomeRedError {
    fn from(e: SerdeError) -> Self {
        Self::SerdeError(e)
    }
}

impl From<TonicTransportError> for DomeRedError {
    fn from(e: TonicTransportError) -> Self {
        Self::TonicTransportError(e)
    }
}
