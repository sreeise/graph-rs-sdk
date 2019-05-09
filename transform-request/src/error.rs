use base64;
use graph_error::GraphError;
use serde_json;
use std::error::Error;
use std::io::ErrorKind;
use std::str::Utf8Error;
use std::sync::mpsc;
use std::sync::mpsc::RecvError;
use std::{error, fmt, io, num, string};

/// Error implementation for OAuth
#[derive(Debug)]
pub enum RequestError {
    Io(io::Error),
    Parse(num::ParseIntError),
    ParseString(string::ParseError),
    Utf8Error(Utf8Error),
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::error::Error),
    DecodeError(base64::DecodeError),
    GraphError(GraphError),
    RecvError(mpsc::RecvError),
}

impl RequestError {
    pub fn error_kind(error_kind: io::ErrorKind, message: &str) -> Self {
        let e = io::Error::new(error_kind, message);
        RequestError::from(e)
    }

    pub fn none_err(message: &str) -> Self {
        let string = format!(
            "Retrieving the value for: {:#?} has resulted in a None value",
            message
        );
        RequestError::error_kind(ErrorKind::InvalidData, string.as_str())
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RequestError::Io(ref err) => write!(f, "IO error: {}", err),
            RequestError::Parse(ref err) => write!(f, "Parse error: {}", err),
            RequestError::ParseString(ref err) => write!(f, "Parse string error: {}", err),
            RequestError::Utf8Error(ref err) => write!(f, "Base 64 decode error: {}", err),
            RequestError::ReqwestError(ref err) => write!(f, "Request error: {}", err),
            RequestError::SerdeError(ref err) => write!(f, "Serde error: {}", err),
            RequestError::DecodeError(ref err) => write!(f, "Base64 decode error: {}", err),
            RequestError::GraphError(ref err) => write!(f, "Graph error: {}", err),
            RequestError::RecvError(ref err) => write!(f, "Recv error: {}", err),
        }
    }
}

impl error::Error for RequestError {
    fn description(&self) -> &str {
        match *self {
            RequestError::Io(ref err) => err.description(),
            RequestError::Parse(ref err) => error::Error::description(err),
            RequestError::ParseString(ref err) => error::Error::description(err),
            RequestError::Utf8Error(ref err) => err.description(),
            RequestError::ReqwestError(ref err) => err.description(),
            RequestError::SerdeError(ref err) => err.description(),
            RequestError::DecodeError(ref err) => err.description(),
            RequestError::GraphError(ref err) => err.description(),
            RequestError::RecvError(ref err) => err.description(),
        }
    }

    fn source<'a>(&'a self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RequestError::Io(ref err) => Some(err),
            RequestError::Parse(ref err) => Some(err),
            RequestError::ParseString(ref err) => Some(err),
            RequestError::Utf8Error(ref err) => Some(err),
            RequestError::ReqwestError(ref err) => Some(err),
            RequestError::SerdeError(ref err) => Some(err),
            RequestError::DecodeError(ref err) => Some(err),
            RequestError::RecvError(ref err) => Some(err),
            RequestError::GraphError(_) => None,
        }
    }
}

impl From<io::Error> for RequestError {
    fn from(err: io::Error) -> RequestError {
        RequestError::Io(err)
    }
}

impl From<num::ParseIntError> for RequestError {
    fn from(err: num::ParseIntError) -> RequestError {
        RequestError::Parse(err)
    }
}

impl From<string::ParseError> for RequestError {
    fn from(err: string::ParseError) -> RequestError {
        RequestError::ParseString(err)
    }
}

impl From<reqwest::Error> for RequestError {
    fn from(err: reqwest::Error) -> RequestError {
        RequestError::ReqwestError(err)
    }
}

impl From<serde_json::error::Error> for RequestError {
    fn from(err: serde_json::error::Error) -> RequestError {
        RequestError::SerdeError(err)
    }
}

impl From<base64::DecodeError> for RequestError {
    fn from(err: base64::DecodeError) -> RequestError {
        RequestError::DecodeError(err)
    }
}

impl From<Utf8Error> for RequestError {
    fn from(err: Utf8Error) -> RequestError {
        RequestError::Utf8Error(err)
    }
}

impl From<GraphError> for RequestError {
    fn from(err: GraphError) -> Self {
        RequestError::GraphError(err)
    }
}

impl From<RecvError> for RequestError {
    fn from(err: RecvError) -> Self {
        RequestError::RecvError(err)
    }
}
