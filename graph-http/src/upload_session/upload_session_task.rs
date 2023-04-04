use crate::internal::ReqwestResult;
use crate::traits::AsyncIterator;
use crate::upload_session::RangeIter;
use async_stream::try_stream;
use async_trait::async_trait;
use futures::Stream;
use graph_error::{GraphFailure, GraphResult};
use reqwest::header::HeaderMap;
use reqwest::RequestBuilder;
use std::io::Read;
use std::time::Duration;

pub struct UploadSession {
    url: reqwest::Url,
    range_iter: RangeIter,
    client: reqwest::Client,
}

impl UploadSession {
    pub fn empty(url: reqwest::Url) -> UploadSession {
        UploadSession {
            url,
            range_iter: Default::default(),
            client: Default::default(),
        }
    }

    pub(crate) fn new(url: reqwest::Url, range_iter: RangeIter) -> UploadSession {
        UploadSession {
            url,
            range_iter,
            client: Default::default(),
        }
    }

    pub fn url(&self) -> &reqwest::Url {
        &self.url
    }

    fn map_request_builder(
        &self,
        components: Vec<(HeaderMap, reqwest::Body)>,
    ) -> Vec<RequestBuilder> {
        components
            .into_iter()
            .map(|(header_map, body)| {
                self.client
                    .put(self.url.clone())
                    .headers(header_map)
                    .body(body)
            })
            .collect()
    }

    async fn send(
        &self,
        header_map: HeaderMap,
        body: reqwest::Body,
    ) -> GraphResult<reqwest::Response> {
        self.client
            .put(self.url.clone())
            .headers(header_map)
            .body(body)
            .send()
            .await
            .map_err(GraphFailure::from)
    }

    pub fn status(&self) -> RequestBuilder {
        self.client.get(self.url.clone())
    }

    pub fn cancel(&self) -> RequestBuilder {
        self.client.delete(self.url.clone())
    }

    pub fn from_reader<U: AsRef<str>, R: Read>(
        upload_url: U,
        reader: R,
    ) -> GraphResult<UploadSession> {
        Ok(UploadSession {
            url: reqwest::Url::parse(upload_url.as_ref())?,
            range_iter: RangeIter::from_reader(reader)?,
            client: Default::default(),
        })
    }

    fn try_stream(&mut self) -> impl Stream<Item = GraphResult<reqwest::Response>> + '_ {
        try_stream! {
            let components = self.range_iter.map_all().ok_or(GraphFailure::invalid(
                "Invalid Headers (internal error, please report)",
            ))?;
            let request_builders = self.map_request_builder(components);

            for request_builder in request_builders {
                yield request_builder.send()
                    .await
                    .map_err(GraphFailure::from)?
            }
        }
    }

    /// Stream upload session responses.
    /// Each stream.next().await returns a [`Option<GraphResult<reqwest::Response>>`].
    ///
    /// No pinning is required. The stream is pinned before being returned to the caller.
    ///
    /// # Example
    /// ```rust
    /// use graph_rs_sdk::*;
    /// use futures::stream::StreamExt;
    /// use std::fs::OpenOptions;
    ///
    /// static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
    ///
    /// // Path to upload file to in OneDrive.
    /// static ONEDRIVE_PATH: &str = ":/file.txt:";
    ///
    /// static LOCAL_FILE_PATH: &str = "./file.txt";
    ///
    /// let client = Graph::new("ACCESS_TOKEN");
    ///
    ///  let response = client
    ///     .me()
    ///     .drive()
    ///     .item_by_path(ONEDRIVE_PATH)
    ///     .create_upload_session(&serde_json::json!({
    ///         "@microsoft.graph.conflictBehavior": Some("fail".to_string())
    ///     }))
    ///     .send()
    ///     .await?;
    ///
    ///     assert!(response.status().is_success());
    ///
    ///  let mut file = OpenOptions::new()
    ///     .read(true)
    ///     .open(LOCAL_FILE_PATH)?;
    ///
    ///  let mut upload_session = response.into_upload_session(file).await?;
    ///  let mut stream = upload_session.stream()?;
    ///
    ///  while let Some(result) = stream.next().await {
    ///     match result {
    ///         Ok(response) => {
    ///             println!("{response:#?}");
    ///
    ///             let body: serde_json::Value = response.json().await?;
    ///             println!("{body:#?}");
    ///         }
    ///         Err(err) => panic!("Error on upload session {err:#?}")
    ///     }
    ///  }
    /// ```
    pub fn stream(
        &mut self,
    ) -> GraphResult<impl Stream<Item = GraphResult<reqwest::Response>> + '_> {
        Ok(Box::pin(self.try_stream()))
    }

    pub fn channel(&mut self) -> GraphResult<tokio::sync::mpsc::Receiver<ReqwestResult>> {
        self.channel_buffer_timeout(self.range_iter.len() + 1, Duration::from_secs(30))
    }

    pub fn channel_timeout(
        &mut self,
        timeout: Duration,
    ) -> GraphResult<tokio::sync::mpsc::Receiver<ReqwestResult>> {
        self.channel_buffer_timeout(self.range_iter.len() + 1, timeout)
    }

    pub fn channel_buffer_timeout(
        &mut self,
        buffer: usize,
        timeout: Duration,
    ) -> GraphResult<tokio::sync::mpsc::Receiver<ReqwestResult>> {
        let (sender, receiver) = tokio::sync::mpsc::channel(buffer);

        let components = self.range_iter.map_all().ok_or(GraphFailure::invalid(
            "Invalid Headers (internal error, please report)",
        ))?;
        let request_builders = self.map_request_builder(components);

        tokio::spawn(async move {
            for request_builder in request_builders {
                let result = request_builder.send().await;
                sender.send_timeout(result, timeout).await.unwrap();
            }
        });

        Ok(receiver)
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
