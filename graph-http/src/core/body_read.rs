use crate::api_impl::{BodyExt, FileConfig};
use crate::internal::AsyncTryFrom;
use async_trait::async_trait;
use bytes::{Buf, BytesMut};
use graph_error::{GraphFailure, GraphResult};
use reqwest::Body;
use std::io::{BufReader, Read};

pub struct BodyRead {
    buf: String,
    blocking_body: Option<reqwest::blocking::Body>,
    async_body: Option<reqwest::Body>,
}

impl BodyRead {
    pub fn new(buf: String) -> BodyRead {
        BodyRead {
            buf,
            blocking_body: None,
            async_body: None,
        }
    }

    pub fn from_serialize<T: serde::Serialize>(body: &T) -> GraphResult<BodyRead> {
        let body = serde_json::to_string(body)?;
        Ok(BodyRead::new(body))
    }

    pub fn from_reader<T: Read>(mut reader: T) -> GraphResult<BodyRead> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        Ok(BodyRead::new(buf))
    }

    pub async fn from_async_read<T: tokio::io::AsyncReadExt + Unpin>(
        mut reader: T,
    ) -> GraphResult<BodyRead> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf).await?;
        Ok(BodyRead::new(buf))
    }
}

impl From<BodyRead> for reqwest::Body {
    fn from(upload: BodyRead) -> Self {
        if let Some(body) = upload.async_body {
            return body;
        }

        reqwest::Body::from(upload.buf)
    }
}

impl From<BodyRead> for reqwest::blocking::Body {
    fn from(upload: BodyRead) -> Self {
        if let Some(body) = upload.blocking_body {
            return body;
        }

        reqwest::blocking::Body::from(upload.buf)
    }
}

impl From<String> for BodyRead {
    fn from(value: String) -> Self {
        BodyRead::new(value)
    }
}

impl<R: Read> TryFrom<BufReader<R>> for BodyRead {
    type Error = GraphFailure;

    fn try_from(reader: BufReader<R>) -> Result<Self, Self::Error> {
        BodyRead::from_reader(reader)
    }
}

impl TryFrom<std::fs::File> for BodyRead {
    type Error = GraphFailure;

    fn try_from(value: std::fs::File) -> Result<Self, Self::Error> {
        BodyRead::from_reader(value)
    }
}

#[async_trait]
impl AsyncTryFrom<tokio::fs::File> for BodyRead {
    type Error = GraphFailure;

    async fn async_try_from(file: tokio::fs::File) -> Result<Self, Self::Error> {
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

impl From<reqwest::Body> for BodyRead {
    fn from(body: Body) -> Self {
        BodyRead {
            buf: Default::default(),
            blocking_body: None,
            async_body: Some(body),
        }
    }
}

impl From<reqwest::blocking::Body> for BodyRead {
    fn from(body: reqwest::blocking::Body) -> Self {
        BodyRead {
            buf: Default::default(),
            blocking_body: Some(body),
            async_body: None,
        }
    }
}

impl TryFrom<FileConfig> for BodyRead {
    type Error = GraphFailure;

    fn try_from(file_config: FileConfig) -> Result<Self, Self::Error> {
        BodyRead::try_from(&file_config)
    }
}

impl TryFrom<&FileConfig> for BodyRead {
    type Error = GraphFailure;

    fn try_from(file_config: &FileConfig) -> Result<Self, Self::Error> {
        let mut file = std::fs::File::open(file_config.path.as_path())?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(BodyRead::new(buf))
    }
}

impl BodyExt for BodyRead {
    fn into_body(self) -> GraphResult<BodyRead> {
        Ok(self)
    }
}
