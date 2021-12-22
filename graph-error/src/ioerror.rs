use std::{path::PathBuf, sync::mpsc};

#[derive(Debug, thiserror::Error)]
pub enum ThreadedIoError {
    #[error(transparent)]
    Std(#[from] std::io::Error),

    #[error("failed to send result path: {0}")]
    Send(#[from] mpsc::SendError<Option<PathBuf>>),

    #[error("failed to receive result path: {0}")]
    Receive(#[from] mpsc::RecvError),

    #[error("failed to join copy thread")]
    Join(Box<dyn std::any::Any + Send + 'static>),

    #[error("received an null path buffer")]
    NoPath,
}

#[derive(Debug, thiserror::Error)]
pub enum AsyncIoError {
    #[error(transparent)]
    Std(#[from] std::io::Error),

    #[error(transparent)]
    ResponseStream(#[from] reqwest::Error),
}
