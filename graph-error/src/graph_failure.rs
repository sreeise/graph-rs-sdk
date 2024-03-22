use crate::download::AsyncDownloadError;
use crate::internal::GraphRsError;
use crate::{AuthExecutionError, AuthorizationFailure, ErrorMessage};
use reqwest::header::HeaderMap;
use std::cell::BorrowMutError;
use std::io;
use std::io::ErrorKind;
use std::str::Utf8Error;
use std::sync::mpsc;

#[derive(Debug, thiserror::Error)]
#[allow(clippy::large_enum_variant)]
pub enum GraphFailure {
    #[error("{0:#?}")]
    Io(#[from] io::Error),

    #[error("{0:#?}")]
    Utf8Error(#[from] Utf8Error),

    #[error("{0:#?}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("{0:#?}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("{0:#?}")]
    DecodeError(#[from] base64::DecodeError),

    #[error("{0:#?}")]
    RecvError(#[from] mpsc::RecvError),

    #[error("{0:#?}")]
    BorrowMutError(#[from] BorrowMutError),

    #[error("{0:#?}")]
    UrlParse(#[from] url::ParseError),

    #[error("{0:#?}")]
    HttpError(#[from] http::Error),

    #[error("{0:#?}")]
    GraphRsError(#[from] GraphRsError),

    #[error("{0:#?}")]
    HandlebarsRenderError(#[from] handlebars::RenderError),

    #[error("{0:?}")]
    HandlebarsTemplateRenderError(#[from] handlebars::TemplateRenderError),

    #[error("Crypto Error (Unknown)")]
    CryptoError,

    #[error("{0:#?}")]
    AsyncDownloadError(#[from] AsyncDownloadError),

    #[error(
        "Error building or processing request prior to being sent:\n{0:#?}",
        error
    )]
    PreFlightError {
        url: Option<reqwest::Url>,
        headers: Option<HeaderMap>,
        error: Option<Box<GraphFailure>>,
        message: String,
    },

    #[error("{0:#?}", message)]
    Default {
        url: Option<reqwest::Url>,
        headers: Option<HeaderMap>,
        message: String,
    },

    #[error("{0:#?}")]
    ErrorMessage(#[from] ErrorMessage),

    #[error("message: {0:#?}, response: {1:#?}", message, response)]
    SilentTokenAuth {
        message: String,
        response: http::Response<Result<serde_json::Value, ErrorMessage>>,
    },

    #[error("{0:#?}")]
    JsonWebToken(#[from] jsonwebtoken::errors::Error),
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

impl From<AuthExecutionError> for GraphFailure {
    fn from(value: AuthExecutionError) -> Self {
        match value {
            AuthExecutionError::Authorization(authorization_failure) => match authorization_failure
            {
                AuthorizationFailure::RequiredValue { name, message } => {
                    GraphFailure::PreFlightError {
                        url: None,
                        headers: None,
                        error: None,
                        message: format!("name: {:#?}, message: {:#?}", name, message),
                    }
                }
                AuthorizationFailure::UrlParse(error) => GraphFailure::UrlParse(error),
                AuthorizationFailure::Uuid(error) => GraphFailure::PreFlightError {
                    url: None,
                    headers: None,
                    error: None,
                    message: format!(
                        "name: client_id, message: {:#?}, source: {:#?}",
                        "Client Id is not a valid Uuid",
                        error.to_string()
                    ),
                },
                AuthorizationFailure::Openssl(message) => GraphFailure::PreFlightError {
                    url: None,
                    headers: None,
                    error: None,
                    message,
                },
                AuthorizationFailure::SerdeJson(error) => GraphFailure::SerdeJson(error),
            },
            AuthExecutionError::Request(e) => GraphFailure::ReqwestError(e),
            AuthExecutionError::Http(e) => GraphFailure::HttpError(e),
            AuthExecutionError::SilentTokenAuth { message, response } => {
                GraphFailure::SilentTokenAuth { message, response }
            }
            AuthExecutionError::JsonWebToken(error) => GraphFailure::JsonWebToken(error),
        }
    }
}
