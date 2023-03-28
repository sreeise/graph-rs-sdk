use crate::traits::{AsBytesMut, AsBytesMutWrapped};
use crate::{iotools, FileConfig, RangeIter, UploadSession};
use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use graph_error::download::AsyncDownloadError;
use graph_error::{GraphFailure, GraphResult};
use reqwest::header::HeaderMap;
use reqwest::{Body, Response};
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

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

    async fn into_upload_session<T: Read + Send>(self, reader: T) -> GraphResult<UploadSession>;

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
        Some(
            reqwest::Client::new()
                .get(url)
                .send()
                .await
                .map_err(GraphFailure::from),
        )
    }

    async fn into_upload_session<T: Read + Send>(self, reader: T) -> GraphResult<UploadSession> {
        let body: serde_json::Value = self.json().await?;
        let url = body["uploadUrl"]
            .as_str()
            .ok_or_else(|| GraphFailure::not_found("no \"uploadUrl\""))?;

        let range_iter = RangeIter::from_reader(reader)?;
        Ok(UploadSession::new(reqwest::Url::parse(url)?, range_iter))
    }

    async fn download(self, download_config: &FileConfig) -> Result<PathBuf, AsyncDownloadError> {
        let path = download_config.path.clone();
        let file_name = download_config.file_name.clone();
        let create_dir_all = download_config.create_directory_all.clone();
        let overwrite_existing_file = download_config.overwrite_existing_file;
        let extension = download_config.extension.clone();

        if create_dir_all {
            iotools::create_dir_async(path.as_path()).await?;
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

        Ok(iotools::copy_async(path, self).await?)
    }
}

pub trait BodyExt<RHS = Self> {
    fn as_body(&self) -> GraphResult<reqwest::Body>;
}

impl<T: serde::Serialize> BodyExt for T {
    fn as_body(&self) -> GraphResult<Body> {
        let body = serde_json::to_string(&self).map_err(GraphFailure::from)?;
        Ok(reqwest::Body::from(body))
    }
}

impl BodyExt for FileConfig {
    fn as_body(&self) -> GraphResult<Body> {
        let bytes_mut = self.as_bytes_mut()?;
        Ok(reqwest::Body::from(bytes_mut.to_vec()))
    }
}
