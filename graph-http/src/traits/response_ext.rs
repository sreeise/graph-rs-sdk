use crate::internal::{
    copy_async, create_dir_async, FileConfig, HttpResponseBuilderExt, RangeIter, UploadSession,
};
use crate::traits::UploadSessionLink;
use async_trait::async_trait;
use graph_error::download::AsyncDownloadError;
use graph_error::{ErrorMessage, ErrorType, GraphFailure, GraphResult};
use reqwest::header::HeaderMap;
use reqwest::Response;
use std::ffi::OsString;
use std::path::PathBuf;
use tokio::io::AsyncReadExt;

pub(crate) const MAX_FILE_NAME_LEN: usize = 255;

#[allow(clippy::single_char_pattern)]
pub(crate) fn parse_content_disposition(headers: &HeaderMap) -> Option<OsString> {
    if let Some(value) = headers.get("content-disposition") {
        if let Ok(header) = std::str::from_utf8(value.as_ref()) {
            let mut v: Vec<&str> = header.split(';').collect();
            v.retain(|s| !s.is_empty());

            // The filename* indicates that the filename is encoded
            if let Some(value) = v.iter().find(|s| s.starts_with("filename*=utf-8''")) {
                let s = value.replace("filename*=utf-8''", "");
                if let Ok(s) = percent_encoding::percent_decode(s.as_bytes()).decode_utf8() {
                    return Some(OsString::from(s.to_string().replace('/', "-")));
                }
            }

            if let Some(value) = v.last() {
                if value.trim_start().starts_with("filename=") {
                    return Some(OsString::from(
                        value
                            .replace("\"", "")
                            .replace("filename=", "")
                            .replace('/', "-")
                            .trim(),
                    ));
                }
            }
        }
    }
    None
}

#[async_trait]
pub trait ResponseExt {
    async fn job_status(&self) -> Option<GraphResult<reqwest::Response>>;

    /// # Begin an upload session using any [`std::io::Reader`].<br>
    ///
    /// Converts the current request object into an upload session object for uploading large
    /// files to OneDrive or SharePoint.<br>
    ///
    /// This method takes a `reader` object that implements the [std::io::Read] and [Send] traits,
    /// and returns a [GraphResult] containing an [UploadSession] object.<br>
    ///
    /// The [UploadSession] object contains the upload URL for the file, as well as a [RangeIter] iterator
    /// that can be used to send the file contents to the server in multiple chunks (or "ranges").
    /// If the upload URL is not found in the response body, this method returns a `GraphFailure`
    /// with an error message indicating that no upload URL was found.<br>
    ///
    ///
    /// ## Requires reqwest::Response body to be valid JSON
    /// The body of the reqwest::Response must be valid JSON with an
    /// [uploadUrl] field.
    ///
    /// # Example
    /// ```rust,ignore
    /// use graph_rs_sdk::http::{AsyncIterator, ResponseExt};
    /// use graph_rs_sdk::*;
    ///
    /// static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
    ///
    /// // Put the path to your file and the file name itself that
    /// // you want to upload to one drive.
    /// static LOCAL_FILE_PATH: &str = "/path/to/file/file.txt";
    ///
    /// // Parent folder id of where to store this file.
    /// static DRIVE_PARENT_ID: &str = "PARENT_ID";
    ///
    /// // The conflict behavior can be one of: fail, rename, or replace.
    /// static CONFLICT_BEHAVIOR: &str = "rename";
    /// #[tokio::main]
    /// async fn main() -> GraphResult<()> {
    ///     let client = Graph::new(ACCESS_TOKEN);
    ///
    ///     let conflict_behavior = CONFLICT_BEHAVIOR.to_string()
    ///     let upload = serde_json::json!({
    ///         "@microsoft.graph.conflictBehavior": Some(conflict_behavior)
    ///     });
    ///
    ///     let response = client
    ///         .me()
    ///         .drive()
    ///         .item_by_path(PATH_IN_ONE_DRIVE)
    ///         .create_upload_session(&upload)
    ///         .send()
    ///         .await
    ///         .unwrap();
    ///
    ///     let file = std::fs::File::open(PATH_IN_ONE_DRIVE)?;
    ///
    ///     let mut iter = response.into_upload_session(file).await?;
    ///
    ///     while let Some(result) = iter.next().await {
    ///         let response = result?;
    ///         println!("{response:#?}");
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn into_upload_session(
        self,
        reader: impl std::io::Read + Send,
    ) -> GraphResult<UploadSession>;

