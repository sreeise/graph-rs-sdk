use crate::error::GraphError;
use base64;
use reqwest::Response;
use serde_json;
use std::cell::BorrowMutError;
use std::convert::TryFrom;
use std::error::Error;
use std::io::ErrorKind;
use std::option::NoneError;
use std::str::Utf8Error;
use std::sync::mpsc;
use std::sync::mpsc::RecvError;
use std::{error, fmt, io, num, string};

#[derive(Debug)]
pub enum GraphFailure {
    Io(io::Error),
    Parse(num::ParseIntError),
    ParseString(string::ParseError),
    Utf8Error(Utf8Error),
    ReqwestError(reqwest::Error),
    ReqwestHeaderToStr(reqwest::header::ToStrError),
    SerdeError(serde_json::error::Error),
    SerdeYamlError(serde_yaml::Error),
    DecodeError(base64::DecodeError),
    GraphError(GraphError),
    RecvError(mpsc::RecvError),
    BorrowMutError(BorrowMutError),
}

impl GraphFailure {
    pub fn error_kind(error_kind: io::ErrorKind, message: &str) -> Self {
        let e = io::Error::new(error_kind, message);
        GraphFailure::from(e)
    }

    pub fn invalid_data<T>(msg: &str) -> std::result::Result<T, GraphFailure> {
        Err(GraphFailure::error_kind(ErrorKind::InvalidData, msg))
    }

    pub fn not_found(msg: &str) -> GraphFailure {
        GraphFailure::error_kind(ErrorKind::NotFound, msg)
    }

    pub fn none_err(message: &str) -> Self {
        let string = format!(
            "Retrieving the value for: {:#?} has resulted in a None value",
            message
        );
        GraphFailure::error_kind(ErrorKind::InvalidData, string.as_str())
    }

    pub fn err_from(r: &mut Response) -> Option<GraphFailure> {
        if GraphError::is_error(r.status().as_u16()) {
            Some(GraphFailure::from(
                GraphError::try_from(r).unwrap_or_default(),
            ))
        } else {
            None
        }
    }
}

impl fmt::Display for GraphFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GraphFailure::Io(ref err) => write!(f, "IO error:\n{:#?}", err),
            GraphFailure::Parse(ref err) => write!(f, "Parse error:\n{:#?}", err),
            GraphFailure::ParseString(ref err) => write!(f, "Parse string error:\n{:#?}", err),
            GraphFailure::Utf8Error(ref err) => write!(f, "Base 64 decode error:\n{:#?}", err),
            GraphFailure::ReqwestError(ref err) => write!(f, "Request error:\n{:#?}", err),
            GraphFailure::ReqwestHeaderToStr(ref err) => write!(f, "Request error:\n{:#?}", err),
            GraphFailure::SerdeError(ref err) => write!(f, "Serde error:\n{:#?}", err),
            GraphFailure::SerdeYamlError(ref err) => write!(f, "Serde yaml error:\n{:#?}", err),
            GraphFailure::DecodeError(ref err) => write!(f, "Base64 decode error:\n{:#?}", err),
            GraphFailure::GraphError(ref err) => write!(f, "Graph error:\n{:#?}", err),
            GraphFailure::RecvError(ref err) => write!(f, "Recv error:\n{:#?}", err),
            GraphFailure::BorrowMutError(ref err) => {
                write!(f, "Borrow Mut Error error:\n{:#?}", err)
            },
        }
    }
}

impl error::Error for GraphFailure {
    fn description(&self) -> &str {
        match *self {
            GraphFailure::Io(ref err) => err.description(),
            GraphFailure::Parse(ref err) => error::Error::description(err),
            GraphFailure::ParseString(ref err) => error::Error::description(err),
            GraphFailure::Utf8Error(ref err) => err.description(),
            GraphFailure::ReqwestError(ref err) => err.description(),
            GraphFailure::ReqwestHeaderToStr(ref err) => err.description(),
            GraphFailure::SerdeError(ref err) => err.description(),
            GraphFailure::SerdeYamlError(ref err) => err.description(),
            GraphFailure::DecodeError(ref err) => err.description(),
            GraphFailure::GraphError(ref err) => err.description(),
            GraphFailure::RecvError(ref err) => err.description(),
            GraphFailure::BorrowMutError(ref err) => err.description(),
        }
    }

    fn source<'a>(&'a self) -> Option<&(dyn Error + 'static)> {
        match *self {
            GraphFailure::Io(ref err) => Some(err),
            GraphFailure::Parse(ref err) => Some(err),
            GraphFailure::ParseString(ref err) => Some(err),
            GraphFailure::Utf8Error(ref err) => Some(err),
            GraphFailure::ReqwestError(ref err) => Some(err),
            GraphFailure::ReqwestHeaderToStr(ref err) => Some(err),
            GraphFailure::SerdeError(ref err) => Some(err),
            GraphFailure::SerdeYamlError(ref err) => Some(err),
            GraphFailure::DecodeError(ref err) => Some(err),
            GraphFailure::RecvError(ref err) => Some(err),
            GraphFailure::GraphError(_) => None,
            GraphFailure::BorrowMutError(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for GraphFailure {
    fn from(err: io::Error) -> GraphFailure {
        GraphFailure::Io(err)
    }
}

impl From<num::ParseIntError> for GraphFailure {
    fn from(err: num::ParseIntError) -> GraphFailure {
        GraphFailure::Parse(err)
    }
}

impl From<string::ParseError> for GraphFailure {
    fn from(err: string::ParseError) -> GraphFailure {
        GraphFailure::ParseString(err)
    }
}

impl From<reqwest::Error> for GraphFailure {
    fn from(err: reqwest::Error) -> GraphFailure {
        GraphFailure::ReqwestError(err)
    }
}

impl From<reqwest::header::ToStrError> for GraphFailure {
    fn from(err: reqwest::header::ToStrError) -> GraphFailure {
        GraphFailure::ReqwestHeaderToStr(err)
    }
}

impl From<serde_json::error::Error> for GraphFailure {
    fn from(err: serde_json::error::Error) -> GraphFailure {
        GraphFailure::SerdeError(err)
    }
}

impl From<serde_yaml::Error> for GraphFailure {
    fn from(err: serde_yaml::Error) -> GraphFailure {
        GraphFailure::SerdeYamlError(err)
    }
}

impl From<base64::DecodeError> for GraphFailure {
    fn from(err: base64::DecodeError) -> GraphFailure {
        GraphFailure::DecodeError(err)
    }
}

impl From<Utf8Error> for GraphFailure {
    fn from(err: Utf8Error) -> GraphFailure {
        GraphFailure::Utf8Error(err)
    }
}

impl From<GraphError> for GraphFailure {
    fn from(err: GraphError) -> Self {
        GraphFailure::GraphError(err)
    }
}

impl From<RecvError> for GraphFailure {
    fn from(err: RecvError) -> Self {
        GraphFailure::RecvError(err)
    }
}

impl From<BorrowMutError> for GraphFailure {
    fn from(err: BorrowMutError) -> Self {
        GraphFailure::BorrowMutError(err)
    }
}

impl From<NoneError> for GraphFailure {
    fn from(_: NoneError) -> Self {
        GraphFailure::not_found("NoneError")
    }
}
