use crate::{ErrorMessage, IdentityResult};
use tokio::sync::mpsc::error::SendTimeoutError;

pub type AF = AuthorizationFailure;

/// Errors typically from missing or invalid configuration using one of the
/// identity platform clients such as AuthorizationCodeCredential.
#[derive(Debug, thiserror::Error)]
pub enum AuthorizationFailure {
    #[error("Required value missing:\n{0:#?}", name)]
    RequiredValue {
        name: String,
        message: Option<String>,
    },

    #[error("{0:#?}")]
    UrlParse(#[from] url::ParseError),

    #[error("{0:#?}")]
    Uuid(#[from] uuid::Error),

    #[error("{0:#?}")]
    Openssl(String),

    #[error("{0:#?}")]
    SerdeJson(#[from] serde_json::Error),
}

impl AuthorizationFailure {
    pub fn required<T: AsRef<str>>(name: T) -> AuthorizationFailure {
        AuthorizationFailure::RequiredValue {
            name: name.as_ref().to_owned(),
            message: None,
        }
    }

    pub fn result<U>(name: impl AsRef<str>) -> IdentityResult<U> {
        Err(AuthorizationFailure::RequiredValue {
            name: name.as_ref().to_owned(),
            message: None,
        })
    }

    pub fn msg_err<T: AsRef<str>>(name: T, message: T) -> AuthorizationFailure {
        AuthorizationFailure::RequiredValue {
            name: name.as_ref().to_owned(),
            message: Some(message.as_ref().to_owned()),
        }
    }

    pub fn msg_internal_err<T: AsRef<str>>(name: T) -> AuthorizationFailure {
        AuthorizationFailure::RequiredValue {
            name: name.as_ref().to_owned(),
            message: Some("Internal error please file an issue on GitHub https://github.com/sreeise/graph-rs-sdk/issues".to_owned()),
        }
    }

    pub fn msg_result<T>(
        name: impl AsRef<str>,
        message: impl ToString,
    ) -> Result<T, AuthorizationFailure> {
        Err(AuthorizationFailure::RequiredValue {
            name: name.as_ref().to_owned(),
            message: Some(message.to_string()),
        })
    }

    pub fn msg_internal_result<T>(name: impl AsRef<str>) -> Result<T, AuthorizationFailure> {
        Err(AF::msg_internal_err(name))
    }

    pub fn condition(cond: bool, name: &str, msg: &str) -> IdentityResult<()> {
        if cond {
            AF::msg_result(name, msg)
        } else {
            Ok(())
        }
    }

    pub fn x509(message: impl ToString) -> AuthorizationFailure {
        AuthorizationFailure::Openssl(message.to_string())
    }

    pub fn x509_result<T>(message: impl ToString) -> Result<T, AuthorizationFailure> {
        Err(AuthorizationFailure::Openssl(message.to_string()))
    }
}

/// Error either from missing or invalid configuration using one of the
/// identity platform clients or an error from the result of executing
/// an http request using the identity platform clients.
#[derive(Debug, thiserror::Error)]
pub enum AuthExecutionError {
    #[error("{0:#?}")]
    Authorization(#[from] AuthorizationFailure),

    #[error("{0:#?}")]
    Request(#[from] reqwest::Error),

    #[error("{0:#?}")]
    Http(#[from] http::Error),

    #[error("message: {0:#?}, response: {1:#?}", message, response)]
    SilentTokenAuth {
        message: String,
        response: http::Response<Result<serde_json::Value, ErrorMessage>>,
    },
}

impl AuthExecutionError {
    pub fn silent_token_auth(
        response: http::Response<Result<serde_json::Value, ErrorMessage>>,
    ) -> AuthExecutionError {
        AuthExecutionError::SilentTokenAuth {
            message: "silent token auth failed".into(),
            response,
        }
    }
}

impl From<serde_json::error::Error> for AuthExecutionError {
    fn from(value: serde_json::error::Error) -> Self {
        AuthExecutionError::Authorization(AuthorizationFailure::from(value))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthTaskExecutionError<R> {
    #[error("{0:#?}")]
    AuthExecutionError(#[from] AuthExecutionError),
    #[error("Tokio SendTimeoutError - Reason: {0:#?}")]
    SendTimeoutErrorAsync(#[from] SendTimeoutError<R>),
    #[error("{0:#?}")]
    JoinError(#[from] tokio::task::JoinError),
}
