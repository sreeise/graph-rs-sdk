use crate::{iotools, DownloadConfig};
use async_trait::async_trait;
use graph_error::download::AsyncDownloadError;
use reqwest::header::HeaderMap;
use std::ffi::OsString;
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
pub trait DownloadTask {
    async fn download(self, download_config: DownloadConfig)
        -> Result<PathBuf, AsyncDownloadError>;
}

#[async_trait]
impl DownloadTask for reqwest::Response {
    async fn download(
        self,
        download_config: DownloadConfig,
    ) -> Result<PathBuf, AsyncDownloadError> {
        let path = download_config.path;
        let file_name = download_config.file_name;
        let create_dir_all = download_config.create_directory_all;
        let overwrite_existing_file = download_config.overwrite_existing_file;
        let extension = download_config.extension;

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
            path.with_extension(ext.as_str());
        }

        if path.exists() && !overwrite_existing_file {
            return Err(AsyncDownloadError::FileExists(
                path.to_string_lossy().to_string(),
            ));
        }

        Ok(iotools::copy_async(path, self).await?)
    }
}
