use crate::blocking::UploadSessionBlocking;
use crate::internal::{
    copy, create_dir, parse_content_disposition, FileConfig, HttpResponseBuilderExt, RangeIter,
    UploadSessionLink, MAX_FILE_NAME_LEN,
};
use graph_error::download::BlockingDownloadError;
use graph_error::{ErrorMessage, ErrorType, GraphFailure, GraphResult};
use std::io::Read;
use std::path::PathBuf;

pub trait ResponseBlockingExt {
    fn job_status(&self) -> Option<GraphResult<reqwest::blocking::Response>>;

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
    /// ## Requires reqwest::blocking::Response body to be valid JSON
    /// The body of the reqwest::blocking::Response must be valid JSON with an
    /// [uploadUrl] field.
    ///
    /// # Example
    /// ```rust,ignore
    /// use graph_rs_sdk::http::{ResponseBlockingExt};
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
    ///         .unwrap();
    ///
    ///     let file = std::fs::File::open(PATH_IN_ONE_DRIVE)?;
    ///
    ///     let upload_session_task = response.into_upload_session(file)?;
    ///
    ///     for result in upload_session_task {
    ///         let response = result?;
    ///         println!("{:#?}", response);
    ///         let body: serde_json::Value = response.json().unwrap();
    ///         println!("{:#?}", body);
    ///     }
    ///
    ///
    ///     Ok(())
    /// }
    /// ```
    fn into_upload_session(
        self,
        reader: impl std::io::Read + Send,
    ) -> GraphResult<UploadSessionBlocking>;

