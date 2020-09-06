use serde::export::Formatter;

#[derive(Debug)]
pub enum ParserError {
    DeserializeMatchTarget,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::DeserializeMatchTarget => {
                write!(f, "Failed to parse MatchTarget. String is in wrong format.")
            },
        }
    }
}

impl std::error::Error for ParserError {}
