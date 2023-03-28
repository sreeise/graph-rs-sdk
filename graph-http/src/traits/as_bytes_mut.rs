use crate::FileConfig;
use async_trait::async_trait;
use bytes::{BufMut, BytesMut};
use graph_error::{GraphFailure, GraphResult};
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use tokio::io::AsyncReadExt;

pub struct AsBytesMutWrapped<T: AsBytesMut>(T);

impl<T: AsBytesMut> AsBytesMut for AsBytesMutWrapped<T> {
    fn as_bytes_mut(&self) -> GraphResult<BytesMut> {
        self.0.as_bytes_mut()
    }
}

pub trait AsBytesMut<RHS = Self> {
    fn as_bytes_mut(&self) -> GraphResult<BytesMut>;
}

impl AsBytesMut for Vec<u8> {
    fn as_bytes_mut(&self) -> GraphResult<BytesMut> {
        Ok(BytesMut::from_iter(self.iter()))
    }
}

impl AsBytesMut for VecDeque<u8> {
    fn as_bytes_mut(&self) -> GraphResult<BytesMut> {
        Ok(BytesMut::from_iter(self.iter()))
    }
}

impl AsBytesMut for &Path {
    fn as_bytes_mut(&self) -> GraphResult<BytesMut> {
        let mut file = std::fs::File::open(self)?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(BytesMut::from_iter(buf))
    }
}

impl AsBytesMut for FileConfig {
    fn as_bytes_mut(&self) -> GraphResult<BytesMut> {
        self.path.as_path().as_bytes_mut()
    }
}

impl AsBytesMut for PathBuf {
    fn as_bytes_mut(&self) -> GraphResult<BytesMut> {
        self.as_path().as_bytes_mut()
    }
}