    /// # Downloads the content of the HTTP response and saves it to a file.<br>
    ///
    /// This method takes a `file_config` object containing various parameters that control how the
    /// file is downloaded and saved. The `file_config` object includes the file path, file name,
    /// whether to create the directory recursively, whether to overwrite existing files, and the
    /// desired file extension.<br><br>
    ///
    /// If `create_dir_all` is set to true (default is true), this method will create the directory at the specified
    /// path if it doesn't exist yet. If it is set to false and the target directory doesn't exist,
    /// this method will return an [BlockingDownloadError] with an error message indicating that the
    /// target directory does not exist.<br><br>
    ///
    /// The [`FileConfig::file_name`] parameter can be used to specify a custom file name for the downloaded file.
    /// If it is not provided, the method will attempt to parse the `Content-Disposition` header to extract the file name.
    /// If no file name can be obtained from the header, this method will return an [BlockingDownloadError::NoFileName]
    /// with an error message indicating that no file name was found.<br><br>
    ///
    /// If the [`FileConfig::extension`] parameter is set to a non-empty string,
    /// this method will set the file extension of the downloaded file to the specified value. <br><br>
    ///
    /// If the target file already exists and [`overwrite_existing_file`] is set to false,
    /// this method will return an [BlockingDownloadError::FileExists] with an error message
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
    ///         .send()?;
    ///
    ///     println!("{response:#?}");
    ///
    ///     let response2 = response.download(&FileConfig::new(DOWNLOAD_DIRECTORY))
    ///         .send()?;
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
    ///         .send()?;
    ///
    ///     println!("{response:#?}");
    ///
    ///     let file_config = FileConfig::new(DOWNLOAD_DIRECTORY)
    ///         .file_name(OsStr::new(FILE_NAME));
    ///
    ///     let response2 = response.download(file_config)
    ///         .send()?;
    ///
    ///     let path_buf = response2.body();
    ///     println!("{:#?}", path_buf.metadata());
    ///
    ///     Ok(())
    /// }
    /// ```
    fn download(
        self,
        file_config: &FileConfig,
    ) -> Result<http::Response<PathBuf>, BlockingDownloadError>;

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
    ///     let error_message = response.into_error_message().unwrap();
    ///     println!("{error_message:#?}");
    ///
    ///     // This is the same thing as doing
    ///     let error_message: ErrorMessage = response.json().unwrap();
    /// }
    /// ```
    fn into_graph_error_message(self) -> Result<ErrorMessage, reqwest::Error>;

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

impl ResponseBlockingExt for reqwest::blocking::Response {
    fn job_status(&self) -> Option<GraphResult<reqwest::blocking::Response>> {
        let url = self
            .headers()
            .get(reqwest::header::LOCATION)?
            .to_str()
            .ok()?;
        let result = reqwest::blocking::Client::new()
            .get(url)
            .send()
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
    /// ## Requires reqwest::blocking::Response body to be valid JSON
    /// The body of the reqwest::blocking::Response must be valid JSON with an
    /// [uploadUrl] field.
    ///
    /// # Example
    /// ```rust,ignore
    /// use graph_rs_sdk::http::{ResponseBlockingExt};
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
    ///         .unwrap();
    ///
    ///     let file = std::fs::File::open(PATH_IN_ONE_DRIVE)?;
    ///
    ///     let upload_session_task = response.into_upload_session(file)?;
    ///
    ///     for result in upload_session_task {
    ///         let response = result?;
    ///         println!("{:#?}", response);
    ///         let body: serde_json::Value = response.json().unwrap();
    ///         println!("{:#?}", body);
    ///     }
    ///
    ///
    ///     Ok(())
    /// }
    /// ```
    fn into_upload_session(self, reader: impl Read + Send) -> GraphResult<UploadSessionBlocking> {
        let body: serde_json::Value = self.json()?;
        let url = body
            .upload_session_link()
            .ok_or_else(|| GraphFailure::not_found("No uploadUrl found in response body"))?;

        let range_iter = RangeIter::from_reader(reader)?;
        Ok(UploadSessionBlocking::new(
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
    /// If `create_dir_all` is set to true (default is true), this method will create the directory at the specified
    /// path if it doesn't exist yet. If it is set to false and the target directory doesn't exist,
    /// this method will return an [BlockingDownloadError] with an error message indicating that the
    /// target directory does not exist.<br><br>
    ///
    /// The [`FileConfig::file_name`] parameter can be used to specify a custom file name for the downloaded file.
    /// If it is not provided, the method will attempt to parse the `Content-Disposition` header to extract the file name.
    /// If no file name can be obtained from the header, this method will return an [BlockingDownloadError::NoFileName]
    /// with an error message indicating that no file name was found.<br><br>
    ///
    /// If the [`FileConfig::extension`] parameter is set to a non-empty string,
    /// this method will set the file extension of the downloaded file to the specified value. <br><br>
    ///
    /// If the target file already exists and [`overwrite_existing_file`] is set to false,
    /// this method will return an [BlockingDownloadError::FileExists] with an error message
    /// indicating that the file already exists and cannot be overwritten. <br><br>
    ///
    /// If the file is downloaded and saved successfully, this method returns a [`http::Response<PathBuf>`] object
    /// containing the path to the downloaded file.
    ///
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use graph_rs_sdk::http::{BodyRead, FileConfig, ResponseBlockingExt};
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
    ///         .send()?;
    ///
    ///     println!("{response:#?}");
    ///
    ///     let response2 = response.download(&FileConfig::new(DOWNLOAD_DIRECTORY))
    ///         .send()?;
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
    ///         .send()?;
    ///
    ///     println!("{response:#?}");
    ///
    ///     let file_config = FileConfig::new(DOWNLOAD_DIRECTORY)
    ///         .file_name(OsStr::new(FILE_NAME));
    ///
    ///     let response2 = response.download(file_config)
    ///         .send()?;
    ///
    ///     let path_buf = response2.body();
    ///     println!("{:#?}", path_buf.metadata());
    ///
    ///     Ok(())
    /// }
    /// ```
    fn download(
        self,
        file_config: &FileConfig,
    ) -> Result<http::Response<PathBuf>, BlockingDownloadError> {
        let path = file_config.path.clone();
        let file_name = file_config.file_name.clone();
        let create_dir_all = file_config.create_directory_all;
        let overwrite_existing_file = file_config.overwrite_existing_file;
        let extension = file_config.extension.clone();

        if create_dir_all {
            create_dir(path.as_path())?;
        } else if !path.exists() {
            return Err(BlockingDownloadError::TargetDoesNotExist(
                path.to_string_lossy().to_string(),
            ));
        }

        let path = {
            if let Some(name) = file_name.or_else(|| parse_content_disposition(self.headers())) {
                if name.len() > MAX_FILE_NAME_LEN {
                    return Err(BlockingDownloadError::FileNameTooLong);
                }
                path.join(name)
            } else {
                return Err(BlockingDownloadError::NoFileName);
            }
        };

        if let Some(ext) = extension.as_ref() {
            path.with_extension(ext.as_os_str());
        }

        if path.exists() && !overwrite_existing_file {
            return Err(BlockingDownloadError::FileExists(
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
            .body(copy(path, self)?)?)
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
    ///     let error_message = response.into_error_message().unwrap();
    ///     println!("{error_message:#?}");
    ///
    ///     // This is the same thing as doing
    ///     let error_message: ErrorMessage = response.json().unwrap();
    /// }
    /// ```
    fn into_graph_error_message(self) -> Result<ErrorMessage, reqwest::Error> {
        self.json()
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
