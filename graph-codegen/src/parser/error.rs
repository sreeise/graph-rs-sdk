use from_as::FromAsError;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ParseError {
    DeserializeMatchTarget,
    ReqwestError(reqwest::Error),
    FromAsError(FromAsError),
    Path,
    Unknown,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::DeserializeMatchTarget => {
                write!(f, "Failed to parse MatchTarget. String is in wrong format.")
            }
            ParseError::ReqwestError(err) => err.fmt(f),
            ParseError::FromAsError(err) => err.fmt(f),
            ParseError::Path => write!(f, "Failed to parse path."),
            ParseError::Unknown => write!(f, "Either the error is unknown or is irrelevant"),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<reqwest::Error> for ParseError {
    fn from(err: reqwest::Error) -> Self {
        ParseError::ReqwestError(err)
    }
}

impl From<FromAsError> for ParseError {
    fn from(err: FromAsError) -> Self {
        ParseError::FromAsError(err)
    }
}

impl From<()> for ParseError {
    fn from(_: ()) -> Self {
        ParseError::Unknown
    }
}