    /// # Begin an upload session using any [tokio::io::AsyncReadExt].<br>
    ///
    /// Converts the current request object into an upload session object for uploading large
    /// files to OneDrive or SharePoint.<br>
    ///
    /// This method takes a `reader` object that implements the [tokio::io::AsyncReadExt], [Send], and [Unpin] traits,
    /// and returns a [GraphResult] containing an [UploadSession] object.<br>
    ///
    /// The [UploadSession] object contains the upload URL for the file, as well as a [RangeIter] iterator
    /// that can be used to send the file contents to the server in multiple chunks (or "ranges").
    /// If the upload URL is not found in the response body, this method returns a `GraphFailure`
    /// with an error message indicating that no upload URL was found.<br>
    ///
    ///
    /// ## Requires reqwest::Response body can be deserialized to valid JSON
    /// The body of the reqwest::Response must be valid JSON with an
    /// [uploadUrl] field.
    ///
    /// # Example
    /// ```rust,ignore
    /// use graph_rs_sdk::http::{AsyncIterator, ResponseExt};
    /// use graph_rs_sdk::*;
    ///
    /// static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
    ///
    /// // Put the path to your file and the file name itself that
    /// // you want to upload to one drive.
    /// static LOCAL_FILE_PATH: &str = "/path/to/file/file.txt";
    ///
    /// // Parent folder id of where to store this file.
    /// static DRIVE_PARENT_ID: &str = "PARENT_ID";
    ///
    /// // The conflict behavior can be one of: fail, rename, or replace.
    /// static CONFLICT_BEHAVIOR: &str = "rename";
    ///
    /// #[tokio::main]
    /// async fn main() -> GraphResult<()> {
    ///     let client = Graph::new(ACCESS_TOKEN);
    ///
    ///     let conflict_behavior = CONFLICT_BEHAVIOR.to_string()
    ///     let upload = serde_json::json!({
    ///         "@microsoft.graph.conflictBehavior": Some(conflict_behavior)
    ///     });
    ///
    ///     let response = client
    ///         .me()
    ///         .drive()
    ///         .item_by_path(PATH_IN_ONE_DRIVE)
    ///         .create_upload_session(&upload)
    ///         .send()
    ///         .await
    ///         .unwrap();
    ///
    ///     let file = tokio::fs::File::open(PATH_IN_ONE_DRIVE).await?;
    ///
    ///     let mut iter = response.into_upload_session_async_read(file).await?;
    ///
    ///     while let Some(result) = iter.next().await {
    ///         let response = result?;
    ///         println!("{response:#?}");
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn into_upload_session_async_read(
        self,
        reader: impl AsyncReadExt + Send + Unpin,
    ) -> GraphResult<UploadSession>;

