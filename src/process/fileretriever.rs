use crate::drive::ItemResult;
use reqwest::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::copy;
use std::ops::Try;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use transform_request::RequestError;

pub struct FileRetriever;

impl FileRetriever {
    pub fn download<P: AsRef<Path>>(directory: P, target: &str) -> ItemResult<PathBuf> {
        // Create the directory if it does not exist.
        if let Some(location) = directory.as_ref().to_str() {
            if !Path::new(&location).exists() {
                fs::create_dir_all(&directory).ok().unwrap_or_default();
            }
        }

        // Fetch the request which returns a PathBuf (result.0) and Response (result.1).
        // If the request is successful copy its contents to the new file.
        match FileRetriever::fetch_url(directory, target) {
            Ok(mut result) => {
                let (sender, receiver) = mpsc::channel();

                let handle = thread::spawn(move || {
                    let mut file_writer = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .read(true)
                        .open(&result.0)
                        .expect("Error performing one of create, write, or read path given.");
                    copy(&mut result.1, &mut file_writer)
                        .expect("Unknown error copying response contents.");
                    sender
                        .send(Some(result.0))
                        .expect("Error sending PathBuf from download.");
                });
                handle.join().expect("Thread could not be joined");

                match receiver.recv() {
                    Ok(t) => Ok(t.unwrap()),
                    Err(e) => Err(RequestError::from(e)),
                }
            },
            Err(e) => Err(e),
        }
    }

    pub fn download_as<P: AsRef<Path>>(
        directory: P,
        target: &str,
        name: &str,
    ) -> ItemResult<PathBuf> {
        // Create the directory if it does not exist.
        if let Some(location) = directory.as_ref().to_str() {
            if !Path::new(&location).exists() {
                fs::create_dir_all(&directory).into_result()?;
            }
        }

        // Fetch the request which returns a PathBuf (result.0) and Response (result.1).
        // If the request is successful copy its contents to the new file.
        match FileRetriever::fetch_url_as(directory, target, Some(name)) {
            Ok(mut result) => {
                let (sender, receiver) = mpsc::channel();

                let handle = thread::spawn(move || {
                    let mut file_writer = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .read(true)
                        .open(&result.0)
                        .expect("Error performing one of create, write, or read path given.");
                    copy(&mut result.1, &mut file_writer)
                        .expect("Unknown error copying response contents.");
                    sender.send(Some(result.0)).unwrap_or_default();
                });
                handle.join().expect("Thread could not be joined");

                match receiver.recv() {
                    Ok(t) => Ok(t.unwrap()),
                    Err(e) => Err(RequestError::from(e)),
                }
            },
            Err(e) => Err(e),
        }
    }

    pub fn fetch_url<P: AsRef<Path>>(
        directory: P,
        target_url: &str,
    ) -> ItemResult<(PathBuf, Response)> {
        let response = reqwest::get(target_url)?;

        match response
            .url()
            .path_segments()
            .and_then(std::iter::Iterator::last)
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            {
                Some(name) => {
                    let dir = directory.as_ref().join(name);
                    Ok((dir, response))
                }
                None => Err(RequestError::none_err("url path segments")),
            }
    }

    fn fetch_url_as<P: AsRef<Path>>(
        directory: P,
        target_url: &str,
        file_name: Option<&str>,
    ) -> ItemResult<(PathBuf, Response)> {
        let response = reqwest::get(target_url)?;

        match response
            .url()
            .path_segments()
            .and_then(std::iter::Iterator::last)
            .and_then(|name| {
                if file_name.is_some() {
                    file_name
                } else if name.is_empty() {
                    None
                } else {
                    Some(name)
                }
            }) {
            Some(name) => {
                let dir = directory.as_ref().join(name);
                Ok((dir, response))
            }
            None => Err(RequestError::none_err(
                "could not get file name from response",
            )),
        }
    }
}
