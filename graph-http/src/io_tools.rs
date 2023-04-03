use futures::StreamExt;
use graph_error::io_error::{AsyncIoError, ThreadedIoError};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
};
use tokio::io::AsyncWriteExt;

pub fn create_dir<P: AsRef<Path>>(directory: P) -> Result<(), std::io::Error> {
    if !directory.as_ref().exists() {
        fs::create_dir_all(&directory)?;
    }
    Ok(())
}

pub async fn create_dir_async<P: AsRef<Path>>(directory: P) -> Result<(), std::io::Error> {
    if !directory.as_ref().exists() {
        tokio::fs::create_dir_all(directory).await?;
    }
    Ok(())
}

pub fn copy(
    path: PathBuf,
    mut response: reqwest::blocking::Response,
) -> Result<PathBuf, ThreadedIoError> {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn::<_, Result<(), ThreadedIoError>>(move || {
        let mut file_writer = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&path)?;
        std::io::copy(&mut response, &mut file_writer)?;
        sender.send(Some(path))?;
        Ok(())
    });

    handle.join().map_err(ThreadedIoError::Join)??;
    receiver.recv()?.ok_or(ThreadedIoError::NoPath)
}

pub async fn copy_async(
    path: PathBuf,
    response: reqwest::Response,
) -> Result<PathBuf, AsyncIoError> {
    let mut file = tokio::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(&path)
        .await?;
    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        file.write_all(&item?).await?;
    }
    Ok(path)
}
