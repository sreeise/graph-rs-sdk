use super::{AsyncDownloadError, BlockingDownloadError};
use crate::async_client::AsyncClient;
use crate::blocking_client::BlockingClient;
use crate::iotools;
use crate::url::GraphUrl;
use crate::{HttpClient, RequestClient, RequestType};
use graph_error::{WithGraphError, WithGraphErrorAsync};
use reqwest::header::HeaderMap;
use reqwest::Method;
use std::cell::RefCell;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

pub struct DownloadRequest {
    path: Option<PathBuf>,
    create_dir_all: bool,
    overwrite_existing_file: bool,
    file_name: Option<OsString>,
    extension: Option<String>,
}

impl DownloadRequest {
    pub fn new() -> DownloadRequest {
        DownloadRequest {
            path: None,
            create_dir_all: false,
            overwrite_existing_file: false,
            file_name: None,
            extension: None,
        }
    }
}

/// Provides an abstraction for downloading files.
pub struct DownloadClient<Client, Request> {
    client: Client,
    request: Request,
}

pub type BlockingDownload =
    DownloadClient<HttpClient<RefCell<BlockingClient>>, RefCell<DownloadRequest>>;
pub type AsyncDownload = DownloadClient<
    HttpClient<std::sync::Arc<tokio::sync::Mutex<AsyncClient>>>,
    std::sync::Arc<tokio::sync::Mutex<DownloadRequest>>,
>;

pub const MAX_FILE_NAME_LEN: usize = 255;

impl<Client, Request> DownloadClient<Client, Request> {
    fn parse_content_disposition(&self, headers: &HeaderMap) -> Option<OsString> {
        if let Some(value) = headers.get("content-disposition") {
            if let Ok(header) = std::str::from_utf8(value.as_ref()) {
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
            }
        }
        None
    }
}

impl BlockingDownload {
    pub fn new(client: BlockingClient) -> BlockingDownload {
        DownloadClient {
            request: RefCell::new(DownloadRequest::new()),
            client: HttpClient::from(client),
        }
    }

    pub fn create_dir_all(&self, value: bool) -> &Self {
        self.request.borrow_mut().create_dir_all = value;
        self
    }

    pub fn is_create_dir_all(&self) -> bool {
        self.request.borrow().create_dir_all
    }

    pub fn overwrite_existing_file(&self, value: bool) -> &Self {
        self.request.borrow_mut().overwrite_existing_file = value;
        self
    }

    pub fn is_overwrite_existing_file(&self) -> bool {
        self.request.borrow().overwrite_existing_file
    }

    pub fn rename(&self, value: OsString) -> &Self {
        self.request.borrow_mut().file_name = Some(value);
        self
    }

    pub fn set_extension(&self, value: &str) -> &Self {
        self.request.borrow_mut().extension = Some(value.into());
        self
    }

    pub fn set_dir<P: AsRef<Path>>(&self, path: P) -> &Self {
        self.request.borrow_mut().path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn directory(&self) -> Option<PathBuf> {
        self.request.borrow().path.clone()
    }

    pub fn file_name(&self) -> Option<OsString> {
        self.request.borrow().file_name.clone()
    }

    pub fn extension(&self) -> Option<String> {
        self.request.borrow().extension.clone()
    }

    pub fn url(&self) -> GraphUrl {
        self.client.url()
    }

    pub fn format(&self, format: &str) {
        self.client.set_request_type(RequestType::Redirect);
        self.client.url_mut(|url| url.format(format));
    }

    pub fn send(self) -> Result<PathBuf, BlockingDownloadError> {
        self.download()
    }

    fn download(self) -> Result<PathBuf, BlockingDownloadError> {
        let request = self.request.borrow();

        if request.path.is_none() {
            return Err(BlockingDownloadError::NoFileName);
        }

        // Create the directory if it does not exist.
        if request.create_dir_all {
            iotools::create_dir(request.path.as_ref().unwrap().as_path())?;
        } else if !request.path.as_ref().unwrap().exists() {
            return Err(BlockingDownloadError::TargetDoesNotExist(
                request.path.as_ref().unwrap().to_string_lossy().to_string(),
            ));
        }

        if self.client.request_type() == RequestType::Redirect {
            let response = self.client.build().send()?.with_graph_error()?;
            let mut client = self.client.client.borrow_mut();
            client.headers.clear();
            client.method = Method::GET;
            client.req_type = RequestType::Basic;
            client.url = GraphUrl::from(response.url().clone());
        }

        let response = self.client.build().send()?.with_graph_error()?;

        let path = {
            if let Some(name) = request
                .file_name
                .clone()
                .or_else(|| self.parse_content_disposition(response.headers()))
            {
                if name.len() > MAX_FILE_NAME_LEN {
                    return Err(BlockingDownloadError::FileNameTooLong);
                }
                request.path.as_ref().unwrap().join(name)
            } else {
                return Err(BlockingDownloadError::NoFileName);
            }
        };

        if let Some(ext) = request.extension.as_ref() {
            path.with_extension(ext.as_str());
        }

        if path.exists() && !self.is_overwrite_existing_file() {
            return Err(BlockingDownloadError::FileExists(
                path.to_string_lossy().to_string(),
            ));
        }

        Ok(iotools::copy(path, response)?)
    }
}

impl AsyncDownload {
    pub fn new_async(client: AsyncClient) -> AsyncDownload {
        DownloadClient {
            request: std::sync::Arc::new(tokio::sync::Mutex::new(DownloadRequest::new())),
            client: HttpClient::from(client),
        }
    }

