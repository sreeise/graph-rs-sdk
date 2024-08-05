use crate::api_impl::FileConfig;
use crate::traits::BodyExt;
use bytes::{Buf, BytesMut};
use graph_error::{GraphFailure, GraphResult};
use reqwest::Body;
use std::io::{BufReader, Read};

pub struct BodyRead {
    buf: String,
    bytes_buf: Option<Vec<u8>>,
    blocking_body: Option<reqwest::blocking::Body>,
    async_body: Option<Body>,
}

impl BodyRead {
    pub fn new(buf: String) -> BodyRead {
        BodyRead {
            buf,
            bytes_buf: None,
            blocking_body: None,
            async_body: None,
        }
    }

    pub fn has_string_buf(&self) -> bool {
        self.bytes_buf.is_none() && self.blocking_body.is_none() && self.async_body.is_none()
    }

    pub fn has_byte_buf(&self) -> bool {
        self.bytes_buf.is_some()
    }

    pub fn from_serialize<T: serde::Serialize>(body: &T) -> GraphResult<BodyRead> {
        let body = serde_json::to_string(body)?;
        Ok(BodyRead::new(body))
    }

    pub fn from_read<T: Read>(mut reader: T) -> GraphResult<BodyRead> {
        let mut byte_buf = Vec::new();
        reader.read_to_end(&mut byte_buf)?;
        Ok(BodyRead::from(byte_buf))
    }

    pub async fn from_async_read<T: tokio::io::AsyncReadExt + Unpin>(
        mut reader: T,
    ) -> GraphResult<BodyRead> {
        let mut byte_buf = Vec::new();
        reader.read_to_end(&mut byte_buf).await?;
        Ok(BodyRead::from(reqwest::Body::from(byte_buf)))
    }
}

impl From<BodyRead> for Body {
    fn from(body_read: BodyRead) -> Self {
        if let Some(body) = body_read.async_body {
            return body;
        }

        if let Some(buf) = body_read.bytes_buf {
            return reqwest::Body::from(buf);
        }

        reqwest::Body::from(body_read.buf)
    }
}

impl From<BodyRead> for reqwest::blocking::Body {
    fn from(body_read: BodyRead) -> Self {
        if let Some(body) = body_read.blocking_body {
            return body;
        }

        if let Some(buf) = body_read.bytes_buf {
            return reqwest::blocking::Body::from(buf);
        }

        reqwest::blocking::Body::from(body_read.buf)
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
        BodyRead::from_read(reader)
    }
}

impl From<tokio::fs::File> for BodyRead {
    fn from(file: tokio::fs::File) -> Self {
        BodyRead {
            buf: Default::default(),
            bytes_buf: None,
            blocking_body: None,
            async_body: Some(reqwest::Body::from(file)),
        }
    }
}

impl TryFrom<BytesMut> for BodyRead {
    type Error = GraphFailure;

    fn try_from(bytes_mut: BytesMut) -> Result<Self, Self::Error> {
        BodyRead::from_read(bytes_mut.reader())
    }
}

impl TryFrom<bytes::Bytes> for BodyRead {
    type Error = GraphFailure;

    fn try_from(bytes: bytes::Bytes) -> Result<Self, Self::Error> {
        let mut byte_buf = Vec::new();
        bytes.reader().read_to_end(&mut byte_buf)?;
        Ok(BodyRead::from(byte_buf))
    }
}

impl From<Vec<u8>> for BodyRead {
    fn from(value: Vec<u8>) -> Self {
        BodyRead {
            buf: Default::default(),
            bytes_buf: Some(value),
            blocking_body: None,
            async_body: None,
        }
    }
}

impl From<Body> for BodyRead {
    fn from(body: Body) -> Self {
        BodyRead {
            buf: Default::default(),
            bytes_buf: None,
            blocking_body: None,
            async_body: Some(body),
        }
    }
}

impl From<reqwest::blocking::Body> for BodyRead {
    fn from(body: reqwest::blocking::Body) -> Self {
        BodyRead {
            buf: Default::default(),
            bytes_buf: None,
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
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(BodyRead::from(buf))
    }
}

impl TryFrom<&mut std::fs::File> for BodyRead {
    type Error = GraphFailure;

    fn try_from(file: &mut std::fs::File) -> Result<Self, Self::Error> {
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(BodyRead::from(buf))
    }
}

impl BodyExt for BodyRead {
    fn into_body(self) -> GraphResult<BodyRead> {
        Ok(self)
    }
}
