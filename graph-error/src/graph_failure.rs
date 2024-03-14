use crate::download::AsyncDownloadError;
use crate::internal::GraphRsError;
use crate::ErrorMessage;
use reqwest::header::HeaderMap;
use std::cell::BorrowMutError;
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::num::ParseIntError;
use std::str::Utf8Error;
use std::sync::mpsc;

#[derive(Debug, thiserror::Error)]
#[allow(clippy::large_enum_variant)]
pub enum GraphFailure {
    #[error("IO error:\n{0:#?}")]
    Io(#[from] io::Error),

    #[error("Base 64 decode error:\n{0:#?}")]
    Utf8Error(#[from] Utf8Error),

    #[error("Request error:\n{0:#?}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Request error:\n{0:#?}")]
    ReqwestHeaderToStr(#[from] reqwest::header::ToStrError),

    #[error("Serde error:\n{0:#?}")]
    SerdeError(#[from] serde_json::error::Error),

    #[error("Base64 decode error:\n{0:#?}")]
    DecodeError(#[from] base64::DecodeError),

    #[error("Recv error:\n{0:#?}")]
    RecvError(#[from] mpsc::RecvError),

    #[error("Borrow Mut Error error:\n{0:#?}")]
    BorrowMutError(#[from] BorrowMutError),

    #[error("Url parse error:\n{0:#?}")]
    UrlParseError(#[from] url::ParseError),

    #[error("http::Error:\n{0:#?}")]
    HttpError(#[from] http::Error),

    #[error("Internal error:\n{0:#?}")]
    GraphRsError(#[from] GraphRsError),

    #[error("Handlebars render error:\n{0:#?}")]
    HandlebarsRenderError(#[from] handlebars::RenderError),

    #[error("Handlebars template render error:\n{0:?}")]
    HandlebarsTemplateRenderError(#[from] handlebars::TemplateRenderError),

    #[error("Crypto Error (Unknown)")]
    CryptoError,

    #[error("Async Download Error:\n{0:#?}")]
    AsyncDownloadError(#[from] AsyncDownloadError),

    #[error(
        "Error building or processing request prior to being sent:\n{0:#?}",
        error
    )]
    PreFlightError {
        url: Option<reqwest::Url>,
        headers: HeaderMap,
        error: Box<GraphFailure>,
    },

    #[error("{0:#?}", message)]
    Default {
        url: Option<reqwest::Url>,
        headers: Option<HeaderMap>,
        message: String,
    },

    #[error("{0:#?}")]
    ErrorMessage(#[from] ErrorMessage),

    #[error("Temporary Graph API Error")]
    TemporaryError,

    #[error("Parse Int error:\n{0:#?}")]
    ParseIntError(#[from] ParseIntError),
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
}

impl Default for GraphFailure {
    fn default() -> Self {
        GraphFailure::Default {
            url: None,
            headers: None,
            message: "Unknown Error".into(),
        }
    }
}

impl From<ring::error::Unspecified> for GraphFailure {
    fn from(_: ring::error::Unspecified) -> Self {
        GraphFailure::CryptoError
    }
}

impl From<Box<dyn Error + Send + Sync>> for GraphFailure {
    fn from(value: Box<dyn Error + Send + Sync>) -> Self {
        value.into()
    }
}