    /// # Downloads the content of the HTTP response and saves it to a file.<br>
    ///
    /// This method takes a `file_config` object containing various parameters that control how the
    /// file is downloaded and saved. The `file_config` object includes the file path, file name,
    /// whether to create the directory recursively, whether to overwrite existing files, and the
    /// desired file extension.<br><br>
    ///
    /// If `create_dir_all` is set to true (default is true), this method will recursively create the directory
    /// at the path specified if it doesn't exist yet. If it is set to false and the target directory doesn't exist,
    /// this method will return an [AsyncDownloadError] with an error message indicating that the
    /// target directory does not exist.<br><br>
    ///
    /// The [`FileConfig::file_name`] parameter can be used to specify a custom file name for the downloaded file.
    /// If it is not provided, the method will attempt to parse the `Content-Disposition` header to extract the file name.
    /// If no file name can be obtained from the header, this method will return an [AsyncDownloadError::NoFileName]
    /// with an error message indicating that no file name was found.<br><br>
    ///
    /// If the [`FileConfig::extension`] parameter is set to a non-empty string,
    /// this method will set the file extension of the downloaded file to the specified value. <br><br>
    ///
    /// If the target file already exists and [`overwrite_existing_file`] is set to false,
    /// this method will return an [AsyncDownloadError::FileExists] with an error message
    /// indicating that the file already exists and cannot be overwritten. <br><br>
    ///
    /// If the file is downloaded and saved successfully, this method returns a [`http::Response<PathBuf>`] object
    /// containing the path to the downloaded file.
    ///
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use graph_rs_sdk::http::{BodyRead, FileConfig};
    /// use graph_rs_sdk::*;
    ///
    /// static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
    ///
    /// static ITEM_ID: &str = "ITEM_ID";
    ///
    /// static FORMAT: &str = "pdf";
    ///
    /// static DOWNLOAD_DIRECTORY: &str = "./examples";
    ///
    /// #[tokio::main]
    /// async fn main() -> GraphResult<()> {
    ///     let client = Graph::new(ACCESS_TOKEN);
    ///
    ///     let response = client
    ///         .me()
    ///         .drive()
    ///         .item(ITEM_ID)
    ///         .get_items_content()
    ///         .format(FORMAT)
    ///         .send()
    ///         .await?;
    ///
    ///     println!("{response:#?}");
    ///
    ///     let response2 = response.download(&FileConfig::new(DOWNLOAD_DIRECTORY))
    ///         .send()
    ///         .await?;
    ///
    ///     let path_buf = response2.body();
    ///     println!("{:#?}", path_buf.metadata());
    ///
    ///     Ok(())
    /// }
    /// ```
    /// <br><br>
    /// # Example format and rename
    ///
    /// ```rust,ignore
    /// use graph_rs_sdk::http::{BodyRead, FileConfig};
    /// use graph_rs_sdk::*;
    /// use std::ffi::OsStr;
    ///
    /// static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
    ///
    /// static ITEM_ID: &str = "ITEM_ID";
    ///
    /// static FORMAT: &str = "pdf";
    ///
    /// static DOWNLOAD_DIRECTORY: &str = "./examples";
    ///
    /// static FILE_NAME: &str = "new_file_name.pdf";
    ///
    /// #[tokio::main]
    /// async fn main() -> GraphResult<()> {
    ///     let client = Graph::new(ACCESS_TOKEN);
    ///
    ///     let response = client
    ///         .me()
    ///         .drive()
    ///         .item(ITEM_ID)
    ///         .get_items_content()
    ///         .format(FORMAT)
    ///         .send()
    ///         .await?;
    ///
    ///     println!("{response:#?}");
    ///
    ///     let file_config = FileConfig::new(DOWNLOAD_DIRECTORY)
    ///         .file_name(OsStr::new(FILE_NAME));
    ///
    ///     let response2 = response.download(file_config)
    ///         .send()
    ///         .await?;
    ///
    ///     let path_buf = response2.body();
    ///     println!("{:#?}", path_buf.metadata());
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn download(
        self,
        file_config: &FileConfig,
    ) -> Result<http::Response<PathBuf>, AsyncDownloadError>;

    /// If the response is a server error then Microsoft Graph will return
    /// an error in the response body. The [`ErrorMessage`] type maps to these
    /// errors and this method deserializes to this type.
    ///
    /// Microsoft Graph does not return this error message in all situations so it
    /// make sure to handle cases where the body could not be deserialized properly.
    /// ```rust,ignore
    /// let status = response.status();
    ///
    /// if status.is_server_error() || status.is_client_error() {
    ///     let error_message = response.into_error_message().await.unwrap();
    ///     println!("{error_message:#?}");
    ///
    ///     // This is the same thing as doing
    ///     let error_message: ErrorMessage = response.json().await.unwrap();
    /// }
    /// ```
    async fn into_graph_error_message(self) -> Result<ErrorMessage, reqwest::Error>;

    /// Microsoft Graph specific status code errors mapped from the response [StatusCode].
    /// Not all status codes map to a Microsoft Graph error.
    ///
    /// Use [`ErrorType::as_str`] to get a short description of the Microsoft Graph specific error.
    /// ```rust,ignore
    /// let error_type = response.graph_error_type().unwrap();
    /// println!("{:#?}", error_type.as_str());
    /// ```
    fn graph_error_type(&self) -> Option<ErrorType>;
}

#[async_trait]
impl ResponseExt for reqwest::Response {
    async fn job_status(&self) -> Option<GraphResult<Response>> {
        let url = self
            .headers()
            .get(reqwest::header::LOCATION)?
            .to_str()
            .ok()?;
        let result = reqwest::Client::new()
            .get(url)
            .send()
            .await
            .map_err(GraphFailure::from);

        Some(result)
    }

