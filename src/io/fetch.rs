use crate::drive::ItemResult;
use crate::io::iotools::IOTools;
use graph_error::GraphError;
use reqwest::*;
use std::convert::TryFrom;
use std::fs::OpenOptions;
use std::io::copy;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use transform_request::RequestError;

pub struct Fetch;

impl Fetch {
    fn file_response<P: AsRef<Path>>(
        directory: P,
        target_url: &str,
        bearer_token: &str,
    ) -> ItemResult<(PathBuf, Response)> {
        let client = reqwest::Client::builder()
            .build()
            .map_err(RequestError::from)?;
        let mut response = client.get(target_url).bearer_auth(bearer_token).send()?;

        let status = response.status().as_u16();
        if GraphError::is_error(status) {
            return Err(RequestError::from(
                GraphError::try_from(&mut response).unwrap_or_default(),
            ));
        }

        Fetch::parse_response(directory, response)
    }

    fn parse_response<P: AsRef<Path>>(
        directory: P,
        response: Response,
    ) -> ItemResult<(PathBuf, Response)> {
        match response
            .url()
            .path_segments()
            .and_then(std::iter::Iterator::last)
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
        {
            Some(name) => {
                let dir = directory.as_ref().join(name);
                Ok((dir, response))
            },
            None => Err(RequestError::none_err("Unknown error downloading file")),
        }
    }

    fn copy(mut response: (PathBuf, Response)) -> ItemResult<PathBuf> {
        // Fetch the request which returns a PathBuf (result.0) and Response (result.1).
        // If the request is successful copy its contents to the new file.
        let (sender, receiver) = mpsc::channel();
        let handle = thread::spawn(move || {
            let mut file_writer = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(&response.0)
                .expect("Error creating file.");
            copy(&mut response.1, &mut file_writer).expect("Error copying file contents.");
            sender.send(Some(response.0)).unwrap();
        });

        handle.join().expect("Thread could not be joined");
        match receiver.recv() {
            Ok(t) => Ok(t.unwrap()),
            Err(e) => Err(RequestError::from(e)),
        }
    }

    pub fn file<P: AsRef<Path>>(
        directory: P,
        target: &str,
        bearer_token: &str,
    ) -> ItemResult<PathBuf> {
        // Create the directory if it does not exist.
        IOTools::create_dir(directory.as_ref())?;

        // Request file and if successful copy file contents to new file.
        match Fetch::file_response(directory, target, bearer_token) {
            Ok(result) => Fetch::copy(result),
            Err(e) => Err(e),
        }
    }
}
