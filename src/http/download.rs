use crate::http::{GraphRequest, GraphRequestBuilder, GraphRequestType, IoTools};
use crate::url::GraphUrl;
use graph_error::{GraphFailure, GraphResult};
use reqwest::{Method, Response};
use std::cell::RefCell;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

/// Provides an abstraction for downloading files.
pub struct FetchClient {
    path: PathBuf,
    file_name: Option<OsString>,
    extension: Option<String>,
    request: RefCell<GraphRequestBuilder>,
    client: RefCell<GraphRequest>,
}

impl FetchClient {
    pub fn new(token: &str, request: GraphRequestBuilder) -> FetchClient {
        let path = request.download_dir.clone().unwrap();
        let mut client = GraphRequest::default();
        client.set_token(token);
        FetchClient {
            path,
            file_name: None,
            extension: None,
            request: RefCell::new(request),
            client: RefCell::new(client),
        }
    }

    pub fn rename(&mut self, value: OsString) -> &mut Self {
        self.file_name = Some(value);
        self
    }

    pub fn set_extension(&mut self, value: &str) -> &mut Self {
        self.extension = Some(value.into());
        self
    }

    pub fn set_dir<P: AsRef<Path>>(&mut self, path: P) {
        self.path = path.as_ref().to_path_buf();
    }

    pub fn directory(&self) -> &PathBuf {
        &self.path
    }

    pub fn file_name(&self) -> Option<&OsString> {
        self.file_name.as_ref()
    }

    pub fn extension(&self) -> Option<&String> {
        self.extension.as_ref()
    }

    pub fn url(&self) -> GraphUrl {
        self.request.borrow().url().clone()
    }

    pub fn send(&mut self) -> GraphResult<PathBuf> {
        self.download(self.path.clone())
    }

    pub fn format(&mut self, format: &str) {
        let mut request = self.request.borrow_mut();
        request.set_request_type(GraphRequestType::Redirect);
        request.url.format(format);
    }

    fn parse_content_disposition(&mut self, header: &str) -> Option<OsString> {
        let mut v: Vec<&str> = header.split(';').collect();
        v.retain(|s| !s.is_empty());

        // The filename* indicates that the filename is encoded
        if let Some(value) = v.iter().find(|s| s.starts_with("filename*=utf-8''")) {
            let s = value.replace("filename*=utf-8''", "");
            if let Ok(s) = percent_encoding::percent_decode(s.as_bytes()).decode_utf8() {
                return Some(OsString::from(s.to_string()));
            }
        }

        if let Some(value) = v.last() {
            if value.starts_with("filename=") {
                let s = value.replace("\"", "");
                let s = s.replace("filename=", "");
                return Some(OsString::from(s.to_string()));
            }
        }
        None
    }

    fn download<P: AsRef<Path>>(&mut self, directory: P) -> GraphResult<PathBuf> {
        // Create the directory if it does not exist.
        IoTools::create_dir(&directory)?;

        if self.request.borrow().req_type == GraphRequestType::Redirect {
            let request = self.request.replace(GraphRequestBuilder::default());
            let mut response = self.client.borrow_mut().build(request).send()?;

            if let Some(err) = GraphFailure::from_response(&mut response) {
                return Err(err);
            }
            let mut request = GraphRequestBuilder::new(GraphUrl::from(response.url().clone()));
            request.set_method(Method::GET);
            self.request.replace(request);
        }

        let request = self.request.replace(GraphRequestBuilder::default());
        let mut response = self.client.borrow_mut().build(request).send()?;

        if let Some(err) = GraphFailure::from_response(&mut response) {
            return Err(err);
        }

        // If a filename was specified beforehand.
        if let Some(name) = self.file_name.as_ref() {
            if name.len() <= 255 {
                let path = directory.as_ref().join(name);
                return self.finish((path, response));
            }
        }

        // The content-disposition header, if available, may include the
        // filename either in its normal form or percent encoded.
        if let Some(value) = response.headers().get("content-disposition") {
            if let Ok(s) = std::str::from_utf8(value.as_ref()) {
                if let Some(name) = self.parse_content_disposition(s) {
                    if name.len() <= 255 {
                        let path = directory.as_ref().join(name);
                        return self.finish((path, response));
                    }
                }
            }
        }

        // This is a last ditch effort to find the file name and it
        // may not be the correct one.
        if let Some(name) = response
            .url()
            .path_segments()
            .and_then(std::iter::Iterator::last)
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
        {
            if name.len() <= 255 {
                let path = directory.as_ref().join(name);
                return self.finish((path, response));
            }
        }

        Err(GraphFailure::none_err(
            "Could not determine file name or the file name exceeded 255 characters",
        ))
    }

    fn finish(&mut self, values: (PathBuf, Response)) -> GraphResult<PathBuf> {
        if let Some(ext) = self.extension.as_ref() {
            values.0.with_extension(ext.as_str());
        }
        IoTools::copy(values)
    }
}
