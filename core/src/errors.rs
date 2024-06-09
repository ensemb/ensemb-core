use std::fmt::{Display, Formatter};
use std::io::Error as IoError;

use serde_json::Error as SerdeError;
use shellish_parse::ParseError;
use tonic::transport::Error as TonicTransportError;

pub type Result<T> = std::result::Result<T, OracleError>;

#[derive(Debug)]
pub enum OracleError {
    CustomError(String),
    IoError(IoError),
    ParseError(ParseError),
    SerdeError(SerdeError),
    TonicTransportError(TonicTransportError),
}

impl Display for OracleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OracleError::CustomError(e) => write!(f, "Custom error: {}", e),
            OracleError::IoError(e) => write!(f, "IO error: {}", e),
            OracleError::ParseError(e) => write!(f, "Parse error: {}", e),
            OracleError::SerdeError(e) => write!(f, "Serde error: {}", e),
            OracleError::TonicTransportError(e) => write!(f, "Tonic Transport error: {}", e),
        }
    }
}

impl From<IoError> for OracleError {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseError> for OracleError {
    fn from(e: ParseError) -> Self {
        Self::ParseError(e)
    }
}

impl From<TonicTransportError> for OracleError {
    fn from(e: TonicTransportError) -> Self {
        Self::TonicTransportError(e)
    }
}

impl From<SerdeError> for OracleError {
    fn from(e: SerdeError) -> Self {
        Self::SerdeError(e)
    }
}