    /// # Begin an upload session using any [`std::io::Reader`].<br>
    ///
    /// Converts the current request object into an upload session object for uploading large
    /// files to OneDrive or SharePoint.<br>
    ///
    /// This method takes a `reader` object that implements the [std::io::Read] and [Send] traits,
    /// and returns a [GraphResult] containing an [UploadSession] object.<br>
    ///
    /// The [UploadSession] object contains the upload URL for the file, as well as a [RangeIter] iterator
    /// that can be used to send the file contents to the server in multiple chunks (or "ranges").
    /// If the upload URL is not found in the response body, this method returns a `GraphFailure`
    /// with an error message indicating that no upload URL was found.<br>
    ///
    ///
    /// ## Requires reqwest::Response body to be valid JSON
    /// The body of the reqwest::Response must be valid JSON with an
    /// [uploadUrl] field.
    ///
    /// # Example
    /// ```rust,ignore
    /// use graph_rs_sdk::http::{AsyncIterator, ResponseExt};
    /// use graph_rs_sdk::*;
    ///
    /// static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
    ///
    /// // Put the path to your file and the file name itself that
    /// // you want to upload to one drive.
    /// static LOCAL_FILE_PATH: &str = "/path/to/file/file.txt";
    ///
    /// // Parent folder id of where to store this file.
    /// static DRIVE_PARENT_ID: &str = "PARENT_ID";
    ///
    /// // The conflict behavior can be one of: fail, rename, or replace.
    /// static CONFLICT_BEHAVIOR: &str = "rename";
    /// #[tokio::main]
    /// async fn main() -> GraphResult<()> {
    ///     let client = Graph::new(ACCESS_TOKEN);
    ///
    ///     let conflict_behavior = CONFLICT_BEHAVIOR.to_string();
    ///     let upload = serde_json::json!({
    ///         "@microsoft.graph.conflictBehavior": Some(conflict_behavior)
    ///     });
    ///
    ///     let response = client
    ///         .me()
    ///         .drive()
    ///         .item_by_path(PATH_IN_ONE_DRIVE)
    ///         .create_upload_session(&upload)
    ///         .send()
    ///         .await
    ///         .unwrap();
    ///
    ///     let file = std::fs::File::open(PATH_IN_ONE_DRIVE)?;
    ///
    ///     let mut iter = response.into_upload_session(file).await?;
    ///
    ///     while let Some(result) = iter.next().await {
    ///         let response = result?;
    ///         println!("{response:#?}");
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn into_upload_session(
        self,
        reader: impl std::io::Read + Send,
    ) -> GraphResult<UploadSession> {
        let body: serde_json::Value = self.json().await?;
        let url = body
            .upload_session_link()
            .ok_or_else(|| GraphFailure::not_found("No uploadUrl found in response body"))?;

        let range_iter = RangeIter::from_reader(reader)?;
        Ok(UploadSession::new(
            reqwest::Url::parse(url.as_str())?,
            range_iter,
        ))
    }

