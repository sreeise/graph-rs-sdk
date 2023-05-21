use crate::AuthorizationResult;

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
}

impl AuthorizationFailure {
    pub fn err<T: AsRef<str>>(name: T) -> AuthorizationFailure {
        AuthorizationFailure::RequiredValue {
            name: name.as_ref().to_owned(),
            message: None,
        }
    }

    pub fn result<U>(name: impl AsRef<str>) -> AuthorizationResult<U> {
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

    pub fn msg_result<T>(
        name: impl AsRef<str>,
        message: impl ToString,
    ) -> Result<T, AuthorizationFailure> {
        Err(AuthorizationFailure::RequiredValue {
            name: name.as_ref().to_owned(),
            message: Some(message.to_string()),
        })
    }

    pub fn url_parse_error<T>(url_parse_error: url::ParseError) -> Result<T, AuthorizationFailure> {
        Err(AuthorizationFailure::UrlParseError(url_parse_error))
    }
}
