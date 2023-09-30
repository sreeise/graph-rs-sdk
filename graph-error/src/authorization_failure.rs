use crate::IdentityResult;
use tokio::sync::mpsc::error::SendTimeoutError;

pub type AF = AuthorizationFailure;

#[derive(Debug, thiserror::Error)]
pub enum AuthorizationFailure {
    #[error("Required value missing:\n{0:#?}", name)]
    RequiredValue {
        name: String,
        message: Option<String>,
    },

    #[error("{0:#?}")]
    UrlParseError(#[from] url::ParseError),

    #[error("{0:#?}")]
    UuidError(#[from] uuid::Error),

    #[error("{0:#?}")]
    Unknown(String),
}

impl AuthorizationFailure {
    pub fn unknown<T: ToString>(value: T) -> AuthorizationFailure {
        AuthorizationFailure::Unknown(value.to_string())
    }

    pub fn unknown_result<T: ToString>(value: T) -> IdentityResult<AuthorizationFailure> {
        Err(AuthorizationFailure::Unknown(value.to_string()))
    }

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

    pub fn url_parse_error<T>(url_parse_error: url::ParseError) -> Result<T, AuthorizationFailure> {
        Err(AuthorizationFailure::UrlParseError(url_parse_error))
    }

    pub fn condition(cond: bool, name: &str, msg: &str) -> IdentityResult<()> {
        if cond {
            AF::msg_result(name, msg)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthExecutionError {
    #[error("{0:#?}")]
    AuthorizationFailure(#[from] AuthorizationFailure),
    #[error("{0:#?}")]
    RequestError(#[from] reqwest::Error),
    #[error("{0:#?}")]
    SerdeError(#[from] serde_json::error::Error),
    #[error("{0:#?}")]
    HttpError(#[from] http::Error),
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
