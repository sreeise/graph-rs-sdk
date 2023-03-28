use crate::traits::AsyncIterator;
use crate::RangeIter;
use async_trait::async_trait;
use graph_error::{GraphFailure, GraphResult, WithGraphErrorAsync};
use reqwest::header::HeaderMap;

pub struct UploadSession {
    url: reqwest::Url,
    range_iter: RangeIter,
    client: reqwest::Client,
}

impl UploadSession {
    pub fn new(url: reqwest::Url, range_iter: RangeIter) -> UploadSession {
        UploadSession {
            url,
            range_iter,
            client: Default::default(),
        }
    }

    pub fn url(&self) -> &reqwest::Url {
        &self.url
    }

    async fn send(
        &mut self,
        header_map: HeaderMap,
        body: reqwest::Body,
    ) -> GraphResult<reqwest::Response> {
        self.client
            .put(self.url.clone())
            .headers(header_map)
            .body(body)
            .send()
            .await?
            .with_graph_error()
            .await
            .map_err(GraphFailure::from)
    }

    pub async fn cancel(&mut self) -> GraphResult<reqwest::Response> {
        self.client
            .delete(self.url.clone())
            .send()
            .await?
            .with_graph_error()
            .await
            .map_err(GraphFailure::from)
    }
}

#[async_trait]
impl AsyncIterator for UploadSession {
    type Item = GraphResult<reqwest::Response>;

    async fn next(&mut self) -> Option<Self::Item> {
        let (header_map, body) = self.range_iter.pop_front()?;
        Some(self.send(header_map, body).await)
    }
}
