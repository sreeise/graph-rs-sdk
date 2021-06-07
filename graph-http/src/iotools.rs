use async_std::prelude::*;
use graph_error::{GraphFailure, GraphResult};
use std::fs::OpenOptions;
use std::io::copy;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::{fs, thread};
use tokio::prelude::*;

pub struct IoTools;

impl IoTools {
    pub fn create_dir<P: AsRef<Path>>(directory: P) -> GraphResult<()> {
        if !directory.as_ref().exists() {
            fs::create_dir_all(&directory)?;
        }
        Ok(())
    }

    pub async fn create_dir_async<P: AsRef<Path>>(directory: P) -> GraphResult<()> {
        if !directory.as_ref().exists() {
            tokio::fs::create_dir_all(directory).await?;
        }
        Ok(())
    }

    pub fn copy(path: PathBuf, mut response: reqwest::blocking::Response) -> GraphResult<PathBuf> {
        let (sender, receiver) = mpsc::channel();
        let handle = thread::spawn::<_, Result<(), GraphFailure>>(move || {
            let mut file_writer = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(&path)?;
            copy(&mut response, &mut file_writer)?;
            sender.send(Some(path))?;
            Ok(())
        });

        handle.join().map_err(GraphFailure::ThreadJoinError)??;
        match receiver.recv() {
            Ok(t) => {
                Ok(t.ok_or_else(|| GraphFailure::not_found("Unknown error downloading file"))?)
            },
            Err(e) => Err(GraphFailure::from(e)),
        }
    }

    pub async fn copy_async(path: PathBuf, response: reqwest::Response) -> GraphResult<PathBuf> {
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
}
