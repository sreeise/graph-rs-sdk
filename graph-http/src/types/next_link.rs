pub struct NextLink;

#[derive(Deserialize)]
pub struct NextLinkValues<V> {
    pub(crate) value: Vec<V>,
    #[serde(rename = "@odata.nextLink")]
    pub(crate) next_link: Option<String>,
}

use crate::traits::ODataNextLink;
use async_trait::async_trait;
use bytes::{BufMut, BytesMut};
use graph_error::{GraphFailure, GraphResult, WithGraphErrorAsync};
use reqwest::header::CONTENT_TYPE;
use reqwest::Response;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct NextLinkResponse {
    pub next_link: Option<String>,
    pub error: Option<GraphFailure>,
    pub response: reqwest::Response,
}

impl NextLinkResponse {
    async fn from_response<T: 'static + Send + ODataNextLink + Clone + Debug>(
        mut response: Response,
    ) -> NextLinkResponse
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let url = response.url().clone();
        let mut buf = BytesMut::new();

        while let Ok(bytes_option) = response.chunk().await {
            if let Some(bytes) = bytes_option {
                buf.put(bytes);
            }
        }

        match serde_json::from_slice::<T>(buf.as_mut()).map_err(GraphFailure::from) {
            Ok(body) => NextLinkResponse {
                next_link: body.odata_next_link(),
                error: None,
                response,
            },
            Err(error) => NextLinkResponse {
                next_link: None,
                error: Some(error),
                response,
            },
        }
    }
}

#[async_trait]
pub trait NextLinkTask {
    async fn next_link_channel<
        T: 'static + Send + ODataNextLink + Clone + Debug + DeserializeOwned,
    >(
        self,
        access_token: String,
    ) -> tokio::sync::mpsc::Receiver<Option<NextLinkResponse>>;
}

#[async_trait]
impl NextLinkTask for reqwest::Response {
    async fn next_link_channel<
        T: 'static + Send + ODataNextLink + Clone + Debug + DeserializeOwned,
    >(
        self,
        access_token: String,
    ) -> Receiver<Option<NextLinkResponse>> {
        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        let body_result: GraphResult<T> = self.json().await.map_err(GraphFailure::from);

        if let Ok(body) = body_result {
            let mut next_link = body.odata_next_link().to_owned();

            tokio::spawn(async move {
                let client = reqwest::Client::new();
                while let Some(next) = next_link.as_ref() {
                    let mut res = client
                        .get(next.as_str())
                        .header(CONTENT_TYPE, "application/json")
                        .bearer_auth(access_token.as_str())
                        .send()
                        .await
                        .map_err(GraphFailure::from);

                    if res.is_ok() {
                        if let Ok(mut response) = res.unwrap().with_graph_error().await {
                            let next_response =
                                NextLinkResponse::from_response::<T>(response).await;
                            next_link = next_response.next_link.clone();
                            sender.send(Some(next_response));
                        }
                    }
                }
            })
            .await.unwrap();
        }

        receiver
    }
}
