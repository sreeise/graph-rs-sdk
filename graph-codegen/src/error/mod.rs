use std::{error::Error, fmt};

#[derive(Debug)]
pub enum DeserializerError {
    SerdeJson(serde_json::error::Error),
}

impl fmt::Display for DeserializerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DeserializerError")
    }
}

impl Error for DeserializerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DeserializerError::SerdeJson(err) => err.source(),
        }
    }
}

impl From<serde_json::error::Error> for DeserializerError {
    fn from(err: serde_json::error::Error) -> Self {
        DeserializerError::SerdeJson(err)
    }
}

#[derive(Debug)]
pub enum CodegenError {
    DeserializerError(DeserializerError),
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodegenError")
    }
}

impl Error for CodegenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CodegenError::DeserializerError(err) => err.source(),
        }
    }
}

impl From<DeserializerError> for CodegenError {
    fn from(err: DeserializerError) -> Self {
        CodegenError::DeserializerError(err)
    }
}