    /// # Begin an upload session using any [tokio::io::AsyncReadExt].<br>
    ///
    /// Converts the current request object into an upload session object for uploading large
    /// files to OneDrive or SharePoint.<br>
    ///
    /// This method takes a `reader` object that implements the [tokio::io::AsyncReadExt], [Send], and [Unpin] traits,
    /// and returns a [GraphResult] containing an [UploadSession] object.<br>
    ///
    /// The [UploadSession] object contains the upload URL for the file, as well as a [RangeIter] iterator
    /// that can be used to send the file contents to the server in multiple chunks (or "ranges").
    /// If the upload URL is not found in the response body, this method returns a `GraphFailure`
    /// with an error message indicating that no upload URL was found.<br>
    ///
    ///
    /// ## Requires reqwest::Response body can be deserialized to valid JSON
    /// The body of the reqwest::Response must be valid JSON with an
    /// [uploadUrl] field.
    ///
    /// # Example
    /// ```rust,ignore
    /// use graph_rs_sdk::http::{AsyncIterator, ResponseExt};
    /// use graph_rs_sdk::*;
    ///
    /// static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
    ///
    /// // Put the path to your file and the file name itself that
    /// // you want to upload to one drive.
    /// static LOCAL_FILE_PATH: &str = "/path/to/file/file.txt";
    ///
    /// // Parent folder id of where to store this file.
    /// static DRIVE_PARENT_ID: &str = "PARENT_ID";
    ///
    /// // The conflict behavior can be one of: fail, rename, or replace.
    /// static CONFLICT_BEHAVIOR: &str = "rename";
    ///
    /// #[tokio::main]
    /// async fn main() -> GraphResult<()> {
    ///     let client = Graph::new(ACCESS_TOKEN);
    ///
    ///     let conflict_behavior = CONFLICT_BEHAVIOR.to_string();
    ///     let upload = serde_json::json!({
    ///         "@microsoft.graph.conflictBehavior": Some(conflict_behavior)
    ///     });
    ///
    ///     let response = client
    ///         .me()
    ///         .drive()
    ///         .item_by_path(PATH_IN_ONE_DRIVE)
    ///         .create_upload_session(&upload)
    ///         .send()
    ///         .await
    ///         .unwrap();
    ///
    ///     let file = tokio::fs::File::open(PATH_IN_ONE_DRIVE).await?;
    ///
    ///     let mut iter = response.into_upload_session_async_read(file).await?;
    ///
    ///     while let Some(result) = iter.next().await {
    ///         let response = result?;
    ///         println!("{response:#?}");
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn into_upload_session_async_read(
        self,
        reader: impl AsyncReadExt + Send + Unpin,
    ) -> GraphResult<UploadSession> {
        let body: serde_json::Value = self.json().await?;
        let url = body
            .upload_session_link()
            .ok_or_else(|| GraphFailure::not_found("No uploadUrl found in response body"))?;

        let range_iter = RangeIter::from_async_read(reader).await?;
        Ok(UploadSession::new(
            reqwest::Url::parse(url.as_str())?,
            range_iter,
        ))
    }

