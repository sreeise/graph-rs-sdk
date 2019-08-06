use crate::drive::event::DownloadFormat;
use crate::drive::ItemResult;
use crate::io::iotools::IoTools;
use graph_error::GraphError;
use graph_error::GraphFailure;
use std::convert::TryFrom;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

pub struct FetchBuilder {
    path: PathBuf,
    token: String,
    target_url: String,
    file_name: Option<OsString>,
    extension: Option<String>,
}

impl FetchBuilder {
    pub fn new(target_url: &str, path: PathBuf, token: &str) -> FetchBuilder {
        FetchBuilder {
            path,
            token: token.into(),
            target_url: target_url.into(),
            file_name: None,
            extension: None,
        }
    }

    pub fn rename(&mut self, value: OsString) -> &mut Self {
        self.file_name = Some(value);
        self
    }

    pub fn ext(&mut self, value: &str) -> &mut Self {
        self.extension = Some(value.into());
        self
    }

    pub fn format(&mut self, format: DownloadFormat) -> &mut Self {
        self.ext(format.as_ref())
    }

    pub fn fetch(&mut self) -> ItemResult<PathBuf> {
        self.response(self.path.clone())
    }

    fn response<P: AsRef<Path>>(&mut self, directory: P) -> ItemResult<PathBuf> {
        // Create the directory if it does not exist.
        IoTools::create_dir(&directory)?;

        let client = reqwest::Client::builder()
            .build()
            .map_err(GraphFailure::from)?;
        let mut response = client
            .get(self.target_url.as_str())
            .bearer_auth(self.token.as_str())
            .send()?;

        let status = response.status().as_u16();
        if GraphError::is_error(status) {
            return Err(GraphFailure::from(
                GraphError::try_from(&mut response).unwrap_or_default(),
            ));
        }

        if let Some(name) = self.file_name.as_ref() {
            if name.len() <= 255 {
                let path = directory.as_ref().join(name);
                if let Some(ext) = self.extension.as_ref() {
                    path.with_extension(ext.as_str());
                }

                return IoTools::copy((path, response));
            }
        }

        if self.file_name.is_none() {
            if let Some(name) = response
                .url()
                .path_segments()
                .and_then(std::iter::Iterator::last)
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
            {
                if name.len() <= 255 {
                    let path = directory.as_ref().join(name);
                    if let Some(ext) = self.extension.as_ref() {
                        path.with_extension(ext.as_str());
                    }
                    return IoTools::copy((path, response));
                }
            }
        }

        Err(GraphFailure::none_err(
            "Could not determine file name or the file name exceeded 255 characters",
        ))
    }
}
