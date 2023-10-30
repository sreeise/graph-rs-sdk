use std::error;
use std::error::Error;
use std::fmt;
use std::io::ErrorKind;

use graph_error::GraphFailure;

use crate::auth::OAuthParameter;

/// Error implementation for OAuth
#[derive(Debug)]
pub enum OAuthError {
    GraphFailure(GraphFailure),
}

impl OAuthError {
    pub fn error_kind(error_kind: ErrorKind, message: &str) -> GraphFailure {
        let e = std::io::Error::new(error_kind, message);
        GraphFailure::from(e)
    }

    pub fn invalid_data<T>(msg: &str) -> std::result::Result<T, GraphFailure> {
        Err(OAuthError::error_kind(ErrorKind::InvalidData, msg))
    }

    pub fn invalid(msg: &str) -> GraphFailure {
        OAuthError::error_kind(ErrorKind::InvalidData, msg)
    }
    pub fn credential_error(c: OAuthParameter) -> GraphFailure {
        GraphFailure::error_kind(
            ErrorKind::NotFound,
            format!("MISSING OR INVALID: {c:#?}").as_str(),
        )
    }
}

impl fmt::Display for OAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OAuthError::GraphFailure(ref err) => write!(f, "Graph Failure: {err}"),
        }
    }
}

impl error::Error for OAuthError {
    fn source<'a>(&'a self) -> Option<&(dyn Error + 'static)> {
        match *self {
            OAuthError::GraphFailure(ref err) => Some(err),
        }
    }
}

impl From<GraphFailure> for OAuthError {
    fn from(err: GraphFailure) -> Self {
        OAuthError::GraphFailure(err)
    }
}
