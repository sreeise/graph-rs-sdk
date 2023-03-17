use async_trait::async_trait;
use bytes::{BufMut, BytesMut};
use graph_error::{GraphFailure, GraphResult};
use serde::de::DeserializeOwned;

#[async_trait]
pub trait BodyFromBytes {
    async fn body_from_bytes<T: DeserializeOwned>(&mut self) -> GraphResult<T>;
}

#[async_trait]
impl BodyFromBytes for reqwest::Response {
    async fn body_from_bytes<T: DeserializeOwned>(&mut self) -> GraphResult<T> {
        let mut buf = BytesMut::new();
        while let Ok(bytes_option) = self.chunk().await {
            if let Some(bytes) = bytes_option {
                buf.put(bytes);
            }
        }
        serde_json::from_slice::<T>(buf.as_mut()).map_err(GraphFailure::from)
    }
}
