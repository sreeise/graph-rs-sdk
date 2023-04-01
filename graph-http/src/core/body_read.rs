use crate::api_impl::{BodyExt, FileConfig};
use bytes::{Buf, BytesMut};
use graph_error::{GraphFailure, GraphResult};
use reqwest::Body;
use std::fs::File;
use std::io::{ErrorKind, Read};

#[derive(Clone)]
pub struct BodyRead {
    buf: String,
}

impl BodyRead {
    pub fn new(buf: String) -> BodyRead {
        BodyRead { buf }
    }

    pub fn from_serialize<T: serde::Serialize>(body: &T) -> GraphResult<BodyRead> {
        let body = serde_json::to_string(body)?;
        Ok(BodyRead::new(body))
    }

    pub fn from_reader<T: std::io::Read>(mut reader: T) -> GraphResult<BodyRead> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        Ok(BodyRead::new(buf))
    }

    pub async fn from_async_read<T: tokio::io::AsyncReadExt + std::marker::Unpin>(
        mut reader: T,
    ) -> GraphResult<BodyRead> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf).await?;
        Ok(BodyRead::new(buf))
    }

    pub fn from_file_config(file_config: &FileConfig) -> GraphResult<BodyRead> {
        let mut file = File::open(file_config.path.as_path())?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(BodyRead::new(buf))
    }

    pub fn from_body(body: Body) -> GraphResult<BodyRead> {
        let bytes_u8 = body.as_bytes().ok_or(std::io::Error::new(
            ErrorKind::InvalidData,
            "Unable to convert reqwest::Body to &[u8]",
        ))?;
        let body = serde_json::from_reader(bytes_u8)?;
        Ok(BodyRead::new(body))
    }

    pub fn from_blocking_body(body: reqwest::blocking::Body) -> GraphResult<BodyRead> {
        let bytes_u8 = body.as_bytes().ok_or(std::io::Error::new(
            ErrorKind::InvalidData,
            "Unable to convert reqwest::Body to &[u8]",
        ))?;
        let body = serde_json::from_reader(bytes_u8)?;
        Ok(BodyRead::new(body))
    }

    pub fn from_file(file: std::fs::File) -> GraphResult<BodyRead> {
        BodyRead::from_reader(file)
    }

    pub async fn from_tokio_file(file: tokio::fs::File) -> GraphResult<BodyRead> {
        BodyRead::from_async_read(file).await
    }
}

impl TryFrom<BytesMut> for BodyRead {
    type Error = GraphFailure;

    fn try_from(bytes_mut: BytesMut) -> Result<Self, Self::Error> {
        BodyRead::from_reader(bytes_mut.reader())
    }
}

impl TryFrom<bytes::Bytes> for BodyRead {
    type Error = GraphFailure;

    fn try_from(bytes: bytes::Bytes) -> Result<Self, Self::Error> {
        BodyRead::from_reader(bytes.reader())
    }
}

impl From<BodyRead> for reqwest::Body {
    fn from(upload: BodyRead) -> Self {
        reqwest::Body::from(upload.buf)
    }
}

impl From<&BodyRead> for reqwest::Body {
    fn from(upload: &BodyRead) -> Self {
        reqwest::Body::from(upload.buf.clone())
    }
}

impl From<BodyRead> for reqwest::blocking::Body {
    fn from(upload: BodyRead) -> Self {
        reqwest::blocking::Body::from(upload.buf)
    }
}

impl From<&BodyRead> for reqwest::blocking::Body {
    fn from(upload: &BodyRead) -> Self {
        reqwest::blocking::Body::from(upload.buf.clone())
    }
}

impl BodyExt for BodyRead {
    fn as_body(&self) -> GraphResult<BodyRead> {
        Ok(self.clone())
    }
}
