use crate::error::GraphError;
use crate::internal::GraphRsError;
use crate::GraphResult;
use handlebars::{RenderError, TemplateRenderError};
use std::cell::BorrowMutError;
use std::convert::TryFrom;
use std::error::Error;
use std::io::ErrorKind;
use std::str::Utf8Error;
use std::sync::mpsc;
use std::sync::mpsc::RecvError;
use std::{error, fmt, io, num, string};

#[cfg(nightly)]
use std::option::NoneError;

pub trait AsRes<RHS = Self> {
    fn as_err_res<T>(self) -> GraphResult<T>;

    fn as_failure(self) -> GraphFailure;
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
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
    UrlParseError(url::ParseError),
    HyperError(hyper::Error),
    HyperHttpError(hyper::http::Error),
    HyperInvalidUri(hyper::http::uri::InvalidUri),
    GraphRsError(GraphRsError),
    HandlebarsRenderError(handlebars::RenderError),
    HandlebarsTemplateRenderError(handlebars::TemplateRenderError),
    CryptoError,
}

impl GraphFailure {
    pub fn error_kind(error_kind: io::ErrorKind, message: &str) -> Self {
        let e = io::Error::new(error_kind, message);
        GraphFailure::from(e)
    }

    pub fn internal(err: GraphRsError) -> GraphFailure {
        GraphFailure::GraphRsError(err)
    }

    pub fn not_found(msg: &str) -> GraphFailure {
        GraphFailure::error_kind(ErrorKind::NotFound, msg)
    }

    pub fn invalid(msg: &str) -> Self {
        GraphFailure::internal(GraphRsError::InvalidOrMissing { msg: msg.into() })
    }

    pub fn from_response(r: &reqwest::blocking::Response) -> Option<GraphFailure> {
        GraphFailure::try_from(r).ok()
    }

    pub fn from_async_response(r: &reqwest::Response) -> Option<GraphFailure> {
        GraphFailure::try_from(r).ok()
    }
}

impl AsRes for GraphFailure {
    fn as_err_res<T>(self) -> Result<T, GraphFailure> {
        Err(self)
    }

    fn as_failure(self) -> GraphFailure {
        self
    }
}

impl Default for GraphFailure {
    fn default() -> Self {
        GraphFailure::GraphError(GraphError::default())
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
            }
            GraphFailure::UrlParseError(ref err) => write!(f, "Url parse error:\n{:#?}", err),
            GraphFailure::HyperError(ref err) => write!(f, "Hyper http error:\n{:#?}", err),
            GraphFailure::HyperHttpError(ref err) => write!(f, "Hyper http error:\n{:#?}", err),
            GraphFailure::HyperInvalidUri(ref err) => write!(f, "Hyper http error:\n{:#?}", err),
            GraphFailure::GraphRsError(ref err) => write!(f, "Internal error:\n{:#?}", err),
            GraphFailure::HandlebarsRenderError(ref err) => {
                write!(f, "Handlebars render error:\n{:#?}", err)
            }
            GraphFailure::HandlebarsTemplateRenderError(ref err) => {
                write!(f, "Handlebars template render error:\n{:#?}", err)
            }
            GraphFailure::CryptoError => write!(f, "Crypto Error (Unknown)"),
        }
    }
}

impl error::Error for GraphFailure {
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
            GraphFailure::UrlParseError(ref err) => Some(err),
            GraphFailure::HyperError(ref err) => Some(err),
            GraphFailure::HyperHttpError(ref err) => Some(err),
            GraphFailure::HyperInvalidUri(ref err) => Some(err),
            GraphFailure::GraphRsError(ref err) => Some(err),
            GraphFailure::HandlebarsRenderError(ref err) => Some(err),
            GraphFailure::HandlebarsTemplateRenderError(ref err) => Some(err),
            GraphFailure::CryptoError => None,
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

#[cfg(nightly)]
impl From<NoneError> for GraphFailure {
    fn from(_: NoneError) -> Self {
        GraphFailure::not_found("NoneError")
    }
}

impl From<url::ParseError> for GraphFailure {
    fn from(err: url::ParseError) -> Self {
        GraphFailure::UrlParseError(err)
    }
}

impl From<hyper::Error> for GraphFailure {
    fn from(err: hyper::Error) -> Self {
        GraphFailure::HyperError(err)
    }
}

impl From<hyper::http::Error> for GraphFailure {
    fn from(err: hyper::http::Error) -> Self {
        GraphFailure::HyperHttpError(err)
    }
}

impl From<hyper::http::uri::InvalidUri> for GraphFailure {
    fn from(err: hyper::http::uri::InvalidUri) -> Self {
        GraphFailure::HyperInvalidUri(err)
    }
}

impl From<ring::error::Unspecified> for GraphFailure {
    fn from(_: ring::error::Unspecified) -> Self {
        GraphFailure::CryptoError
    }
}

impl From<GraphRsError> for GraphFailure {
    fn from(err: GraphRsError) -> Self {
        GraphFailure::GraphRsError(err)
    }
}

impl From<handlebars::RenderError> for GraphFailure {
    fn from(err: RenderError) -> Self {
        GraphFailure::HandlebarsRenderError(err)
    }
}

impl From<handlebars::TemplateRenderError> for GraphFailure {
    fn from(err: TemplateRenderError) -> Self {
        GraphFailure::HandlebarsTemplateRenderError(err)
    }
}

impl TryFrom<&reqwest::blocking::Response> for GraphFailure {
    type Error = std::io::Error;

    fn try_from(value: &reqwest::blocking::Response) -> Result<Self, Self::Error> {
        Ok(GraphFailure::GraphError(GraphError::try_from(value)?))
    }
}

impl TryFrom<&reqwest::Response> for GraphFailure {
    type Error = std::io::Error;

    fn try_from(value: &reqwest::Response) -> Result<Self, Self::Error> {
        Ok(GraphFailure::GraphError(GraphError::try_from(value)?))
    }
}
