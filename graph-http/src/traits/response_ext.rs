use crate::core::BodyRead;
use crate::internal::{copy_async, create_dir_async, FileConfig, RangeIter, UploadSession};
use crate::traits::UploadSessionLink;
use async_trait::async_trait;
use graph_error::download::AsyncDownloadError;
use graph_error::{GraphFailure, GraphResult, WithGraphErrorAsync};
use reqwest::header::HeaderMap;
use reqwest::Response;
use std::ffi::OsString;
use std::path::PathBuf;
use tokio::io::AsyncReadExt;

const MAX_FILE_NAME_LEN: usize = 255;

#[allow(clippy::single_char_pattern)]
fn parse_content_disposition(headers: &HeaderMap) -> Option<OsString> {
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

    async fn into_upload_session(
        self,
        reader: impl std::io::Read + Send,
    ) -> GraphResult<UploadSession>;

    async fn into_upload_session_async_read(
        self,
        reader: impl AsyncReadExt + Send + Unpin,
    ) -> GraphResult<UploadSession>;

    async fn download(self, download_config: &FileConfig) -> Result<PathBuf, AsyncDownloadError>;
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

        match result {
            Ok(response) => Some(response.with_graph_error().await),
            Err(err) => Some(Err(err)),
        }
    }

    /// Provide any [`std::io::Reader`] to create an upload session.
    /// The body of the reqwest::Response must be valid JSON with an
    /// uploadUrl field.
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
}

pub trait BodyExt<RHS = Self> {
    fn into_body(self) -> GraphResult<BodyRead>;
}

impl<U> BodyExt for &U
where
    U: serde::Serialize,
{
    fn into_body(self) -> GraphResult<BodyRead> {
        BodyRead::from_serialize(self)
    }
}

impl BodyExt for &FileConfig {
    fn into_body(self) -> GraphResult<BodyRead> {
        BodyRead::try_from(self)
    }
}
