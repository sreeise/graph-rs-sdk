use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use bytes::{BufMut, BytesMut};
use tokio::io::AsyncReadExt;
use async_trait::async_trait;
use crate::FileConfig;

pub trait UploadSession {
    fn upload_url(&self) -> String;
}

pub trait AsBytesMut<RHS = Self> {
    type Error: Error;

    fn as_bytes_mut(&mut self) -> Result<BytesMut, Self::Error>;
}

impl AsBytesMut for std::fs::File {
    type Error = std::io::Error;

    fn as_bytes_mut(&mut self) -> Result<BytesMut, Self::Error> {
        let mut buf: Vec<u8> = Vec::new();
        self.read_to_end(&mut buf)?;
        let mut bytes_mut = BytesMut::from_iter(buf.iter());
        Ok(bytes_mut)
    }
}

impl AsBytesMut for Vec<u8> {
    type Error = std::io::Error;

    fn as_bytes_mut(&mut self) -> Result<BytesMut, Self::Error> {
        Ok(BytesMut::from_iter(self.iter()))
    }
}

impl AsBytesMut for VecDeque<u8> {
    type Error = std::io::Error;

    fn as_bytes_mut(&mut self) -> Result<BytesMut, Self::Error> {
        Ok(BytesMut::from_iter(self.iter()))
    }
}

impl AsBytesMut for FileConfig {
    type Error = std::io::Error;

    fn as_bytes_mut(&mut self) -> Result<BytesMut, Self::Error> {
        let mut file = File::open(self.path.as_path())?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)?;
        let mut bytes_mut = BytesMut::from_iter(buf.iter());
        Ok(bytes_mut)
    }
}

#[async_trait]
pub trait AsyncAsBytesMut<RHS = Self> {
    type Error: Error;

    async fn async_as_bytes_mut(&mut self) -> Result<BytesMut, Self::Error>;
}

#[async_trait]
impl AsyncAsBytesMut for tokio::fs::File {
    type Error = tokio::io::Error;

    async fn async_as_bytes_mut(&mut self) -> Result<BytesMut, Self::Error> {
        let mut buf: Vec<u8> = Vec::new();
        self.read_to_end(&mut buf).await?;
        let mut bytes_mut = BytesMut::from_iter(buf.iter());
        Ok(bytes_mut)
    }
}

#[async_trait]
impl AsyncAsBytesMut for Vec<u8> {
    type Error = tokio::io::Error;

    async fn async_as_bytes_mut(&mut self) -> Result<BytesMut, Self::Error> {
        Ok(BytesMut::from_iter(self.iter()))
    }
}

#[async_trait]
impl AsyncAsBytesMut for VecDeque<u8> {
    type Error = tokio::io::Error;

    async fn async_as_bytes_mut(&mut self) -> Result<BytesMut, Self::Error> {
        Ok(BytesMut::from_iter(self.iter()))
    }
}

#[async_trait]
impl AsyncAsBytesMut for FileConfig {
    type Error = tokio::io::Error;

    async fn async_as_bytes_mut(&mut self) -> Result<BytesMut, Self::Error> {
        let mut file = tokio::fs::File::open(self.path.as_path()).await?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf).await?;
        let mut bytes_mut = BytesMut::from_iter(buf.iter());
        Ok(bytes_mut)
    }
}

