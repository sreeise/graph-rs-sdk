use crate::internal::{
    copy, create_dir, parse_content_disposition, FileConfig, HttpResponseBuilderExt,
    MAX_FILE_NAME_LEN,
};
use graph_error::download::BlockingDownloadError;
use graph_error::{GraphFailure, GraphResult, WithGraphError};
use std::path::PathBuf;

pub trait ResponseBlockingExt {
    fn job_status(&self) -> Option<GraphResult<reqwest::blocking::Response>>;

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
    /// use graph_rs_sdk::prelude::*;
    ///
    /// static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
    ///
    /// #[tokio::main]
    /// async fn main() -> GraphResult<()> {
    ///     let client = Graph::new(ACCESS_TOKEN);
    ///
    ///     let path_buf = client
    ///         .me()
    ///         .drive()
    ///         .item(ITEM_ID)
    ///         .get_items_content()
    ///         .format(format)
    ///         .download(&FileConfig::new("./examples/example_files")
    ///                 .create_directories(true))
    ///         .await?;
    ///
    ///     println!("{response:#?}");
    ///
    ///     let path_buf = response.body();
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
    /// use graph_rs_sdk::prelude::*;
    ///
    /// static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
    ///
    /// #[tokio::main]
    /// async fn main() -> GraphResult<()> {
    ///     let client = Graph::new(ACCESS_TOKEN);
    ///
    ///     let path_buf = client
    ///         .me()
    ///         .drive()
    ///         .item(ITEM_ID)
    ///         .get_items_content()
    ///         .format("pdf")
    ///         .download(
    ///             &FileConfig::new("./examples/example_files")
    ///                 .create_directories(true)
    ///                 .file_name(OsStr::new("file.pdf")),
    ///         )
    ///         .await?;
    ///
    ///     println!("{response:#?}");
    ///
    ///     let path_buf = response.body();
    ///     println!("{:#?}", path_buf.metadata());
    ///
    ///     Ok(())
    /// }
    /// ```
    fn download(
        self,
        file_config: &FileConfig,
    ) -> Result<http::Response<PathBuf>, BlockingDownloadError>;
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

        match result {
            Ok(response) => Some(response.with_graph_error()),
            Err(err) => Some(Err(err)),
        }
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
    /// If the file is downloaded and saved successfully, this method returns a [`PathBuf`] object
    /// containing the path to the downloaded file.
    ///
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use graph_rs_sdk::http::FileConfig;
    /// use std::ffi::OsStr;
    /// use graph_rs_sdk::prelude::*;
    ///
    /// static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
    ///
    /// fn main() -> GraphResult<()> {
    ///     let client = Graph::new(ACCESS_TOKEN);
    ///
    ///     let path_buf = client
    ///         .me()
    ///         .drive()
    ///         .item(ITEM_ID)
    ///         .get_items_content()
    ///         .into_blocking()
    ///         .format(format)
    ///         .download(&FileConfig::new("./examples/example_files")
    ///                 .create_directories(true))?;
    ///
    ///     println!("{response:#?}");
    ///
    ///     let path_buf = response.body();
    ///     println!("{:#?}", path_buf.metadata());
    ///
    ///     Ok(())
    /// }
    /// ```
    /// <br><br>
    /// # Example format and rename
    ///
    /// ```rust,ignore
    /// use graph_rs_sdk::http::FileConfig;
    /// use std::ffi::OsStr;
    /// use graph_rs_sdk::prelude::*;
    ///
    /// static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
    ///
    /// fn main() -> GraphResult<()> {
    ///
    /// let client = Graph::new(ACCESS_TOKEN);
    ///
    ///  let response = client
    ///     .me()
    ///     .drive()
    ///     .item(ITEM_ID)
    ///     .get_items_content()
    ///     .into_blocking()
    ///     .format("pdf")
    ///     .download(&FileConfig::new("./examples/example_files")
    ///         .create_directories(true)
    ///         .file_name(OsStr::new("file.pdf")
    ///     ))?;
    ///
    ///     println!("{response:#?}");
    ///
    ///     let path_buf = response.body();
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
}

/*
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
   /// If the file is downloaded and saved successfully, this method returns a [`PathBuf`] object
   /// containing the path to the downloaded file.
   ///
   ///
   /// # Example
   ///
   /// ```rust,ignore
   /// use graph_rs_sdk::http::{BodyRead, FileConfig};
   /// use graph_rs_sdk::prelude::*;
   ///
   /// static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
   ///
   /// #[tokio::main]
   /// async fn main() -> GraphResult<()> {
   ///     let client = Graph::new(ACCESS_TOKEN);
   ///
   ///     let path_buf = client
   ///         .me()
   ///         .drive()
   ///         .item(ITEM_ID)
   ///         .get_items_content()
   ///         .format(format)
   ///         .download(&FileConfig::new("./examples/example_files")
   ///                 .create_directories(true))
   ///         .await?;
   ///
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
   /// use graph_rs_sdk::prelude::*;
   ///
   /// static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
   ///
   /// #[tokio::main]
   /// async fn main() -> GraphResult<()> {
   ///     let client = Graph::new(ACCESS_TOKEN);
   ///
   ///     let path_buf = client
   ///         .me()
   ///         .drive()
   ///         .item(ITEM_ID)
   ///         .get_items_content()
   ///         .format("pdf")
   ///         .download(
   ///             &FileConfig::new("./examples/example_files")
   ///                 .create_directories(true)
   ///                 .file_name(OsStr::new("file.pdf")),
   ///         )
   ///         .await?;
   ///
   ///     println!("{:#?}", path_buf.metadata());
   ///
   ///     Ok(())
   /// }
   /// ```
   async fn download(self, file_config: &FileConfig) -> Result<PathBuf, AsyncDownloadError> {
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

       Ok(copy_async(path, self).await?)
   }
*/
