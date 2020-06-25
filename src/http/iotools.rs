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

    pub fn copy(mut response: (PathBuf, reqwest::blocking::Response)) -> GraphResult<PathBuf> {
        let (sender, receiver) = mpsc::channel();
        let handle = thread::spawn(move || {
            let mut file_writer = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(&response.0)
                .expect("Error creating file");
            copy(&mut response.1, &mut file_writer).expect("Error copying file contents");
            sender.send(Some(response.0)).unwrap();
        });

        handle.join().expect("Thread could not be joined");
        match receiver.recv() {
            Ok(t) => {
                Ok(t.ok_or_else(|| GraphFailure::not_found("Unknown error downloading file"))?)
            },
            Err(e) => Err(GraphFailure::from(e)),
        }
    }

    pub async fn copy_async(response: (PathBuf, reqwest::Response)) -> GraphResult<PathBuf> {
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&response.0)
            .await?;
        let mut stream = response.1.bytes_stream();
        while let Some(item) = stream.next().await {
            file.write_all(&item?).await?;
        }
        Ok(response.0)
    }
}
