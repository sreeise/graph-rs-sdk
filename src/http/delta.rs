use crate::http::{AsyncTryFrom, GraphResponse};
use crate::types::delta::{Delta, NextLink};
use graph_error::{GraphFailure, GraphResult};
use reqwest::header::CONTENT_TYPE;
use std::marker::PhantomData;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

pub struct IntoDeltaRequest<T, Builder> {
    token: String,
    client: Builder,
    error: Option<GraphFailure>,
    phantom: PhantomData<T>,
}

impl<T, Builder> IntoDeltaRequest<T, Builder> {
    pub fn new(
        token: String,
        client: Builder,
        error: Option<GraphFailure>,
    ) -> IntoDeltaRequest<T, Builder> {
        IntoDeltaRequest {
            token,
            client,
            error,
            phantom: Default::default(),
        }
    }
}

impl<T: 'static + Send + NextLink + Clone> IntoDeltaRequest<T, reqwest::blocking::RequestBuilder>
where
    for<'de> T: serde::Deserialize<'de>,
{
    pub fn send(self) -> Receiver<Delta<T>> {
        let (sender, receiver) = channel();
        if self.error.is_some() {
            sender
                .send(Delta::Done(Some(self.error.unwrap_or_default())))
                .unwrap();
            return receiver;
        }

        let initial_res: GraphResult<reqwest::blocking::Response> =
            self.client.send().map_err(GraphFailure::from);
        let response: GraphResult<GraphResponse<T>> = std::convert::TryFrom::try_from(initial_res);
        if let Err(err) = response {
            sender.send(Delta::Done(Some(err))).unwrap();
            return receiver;
        }

        let token = self.token;
        let response = response.unwrap();
        let mut next_link = response.body().next_link();
        sender.send(Delta::Next(response)).unwrap();

        thread::spawn(move || {
            let mut is_done = false;
            let client = reqwest::blocking::Client::new();
            while let Some(next) = next_link {
                let res = client
                    .get(next.as_str())
                    .header(CONTENT_TYPE, "application/json")
                    .bearer_auth(token.as_str())
                    .send()
                    .map_err(GraphFailure::from);

                if let Err(err) = res {
                    next_link = None;
                    sender.send(Delta::Done(Some(err))).unwrap();
                    is_done = true;
                } else {
                    let response = res.unwrap();
                    if let Some(err) = GraphFailure::from_response(&response) {
                        next_link = None;
                        sender.send(Delta::Done(Some(err))).unwrap();
                        is_done = true;
                    } else {
                        let headers = response.headers().clone();
                        let status = response.status().as_u16();
                        let value_res: GraphResult<T> = response.json().map_err(GraphFailure::from);
                        match value_res {
                            Ok(value) => {
                                next_link = value.next_link();
                                sender
                                    .send(Delta::Next(GraphResponse::new(value, status, headers)))
                                    .unwrap();
                            },
                            Err(err) => {
                                next_link = None;
                                sender.send(Delta::Done(Some(err))).unwrap();
                                is_done = true;
                            },
                        }
                    }
                }
            }
            if !is_done {
                sender.send(Delta::Done(None)).unwrap();
            }
        });

        receiver
    }
}

impl<T: 'static + Send + NextLink + Clone> IntoDeltaRequest<T, reqwest::RequestBuilder>
where
    for<'de> T: serde::Deserialize<'de>,
{
    pub async fn send(self) -> tokio::sync::mpsc::Receiver<Delta<T>> {
        let (mut sender, receiver) = tokio::sync::mpsc::channel(100);

        if self.error.is_some() {
            sender
                .send(Delta::Done(Some(self.error.unwrap_or_default())))
                .await
                .unwrap();
            return receiver;
        }

        let initial_res: GraphResult<reqwest::Response> =
            self.client.send().await.map_err(GraphFailure::from);
        let response: GraphResult<GraphResponse<T>> =
            AsyncTryFrom::<GraphResult<reqwest::Response>>::try_from(initial_res).await;
        if let Err(err) = response {
            sender.send(Delta::Done(Some(err))).await.unwrap();
            return receiver;
        }

        let token = self.token;
        let response = response.unwrap();
        let mut next_link = response.body().next_link();
        sender.send(Delta::Next(response)).await.unwrap();

        tokio::spawn(async move {
            let mut is_done = false;
            let client = reqwest::Client::new();
            while let Some(next) = next_link {
                let res = client
                    .get(next.as_str())
                    .header(CONTENT_TYPE, "application/json")
                    .bearer_auth(token.as_str())
                    .send()
                    .await
                    .map_err(GraphFailure::from);

                if let Err(err) = res {
                    next_link = None;
                    sender.send(Delta::Done(Some(err))).await.unwrap();
                    is_done = true;
                } else {
                    let response = res.unwrap();
                    if let Some(err) = GraphFailure::from_async_response(&response) {
                        next_link = None;
                        sender.send(Delta::Done(Some(err))).await.unwrap();
                        is_done = true;
                    } else {
                        let headers = response.headers().clone();
                        let status = response.status().as_u16();
                        let value_res: GraphResult<T> =
                            response.json().await.map_err(GraphFailure::from);
                        match value_res {
                            Ok(value) => {
                                next_link = value.next_link();
                                sender
                                    .send(Delta::Next(GraphResponse::new(value, status, headers)))
                                    .await
                                    .unwrap();
                            },
                            Err(err) => {
                                next_link = None;
                                sender.send(Delta::Done(Some(err))).await.unwrap();
                                is_done = true;
                            },
                        }
                    }
                }
            }
            if !is_done {
                sender.send(Delta::Done(None)).await.unwrap();
            }
        });

        receiver
    }
}