    pub async fn create_dir_all(&self, value: bool) -> &Self {
        self.request.lock().await.create_dir_all = value;
        self
    }

    pub async fn is_create_dir_all(&self) -> bool {
        self.request.lock().await.create_dir_all
    }

    pub async fn overwrite_existing_file(&self, value: bool) -> &Self {
        self.request.lock().await.overwrite_existing_file = value;
        self
    }

    pub async fn is_overwrite_existing_file(&self) -> bool {
        self.request.lock().await.overwrite_existing_file
    }

    pub async fn rename(&self, value: OsString) -> &Self {
        self.request.lock().await.file_name = Some(value);
        self
    }

    pub async fn set_extension(&self, value: &str) -> &Self {
        self.request.lock().await.extension = Some(value.into());
        self
    }

    pub async fn set_dir<P: AsRef<Path>>(&self, path: P) -> &Self {
        self.request.lock().await.path = Some(path.as_ref().to_path_buf());
        self
    }

    pub async fn directory(&self) -> PathBuf {
        self.request.lock().await.path.as_ref().unwrap().clone()
    }

    pub async fn file_name(&self) -> Option<OsString> {
        self.request.lock().await.file_name.clone()
    }

    pub async fn extension(&self) -> Option<String> {
        self.request.lock().await.extension.clone()
    }

    pub async fn url(&self) -> GraphUrl {
        self.client.url()
    }

    pub async fn format(&self, format: &str) {
        self.client.set_request_type(RequestType::Redirect);
        self.client.url_mut(|url| {
            url.format(format);
        });
    }

    pub async fn send(self) -> Result<PathBuf, AsyncDownloadError> {
        self.download_async().await
    }

    async fn download_async(self) -> Result<PathBuf, AsyncDownloadError> {
        let request = self.request.lock().await;

        if request.path.is_none() {
            return Err(AsyncDownloadError::NoFileName);
        }
        // Create the directory if it does not exist.
        if request.create_dir_all {
            iotools::create_dir_async(request.path.as_ref().unwrap().as_path()).await?;
        } else if !request.path.as_ref().unwrap().exists() {
            return Err(AsyncDownloadError::TargetDoesNotExist(
                request.path.as_ref().unwrap().to_string_lossy().to_string(),
            ));
        }

        if self.client.request_type() == RequestType::Redirect {
            let response = self
                .client
                .build()
                .await
                .send()
                .await?
                .with_graph_error()
                .await?;

            let mut client = self.client.client.lock().await;
            (*client).headers.clear();
            (*client).method = Method::GET;
            (*client).req_type = RequestType::Basic;
            (*client).url = GraphUrl::from(response.url().clone());
        }

        let response = self
            .client
            .build()
            .await
            .send()
            .await?
            .with_graph_error()
            .await?;

        let path = {
            if let Some(name) = request
                .file_name
                .clone()
                .or_else(|| self.parse_content_disposition(response.headers()))
            {
                if name.len() > MAX_FILE_NAME_LEN {
                    return Err(AsyncDownloadError::FileNameTooLong);
                }
                request.path.as_ref().unwrap().join(name)
            } else {
                return Err(AsyncDownloadError::NoFileName);
            }
        };

        if let Some(ext) = request.extension.as_ref() {
            path.with_extension(ext.as_str());
        }

        if path.exists() && !request.overwrite_existing_file {
            return Err(AsyncDownloadError::FileExists(
                path.to_string_lossy().to_string(),
            ));
        }

        Ok(iotools::copy_async(path, response).await?)
    }

    pub async fn text(self) -> Result<String, AsyncDownloadError> {
        if self.client.request_type() == RequestType::Redirect {
            let response = self
                .client
                .build()
                .await
                .send()
                .await?
                .with_graph_error()
                .await?;

            let mut client = self.client.client.lock().await;
            (*client).headers.clear();
            (*client).method = Method::GET;
            (*client).req_type = RequestType::Basic;
            (*client).url = GraphUrl::from(response.url().clone());
        }

        let response = self
            .client
            .build()
            .await
            .send()
            .await?
            .with_graph_error()
            .await?
            .text()
            .await?;

        Ok(response)
    }
}