    /// # Downloads the content of the HTTP response and saves it to a file.<br>
    ///
    /// This method takes a `file_config` object containing various parameters that control how the
    /// file is downloaded and saved. The `file_config` object includes the file path, file name,
    /// whether to create the directory recursively, whether to overwrite existing files, and the
    /// desired file extension.<br><br>
    ///
    /// If `create_dir_all` is set to true, this method will create the directory at the specified
    /// path if it doesn't exist yet. If it is set to false and the target directory doesn't exist,
    /// this method will return an `AsyncDownloadError` with an error message indicating that the
    /// target directory does not exist.<br><br>
    ///
    /// The [`FileConfig::file_name`] parameter can be used to specify a custom file name for the downloaded file.
    /// If it is not provided, the method will attempt to parse the `Content-Disposition` header to extract the file name.
    /// If no file name can be obtained from the header, this method will return an [AsyncDownloadError::NoFileName]
    /// with an error message indicating that no file name was found.<br><br>
    ///
    /// If the [`FileConfig::extension`] parameter is set to a non-empty string,
    /// this method will set the file extension of the downloaded file to the specified value. <br><br>
    ///
    /// If the target file already exists and [`overwrite_existing_file`] is set to false,
    /// this method will return an [AsyncDownloadError::FileExists] with an error message
    /// indicating that the file already exists and cannot be overwritten. <br><br>
    ///
    /// If the file is downloaded and saved successfully, this method returns a [`http::Response<PathBuf>`] object
    /// containing the path to the downloaded file.
    ///
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use graph_rs_sdk::http::{BodyRead, FileConfig};
    /// use graph_rs_sdk::*;
    ///
    /// static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
    ///
    /// static ITEM_ID: &str = "ITEM_ID";
    ///
    /// static FORMAT: &str = "pdf";
    ///
    /// static DOWNLOAD_DIRECTORY: &str = "./examples";
    ///
    /// #[tokio::main]
    /// async fn main() -> GraphResult<()> {
    ///     let client = Graph::new(ACCESS_TOKEN);
    ///
    ///     let response = client
    ///         .me()
    ///         .drive()
    ///         .item(ITEM_ID)
    ///         .get_items_content()
    ///         .format(FORMAT)
    ///         .send()
    ///         .await?;
    ///
    ///     println!("{response:#?}");
    ///
    ///     let response2 = response.download(&FileConfig::new(DOWNLOAD_DIRECTORY))
    ///         .send()
    ///         .await?;
    ///
    ///     let path_buf = response2.body();
    ///     println!("{:#?}", path_buf.metadata());
    ///
    ///     Ok(())
    /// }
    /// ```
    /// <br><br>
    /// # Example format and rename
    ///
    /// ```rust,ignore
    /// use graph_rs_sdk::http::{BodyRead, FileConfig};
    /// use graph_rs_sdk::*;
    /// use std::ffi::OsStr;
    ///
    /// static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
    ///
    /// static ITEM_ID: &str = "ITEM_ID";
    ///
    /// static FORMAT: &str = "pdf";
    ///
    /// static DOWNLOAD_DIRECTORY: &str = "./examples";
    ///
    /// static FILE_NAME: &str = "new_file_name.pdf";
    ///
    /// #[tokio::main]
    /// async fn main() -> GraphResult<()> {
    ///     let client = Graph::new(ACCESS_TOKEN);
    ///
    ///     let response = client
    ///         .me()
    ///         .drive()
    ///         .item(ITEM_ID)
    ///         .get_items_content()
    ///         .format(FORMAT)
    ///         .send()
    ///         .await?;
    ///
    ///     println!("{response:#?}");
    ///
    ///     let file_config = FileConfig::new(DOWNLOAD_DIRECTORY)
    ///         .file_name(OsStr::new(FILE_NAME));
    ///
    ///     let response2 = response.download(file_config)
    ///         .send()
    ///         .await?;
    ///
    ///     let path_buf = response2.body();
    ///     println!("{:#?}", path_buf.metadata());
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn download(
        self,
        file_config: &FileConfig,
    ) -> Result<http::Response<PathBuf>, AsyncDownloadError> {
        let path = file_config.path.clone();
        let file_name = file_config.file_name.clone();
        let create_dir_all = file_config.create_directory_all;
        let overwrite_existing_file = file_config.overwrite_existing_file;
        let extension = file_config.extension.clone();

        if create_dir_all {
            create_dir_async(path.as_path()).await?;
        } else if !path.exists() {
            return Err(AsyncDownloadError::TargetDoesNotExist(
                path.to_string_lossy().to_string(),
            ));
        }

        let path = {
            if let Some(name) = file_name.or_else(|| parse_content_disposition(self.headers())) {
                if name.len() > MAX_FILE_NAME_LEN {
                    return Err(AsyncDownloadError::FileNameTooLong);
                }
                path.join(name)
            } else {
                return Err(AsyncDownloadError::NoFileName);
            }
        };

        if let Some(ext) = extension.as_ref() {
            path.with_extension(ext.as_os_str());
        }

        if path.exists() && !overwrite_existing_file {
            return Err(AsyncDownloadError::FileExists(
                path.to_string_lossy().to_string(),
            ));
        }

        let status = self.status();
        let url = self.url().clone();
        let _headers = self.headers().clone();
        let version = self.version();

        Ok(http::Response::builder()
            .url(url)
            .status(http::StatusCode::from(&status))
            .version(version)
            .body(copy_async(path, self).await?)?)
    }

    /// If the response is a server error then Microsoft Graph will return
    /// an error in the response body. The [`ErrorMessage`] type maps to these
    /// errors and this method deserializes to this type.
    ///
    /// Microsoft Graph does not return this error message in all situations so it
    /// make sure to handle cases where the body could not be deserialized properly.
    /// ```rust,ignore
    /// let status = response.status();
    ///
    /// if status.is_server_error() || status.is_client_error() {
    ///     let error_message = response.into_error_message().await.unwrap();
    ///     println!("{error_message:#?}");
    ///
    ///     // This is the same thing as doing
    ///     let error_message: ErrorMessage = response.json().await.unwrap();
    /// }
    /// ```
    async fn into_graph_error_message(self) -> Result<ErrorMessage, reqwest::Error> {
        self.json().await
    }

    /// Microsoft Graph specific status code errors mapped from the response [StatusCode].
    /// Not all status codes map to a Microsoft Graph error.
    ///
    /// Use [`ErrorType::as_str`] to get a short description of the Microsoft Graph specific error.
    /// ```rust,ignore
    /// let error_type = response.graph_error_type().unwrap();
    /// println!("{:#?}", error_type.as_str());
    /// ```
    fn graph_error_type(&self) -> Option<ErrorType> {
        let status = self.status();
        ErrorType::from_u16(status.as_u16())
    }
}
