use crate::AuthorizationResult;

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
    pub fn required_value<T: AsRef<str>, U>(name: T) -> AuthorizationResult<U> {
        Err(AuthorizationFailure::RequiredValue {
            name: name.as_ref().to_owned(),
            message: None,
        })
    }

    pub fn required_value_msg<T>(
        name: &str,
        message: Option<&str>,
    ) -> Result<T, AuthorizationFailure> {
        Err(AuthorizationFailure::RequiredValue {
            name: name.to_owned(),
            message: message.map(|s| s.to_owned()),
        })
    }

    pub fn url_parse_error<T>(url_parse_error: url::ParseError) -> Result<T, AuthorizationFailure> {
        Err(AuthorizationFailure::UrlParseError(url_parse_error))
    }
}
