use crate::io_error::{AsyncIoError, ThreadedIoError};

#[derive(Debug, thiserror::Error)]
#[allow(clippy::large_enum_variant)]
pub enum BlockingDownloadError {
    #[error(transparent)]
    Io(#[from] ThreadedIoError),

    #[error("target directory does not exist {0}")]
    TargetDoesNotExist(String),

    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("file name is too long (max 255 chars)")]
    FileNameTooLong,

    #[error("could not determine file name")]
    NoFileName,

    #[error(
        "Download file already exists: {0}. \
        If you want to over write this file then use overwrite_existing_file(true)"
    )]
    FileExists(String),

    #[error("http::Error:\n{0:#?}")]
    HttpError(#[from] http::Error),
}

impl From<std::io::Error> for BlockingDownloadError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(ThreadedIoError::Std(err))
    }
}

#[derive(Debug, thiserror::Error)]
#[allow(clippy::large_enum_variant)]
pub enum AsyncDownloadError {
    #[error(transparent)]
    Io(#[from] AsyncIoError),

    #[error("target directory does not exist {0}")]
    TargetDoesNotExist(String),

    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("file name is too long (max 255 chars)")]
    FileNameTooLong,

    #[error("could not determine file name")]
    NoFileName,

    #[error(
        "Download file already exists: {0}. \
        If you want to over write this file then use overwrite_existing_file(true)"
    )]
    FileExists(String),

    #[error("http::Error:\n{0:#?}")]
    HttpError(#[from] http::Error),
}

impl From<std::io::Error> for AsyncDownloadError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(AsyncIoError::Std(err))
    }
}
