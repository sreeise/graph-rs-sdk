use core::num;
use std::error;
use std::error::Error;
use std::fmt;
use std::io;
use std::string;
use std::str::Utf8Error;

/// Error implementation for OAuth
#[derive(Debug)]
pub enum OAuthError {
    Io(io::Error),
    Parse(num::ParseIntError),
    ParseString(string::ParseError),
    Utf8Error(Utf8Error),
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::error::Error),
    DecodeError(base64::DecodeError),
}

impl OAuthError {
    pub fn error_kind(error_kind: io::ErrorKind, message: &str) -> Self {
        let e = io::Error::new(error_kind, message);
        OAuthError::from(e)
    }
}

impl fmt::Display for OAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OAuthError::Io(ref err) => write!(f, "IO error: {}", err),
            OAuthError::Parse(ref err) => write!(f, "Parse error: {}", err),
            OAuthError::ParseString(ref err) => write!(f, "Parse string error: {}", err),
            OAuthError::Utf8Error(ref err) => write!(f, "Base 64 decode error: {}", err),
            OAuthError::ReqwestError(ref err) => write!(f, "Request error: {}", err),
            OAuthError::SerdeError(ref err) => write!(f, "Serde error: {}", err),
            OAuthError::DecodeError(ref err) => write!(f, "Base 64 decode error: {}", err),
        }
    }
}

impl error::Error for OAuthError {
    fn description(&self) -> &str {
        match *self {
            OAuthError::Io(ref err) => err.description(),
            OAuthError::Parse(ref err) => error::Error::description(err),
            OAuthError::ParseString(ref err) => error::Error::description(err),
            OAuthError::Utf8Error(ref err) => err.description(),
            OAuthError::ReqwestError(ref err) => err.description(),
            OAuthError::SerdeError(ref err) => err.description(),
            OAuthError::DecodeError(ref err) => err.description(),
        }
    }

    fn source<'a>(&'a self) -> Option<&(dyn Error + 'static)> {
        match *self {
            OAuthError::Io(ref err) => Some(err),
            OAuthError::Parse(ref err) => Some(err),
            OAuthError::ParseString(ref err) => Some(err),
            OAuthError::Utf8Error(ref err) => Some(err),
            OAuthError::ReqwestError(ref err) => Some(err),
            OAuthError::SerdeError(ref err) => Some(err),
            OAuthError::DecodeError(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for OAuthError {
    fn from(err: io::Error) -> OAuthError {
        OAuthError::Io(err)
    }
}

impl From<num::ParseIntError> for OAuthError {
    fn from(err: num::ParseIntError) -> OAuthError {
        OAuthError::Parse(err)
    }
}

impl From<string::ParseError> for OAuthError {
    fn from(err: string::ParseError) -> OAuthError {
        OAuthError::ParseString(err)
    }
}

impl From<reqwest::Error> for OAuthError {
    fn from(err: reqwest::Error) -> OAuthError {
        OAuthError::ReqwestError(err)
    }
}

impl From<serde_json::error::Error> for OAuthError {
    fn from(err: serde_json::error::Error) -> OAuthError {
        OAuthError::SerdeError(err)
    }
}

impl From<base64::DecodeError> for OAuthError {
    fn from(err: base64::DecodeError) -> OAuthError {
        OAuthError::DecodeError(err)
    }
}


impl From<Utf8Error> for OAuthError {
    fn from(err: Utf8Error) -> OAuthError {
        OAuthError::Utf8Error(err)
    }
}
