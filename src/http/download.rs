use crate::graph_error::AsRes;
use crate::http::{GraphRequest, GraphRequestBuilder, GraphRequestType, IoTools};
use crate::url::GraphUrl;
use graph_error::{GraphFailure, GraphResult, GraphRsError};
use reqwest::{Method, Response};
use std::cell::{Cell, Ref, RefCell};
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

/// Provides an abstraction for downloading files.
pub struct DownloadClient {
    path: RefCell<PathBuf>,
    create_dir_all: Cell<bool>,
    overwrite_existing_file: Cell<bool>,
    file_name: RefCell<Option<OsString>>,
    extension: RefCell<Option<String>>,
    request: RefCell<GraphRequestBuilder>,
    client: RefCell<GraphRequest>,
}

impl DownloadClient {
    pub fn new(token: &str, request: GraphRequestBuilder) -> DownloadClient {
        let path = request.download_dir.clone().unwrap();
        let mut client = GraphRequest::default();
        client.set_token(token);
        DownloadClient {
            path: RefCell::new(path),
            create_dir_all: Cell::new(true),
            overwrite_existing_file: Cell::new(false),
            file_name: RefCell::new(None),
            extension: RefCell::new(None),
            request: RefCell::new(request),
            client: RefCell::new(client),
        }
    }

    pub fn create_dir_all(&self, value: bool) -> &Self {
        self.create_dir_all.set(value);
        self
    }

    pub fn is_create_dir_all(&self) -> bool {
        self.create_dir_all.get()
    }

    pub fn overwrite_existing_file(&self, value: bool) -> &Self {
        self.overwrite_existing_file.set(value);
        self
    }

    pub fn is_overwrite_existing_file(&self) -> bool {
        self.overwrite_existing_file.get()
    }

    pub fn rename(&self, value: OsString) -> &Self {
        self.file_name.replace(Some(value));
        self
    }

    pub fn set_extension(&self, value: &str) -> &Self {
        self.extension.replace(Some(value.into()));
        self
    }

    pub fn set_dir<P: AsRef<Path>>(&self, path: P) -> &Self {
        self.path.replace(path.as_ref().to_path_buf());
        self
    }

    pub fn directory(&self) -> Ref<PathBuf> {
        self.path.borrow()
    }

    pub fn file_name(&self) -> Ref<Option<OsString>> {
        self.file_name.borrow()
    }

    pub fn extension(&self) -> Ref<Option<String>> {
        self.extension.borrow()
    }

    pub fn url(&self) -> GraphUrl {
        self.request.borrow().url().clone()
    }

    pub fn send(&self) -> GraphResult<PathBuf> {
        self.download()
    }

    pub fn format(&self, format: &str) {
        let mut request = self.request.borrow_mut();
        request.set_request_type(GraphRequestType::Redirect);
        request.url.format(format);
    }

    fn parse_content_disposition(&self, header: &str) -> Option<OsString> {
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
                return Some(OsString::from(
                    value.replace("\"", "")
                    .replace("filename=", "")
                ));
            }
        }
        None
    }

    fn download(&self) -> GraphResult<PathBuf> {
        let path = self.path.replace(PathBuf::new());

        // Create the directory if it does not exist.
        if self.is_create_dir_all() {
            IoTools::create_dir(path.as_path())?;
        } else if !path.exists() {
            let dir = path.to_string_lossy().to_string();
            return GraphRsError::DownloadDirNoExists { dir }.as_err_res();
        }

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
        if self.file_name.borrow().is_some() {
            let name = self.file_name.replace(None).unwrap();
            if name.len() <= 255 {
                let path = path.join(name);
                if path.exists() && !self.is_overwrite_existing_file() {
                    return GraphRsError::DownloadFileExists {
                        name: path.to_string_lossy().to_string(),
                    }
                    .as_err_res();
                }
                return self.finish((path, response));
            }
        }

        // The content-disposition header, if available, may include the
        // filename either in its normal form or percent encoded.
        if let Some(value) = response.headers().get("content-disposition") {
            if let Ok(s) = std::str::from_utf8(value.as_ref()) {
                if let Some(name) = self.parse_content_disposition(s) {
                    if name.len() <= 255 {
                        let path = path.join(name);
                        if path.exists() && !self.is_overwrite_existing_file() {
                            return GraphRsError::DownloadFileExists {
                                name: path.to_string_lossy().to_string(),
                            }
                            .as_err_res();
                        }
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
                let path = path.join(name);
                if path.exists() && !self.is_overwrite_existing_file() {
                    return GraphRsError::DownloadFileExists {
                        name: path.to_string_lossy().to_string(),
                    }
                    .as_err_res();
                }
                return self.finish((path, response));
            }
        }

        GraphRsError::DownloadFileName.as_err_res()
    }

    fn finish(&self, values: (PathBuf, Response)) -> GraphResult<PathBuf> {
        if self.extension.borrow().is_some() {
            values
                .0
                .with_extension(self.extension.replace(None).unwrap().as_str());
        }
        IoTools::copy(values)
    }
}
