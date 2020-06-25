use crate::graph_error::AsRes;
use crate::http::{AsyncClient, BlockingClient, GraphRequestType, IoTools, RequestClient};
use crate::url::GraphUrl;
use graph_error::{GraphFailure, GraphResult, GraphRsError};
use reqwest::{blocking::Response, Method};
use std::cell::{Cell, Ref, RefCell};
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

/// Provides an abstraction for downloading files.
pub struct DownloadClient<Client> {
    path: RefCell<PathBuf>,
    create_dir_all: Cell<bool>,
    overwrite_existing_file: Cell<bool>,
    file_name: RefCell<Option<OsString>>,
    extension: RefCell<Option<String>>,
    client: RefCell<Client>,
}

impl<Client> DownloadClient<Client> {
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
                    value.replace("\"", "").replace("filename=", ""),
                ));
            }
        }
        None
    }
}

pub type BlockingDownload = DownloadClient<BlockingClient>;
pub type AsyncDownload = DownloadClient<AsyncClient>;

impl BlockingDownload {
    pub fn new(client: BlockingClient) -> DownloadClient<BlockingClient> {
        let path = client.download_dir.clone().unwrap();
        DownloadClient {
            path: RefCell::new(path),
            create_dir_all: Cell::new(true),
            overwrite_existing_file: Cell::new(false),
            file_name: RefCell::new(None),
            extension: RefCell::new(None),
            client: RefCell::new(client),
        }
    }

    pub fn url(&self) -> GraphUrl {
        self.client.borrow().url().clone()
    }

    pub fn format(&self, format: &str) {
        let mut client = self.client.borrow_mut();
        client.set_request_type(GraphRequestType::Redirect);
        client.as_mut().format(format);
    }

    pub fn send(&self) -> GraphResult<PathBuf> {
        self.download()
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

        if self.client.borrow().request_type() == GraphRequestType::Redirect {
            let mut response = self.client.borrow_mut().build().send()?;

            if let Some(err) = GraphFailure::from_response(&mut response) {
                return Err(err);
            }
            self.client
                .borrow_mut()
                .set_method(Method::GET)
                .set_request_type(GraphRequestType::Basic)
                .set_url(GraphUrl::from(response.url().clone()));
        }

        let mut response = self.client.borrow_mut().build().send()?;

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
                        println!("filename: {:#?}", name);
                        println!("before joined path: {:#?}", path);
                        let path = path.join(name);
                        println!("joined path: {:#?}", path);

                        // let path = path.join(name);
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

impl AsyncDownload {
    pub fn new_async(client: AsyncClient) -> AsyncDownload {
        let path = client.download_dir.clone().unwrap();
        AsyncDownload {
            path: RefCell::new(path),
            create_dir_all: Cell::new(true),
            overwrite_existing_file: Cell::new(false),
            file_name: RefCell::new(None),
            extension: RefCell::new(None),
            client: RefCell::new(client),
        }
    }

    pub fn url(&self) -> GraphUrl {
        self.client.borrow_mut().url().clone()
    }

    pub fn format(&self, format: &str) {
        let mut client = self.client.borrow_mut();
        client.set_request_type(GraphRequestType::Redirect);
        client.as_mut().format(format);
    }

    pub async fn send(&self) -> GraphResult<PathBuf> {
        self.download_async().await
    }

    async fn download_async(&self) -> GraphResult<PathBuf> {
        let path = self.path.replace(PathBuf::new());

        // Create the directory if it does not exist.
        if self.is_create_dir_all() {
            IoTools::create_dir(path.as_path())?;
        } else if !path.exists() {
            let dir = path.to_string_lossy().to_string();
            return GraphRsError::DownloadDirNoExists { dir }.as_err_res();
        }

        if self.client.borrow().request_type() == GraphRequestType::Redirect {
            let mut response = self.client.borrow_mut().build().send().await?;

            if let Some(err) = GraphFailure::from_async_response(&mut response) {
                return Err(err);
            }
            self.client
                .borrow_mut()
                .set_method(Method::GET)
                .set_request_type(GraphRequestType::Basic)
                .set_url(GraphUrl::from(response.url().clone()));
        }

        let mut response = self.client.borrow_mut().build().send().await?;

        if let Some(err) = GraphFailure::from_async_response(&mut response) {
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
                return self.finish((path, response)).await;
            }
        }

        // The content-disposition header, if available, may include the
        // filename either in its normal form or percent encoded.
        if let Some(value) = response.headers().get("content-disposition") {
            if let Ok(s) = std::str::from_utf8(value.as_ref()) {
                if let Some(name) = self.parse_content_disposition(s) {
                    if name.len() <= 255 {
                        println!("filename: {:#?}", name);
                        println!("before joined path: {:#?}", path);
                        let path = path.join(name);
                        println!("joined path: {:#?}", path);
                        if path.exists() && !self.is_overwrite_existing_file() {
                            return GraphRsError::DownloadFileExists {
                                name: path.to_string_lossy().to_string(),
                            }
                            .as_err_res();
                        }
                        return self.finish((path, response)).await;
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
                return self.finish((path, response)).await;
            }
        }

        GraphRsError::DownloadFileName.as_err_res()
    }

    async fn finish(&self, values: (PathBuf, reqwest::Response)) -> GraphResult<PathBuf> {
        if self.extension.borrow().is_some() {
            values
                .0
                .with_extension(self.extension.replace(None).unwrap().as_str());
        }
        IoTools::copy_async(values).await
    }
}
