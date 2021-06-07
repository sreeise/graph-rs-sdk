use crate::error::GraphError;
use crate::internal::GraphRsError;
use std::cell::BorrowMutError;
use std::convert::TryFrom;
use std::io::ErrorKind;
use std::str::Utf8Error;
use std::sync::mpsc;
use std::{io, num, string};

#[derive(Debug, thiserror::Error)]
#[allow(clippy::large_enum_variant)]
pub enum GraphFailure {
    #[error("IO error:\n{0:#?}")]
    Io(#[from] io::Error),

    #[error("Parse error:\n{0:#?}")]
    Parse(#[from] num::ParseIntError),

    #[error("Parse string error:\n{0:#?}")]
    ParseString(#[from] string::ParseError),

    #[error("Base 64 decode error:\n{0:#?}")]
    Utf8Error(#[from] Utf8Error),

    #[error("Request error:\n{0:#?}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Request error:\n{0:#?}")]
    ReqwestHeaderToStr(#[from] reqwest::header::ToStrError),

    #[error("Serde error:\n{0:#?}")]
    SerdeError(#[from] serde_json::error::Error),

    #[error("Serde yaml error:\n{0:#?}")]
    SerdeYamlError(#[from] serde_yaml::Error),

    #[error("Base64 decode error:\n{0:#?}")]
    DecodeError(#[from] base64::DecodeError),

    #[error("Graph error:\n{0:#?}")]
    GraphError(#[from] GraphError),

    #[error("Recv error:\n{0:#?}")]
    RecvError(#[from] mpsc::RecvError),

    #[error("Borrow Mut Error error:\n{0:#?}")]
    BorrowMutError(#[from] BorrowMutError),

    #[error("Url parse error:\n{0:#?}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Hyper http error:\n{0:#?}")]
    HyperError(#[from] hyper::Error),

    #[error("Hyper http error:\n{0:#?}")]
    HyperHttpError(#[from] hyper::http::Error),

    #[error("Hyper http error:\n{0:#?}")]
    HyperInvalidUri(#[from] hyper::http::uri::InvalidUri),

    #[error("Internal error:\n{0:#?}")]
    GraphRsError(#[from] GraphRsError),

    #[error("Handlebars render error:\n{0:#?}")]
    HandlebarsRenderError(#[from] handlebars::RenderError),

    #[error("Handlebars template render error:\n{0:#?}")]
    HandlebarsTemplateRenderError(#[from] handlebars::TemplateRenderError),

    #[error("Crypto Error (Unknown)")]
    CryptoError,

    // TODO: maybe leave these errors in IoTools?
    #[error("Failed to send on mpsc channel: {0}")]
    ChannelSend(#[from] std::sync::mpsc::SendError<Option<std::path::PathBuf>>),
    #[error("Failed to join a spawned thread")]
    ThreadJoinError(Box<dyn std::any::Any + std::marker::Send>),
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

impl Default for GraphFailure {
    fn default() -> Self {
        GraphFailure::GraphError(GraphError::default())
    }
}

impl From<ring::error::Unspecified> for GraphFailure {
    fn from(_: ring::error::Unspecified) -> Self {
        GraphFailure::CryptoError
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
