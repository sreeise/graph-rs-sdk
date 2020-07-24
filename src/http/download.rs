use crate::graph_error::AsRes;
use crate::http::{
    AsyncClient, BlockingClient, GraphRequestType, HttpClient, IoTools, RequestAttribute,
    RequestClient,
};
use crate::url::GraphUrl;
use graph_error::{GraphFailure, GraphResult, GraphRsError};
use reqwest::header::HeaderMap;
use reqwest::Method;
use std::cell::RefCell;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

pub struct DownloadRequest {
    path: PathBuf,
    create_dir_all: bool,
    overwrite_existing_file: bool,
    file_name: Option<OsString>,
    extension: Option<String>,
}

impl DownloadRequest {
    pub fn new(path: PathBuf) -> DownloadRequest {
        DownloadRequest {
            path,
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

    fn check_file_name_length(&self, name: &OsString) -> GraphResult<()> {
        if name.len() > 255 {
            return GraphRsError::DownloadFileName.as_err_res();
        }
        Ok(())
    }
}

impl BlockingDownload {
    pub fn new(client: BlockingClient) -> BlockingDownload {
        let path = client.download_dir.clone().unwrap();
        DownloadClient {
            request: RefCell::new(DownloadRequest::new(path)),
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
        self.request.borrow_mut().path = path.as_ref().to_path_buf();
        self
    }

    pub fn directory(&self) -> PathBuf {
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
        self.client.set_request_type(GraphRequestType::Redirect);
        self.client.url_mut(|url| url.format(format));
    }

    fn check_existing_file(&self, path: &PathBuf) -> GraphResult<()> {
        if path.exists() && !self.is_overwrite_existing_file() {
            return GraphRsError::DownloadFileExists {
                name: path.to_string_lossy().to_string(),
            }
            .as_err_res();
        }
        Ok(())
    }

    pub fn send(self) -> GraphResult<PathBuf> {
        self.download()
    }

    fn download(self) -> GraphResult<PathBuf> {
        let request = self.request.borrow();

        // Create the directory if it does not exist.
        if request.create_dir_all.eq(&true) {
            IoTools::create_dir(request.path.as_path())?;
        } else if !request.path.exists() {
            let dir = request.path.to_string_lossy().to_string();
            return GraphRsError::DownloadDirNoExists { dir }.as_err_res();
        }

        if self.client.request_type().eq(&GraphRequestType::Redirect) {
            let response = self.client.build().send()?;

            if let Some(err) = GraphFailure::from_response(&response) {
                return Err(err);
            }

            self.client.set_request(vec![
                RequestAttribute::ClearHeaders,
                RequestAttribute::Method(Method::GET),
                RequestAttribute::RequestType(GraphRequestType::Basic),
                RequestAttribute::Url(GraphUrl::from(response.url().clone())),
            ])?;
        }

        let response = self.client.build().send()?;
        if let Some(err) = GraphFailure::from_response(&response) {
            return Err(err);
        }

        let path = {
            if let Some(name) = request.file_name.as_ref() {
                self.check_file_name_length(name)?;
                request.path.join(name)
            } else if let Some(name) = self.parse_content_disposition(response.headers()) {
                self.check_file_name_length(&name)?;
                request.path.join(name)
            } else {
                return GraphRsError::DownloadFileName.as_err_res();
            }
        };

        if let Some(ext) = request.extension.as_ref() {
            path.with_extension(ext.as_str());
        }

        self.check_existing_file(&path)?;
        IoTools::copy(path, response)
    }
}

impl AsyncDownload {
    pub fn new_async(client: AsyncClient) -> AsyncDownload {
        let path = client.download_dir.clone().unwrap();
        DownloadClient {
            request: std::sync::Arc::new(tokio::sync::Mutex::new(DownloadRequest::new(path))),
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
        self.request.lock().await.path = path.as_ref().to_path_buf();
        self
    }

    pub async fn directory(&self) -> PathBuf {
        self.request.lock().await.path.clone()
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
        self.client.set_request_type(GraphRequestType::Redirect);
        self.client.url_mut(|url| {
            url.format(format);
        });
    }

    async fn check_existing_file(&self, path: &PathBuf) -> GraphResult<()> {
        if path.exists() && !self.is_overwrite_existing_file().await {
            return GraphRsError::DownloadFileExists {
                name: path.to_string_lossy().to_string(),
            }
            .as_err_res();
        }
        Ok(())
    }

    pub async fn send(self) -> GraphResult<PathBuf> {
        self.download_async().await
    }

    async fn download_async(self) -> GraphResult<PathBuf> {
        let request = self.request.lock().await;
        // Create the directory if it does not exist.
        if request.create_dir_all.eq(&true) {
            IoTools::create_dir_async(request.path.as_path()).await?;
        } else if !request.path.exists() {
            let dir = request.path.to_string_lossy().to_string();
            return GraphRsError::DownloadDirNoExists { dir }.as_err_res();
        }

        if self.client.request_type().eq(&GraphRequestType::Redirect) {
            let response = self.client.build().await.send().await?;
            if let Some(err) = GraphFailure::from_async_response(&response) {
                return Err(err);
            }
            self.client.set_request(vec![
                RequestAttribute::ClearHeaders,
                RequestAttribute::Method(Method::GET),
                RequestAttribute::RequestType(GraphRequestType::Basic),
                RequestAttribute::Url(GraphUrl::from(response.url().clone())),
            ])?;
        }

        let response = self.client.build().await.send().await?;
        if let Some(err) = GraphFailure::from_async_response(&response) {
            return Err(err);
        }

        let path = {
            if let Some(name) = request.file_name.as_ref() {
                self.check_file_name_length(name)?;
                request.path.join(name)
            } else if let Some(name) = self.parse_content_disposition(response.headers()) {
                self.check_file_name_length(&name)?;
                request.path.join(name)
            } else {
                return GraphRsError::DownloadFileName.as_err_res();
            }
        };

        if let Some(ext) = request.extension.as_ref() {
            path.with_extension(ext.as_str());
        }

        self.check_existing_file(&path).await?;
        IoTools::copy_async(path, response).await
    }
}
