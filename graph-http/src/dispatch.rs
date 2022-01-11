use crate::async_client::AsyncHttpClient;
use crate::blocking_client::BlockingHttpClient;
use crate::traits::*;
use crate::types::*;
use crate::uploadsession::UploadSessionClient;
use crate::url::GraphUrl;
use crate::GraphResponse;
use bytes::Bytes;
use graph_error::{GraphFailure, GraphResult, WithGraphError, WithGraphErrorAsync};
use reqwest::header::CONTENT_TYPE;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

pub struct DispatchRequest<T, Builder> {
    request: Builder,
    ident: PhantomData<T>,
    file: Option<PathBuf>,
    error: Option<GraphFailure>,
}

pub type DispatchBlocking<T> = DispatchRequest<T, reqwest::blocking::RequestBuilder>;
pub type DispatchAsync<T> = DispatchRequest<T, reqwest::RequestBuilder>;

impl<T> DispatchBlocking<T> {
    pub fn new(
        request: reqwest::blocking::RequestBuilder,
        file: Option<PathBuf>,
        error: Option<GraphFailure>,
    ) -> DispatchBlocking<T> {
        DispatchBlocking {
            request,
            ident: Default::default(),
            file,
            error,
        }
    }
}

impl<T> DispatchBlocking<T> {
    pub fn json<U>(self) -> GraphResult<GraphResponse<U>>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }

        let response = self.request.send()?;
        let headers = response.headers().clone();
        let status = response.status();
        let url = GraphUrl::from(response.url());
        let json = response.json().map_err(GraphFailure::from)?;
        Ok(GraphResponse::new(url, json, status, headers))
    }

    pub fn text(self) -> GraphResult<GraphResponse<String>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }

        let response = self.request.send()?;
        let headers = response.headers().clone();
        let status = response.status();
        let url = GraphUrl::from(response.url());
        let text = response.text().map_err(GraphFailure::from)?;
        Ok(GraphResponse::new(url, text, status, headers))
    }

    pub fn bytes(self) -> GraphResult<GraphResponse<Bytes>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }

        let response = self.request.send()?;
        let headers = response.headers().clone();
        let status = response.status();
        let url = GraphUrl::from(response.url());
        let bytes = response.bytes().map_err(GraphFailure::from)?;
        Ok(GraphResponse::new(url, bytes, status, headers))
    }
}

impl<T> DispatchBlocking<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    pub fn send(self) -> GraphResult<GraphResponse<T>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.request.send()?;
        Ok(std::convert::TryFrom::try_from(response)?)
    }
}

impl DispatchBlocking<GraphResponse<NoContent>> {
    pub fn send(self) -> GraphResult<GraphResponse<serde_json::Value>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.request.send()?;
        GraphResponse::<serde_json::Value>::from_no_content(response)
    }
}

impl DispatchBlocking<UploadSessionClient<BlockingHttpClient>> {
    pub fn send(self) -> GraphResult<UploadSessionClient<BlockingHttpClient>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }

        let file = self
            .file
            .ok_or_else(|| GraphFailure::invalid("file for upload session"))?;

        let response = self.request.send()?.with_graph_error()?;
        let upload_session: serde_json::Value = response.json()?;
        let mut session = UploadSessionClient::new(upload_session)?;
        session.set_file(file)?;
        Ok(session)
    }
}

// Async Impl

impl<T> DispatchAsync<T> {
    pub fn new(
        request: reqwest::RequestBuilder,
        file: Option<PathBuf>,
        error: Option<GraphFailure>,
    ) -> DispatchAsync<T> {
        DispatchAsync {
            request,
            ident: Default::default(),
            file,
            error,
        }
    }
}

impl<T> DispatchAsync<T> {
    pub async fn json<U>(self) -> GraphResult<GraphResponse<U>>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }

        let response = self.request.send().await?;
        let headers = response.headers().clone();
        let status = response.status();
        let url = GraphUrl::from(response.url());
        let json = response.json().await.map_err(GraphFailure::from)?;
        Ok(GraphResponse::new(url, json, status, headers))
    }

    pub async fn text(self) -> GraphResult<GraphResponse<String>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }

        let response = self.request.send().await?;
        let headers = response.headers().clone();
        let status = response.status();
        let url = GraphUrl::from(response.url());
        let text = response.text().await.map_err(GraphFailure::from)?;
        Ok(GraphResponse::new(url, text, status, headers))
    }

    pub async fn bytes(self) -> GraphResult<GraphResponse<Bytes>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }

        let response = self.request.send().await?;
        let headers = response.headers().clone();
        let status = response.status();
        let url = GraphUrl::from(response.url());
        let bytes = response.bytes().await.map_err(GraphFailure::from)?;
        Ok(GraphResponse::new(url, bytes, status, headers))
    }
}

impl<T> DispatchAsync<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    pub async fn send(self) -> GraphResult<GraphResponse<T>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.request.send().await?;
        AsyncTryFrom::<reqwest::Response>::async_try_from(response).await
    }
}

impl DispatchAsync<GraphResponse<NoContent>> {
    pub async fn send(self) -> GraphResult<GraphResponse<serde_json::Value>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.request.send().await?;
        GraphResponse::<serde_json::Value>::async_from_no_content(response).await
    }
}

impl DispatchAsync<UploadSessionClient<AsyncHttpClient>> {
    pub async fn send(self) -> GraphResult<UploadSessionClient<AsyncHttpClient>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }

        let file = self
            .file
            .ok_or_else(|| GraphFailure::invalid("file for upload session"))?;

        let response = self.request.send().await?.with_graph_error().await?;
        let upload_session: serde_json::Value = response.json().await?;
        let mut session = UploadSessionClient::new_async(upload_session)?;
        session.set_file(file).await?;
        Ok(session)
    }
}

pub struct DispatchDelta<T, Builder> {
    token: String,
    request: Builder,
    error: Option<GraphFailure>,
    phantom: PhantomData<T>,
}

impl<T, Builder> DispatchDelta<T, Builder> {
    pub fn new(
        token: String,
        client: Builder,
        error: Option<GraphFailure>,
    ) -> DispatchDelta<T, Builder> {
        DispatchDelta {
            token,
            request: client,
            error,
            phantom: Default::default(),
        }
    }
}

impl<T: 'static + Send + ODataLink + Clone> DispatchDelta<T, reqwest::blocking::RequestBuilder>
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
            self.request.send().map_err(GraphFailure::from);
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
                    match res.unwrap().with_graph_error() {
                        Ok(response) => {
                            let url = GraphUrl::from(response.url());
                            let headers = response.headers().clone();
                            let status = response.status();
                            match response.json::<T>() {
                                Ok(value) => {
                                    next_link = value.next_link();
                                    sender
                                        .send(Delta::Next(GraphResponse::new(
                                            url, value, status, headers,
                                        )))
                                        .unwrap();
                                }
                                Err(err) => {
                                    next_link = None;
                                    sender.send(Delta::Done(Some(err.into()))).unwrap();
                                    is_done = true;
                                }
                            }
                        }
                        Err(err) => {
                            next_link = None;
                            sender.send(Delta::Done(Some(err.into()))).unwrap();
                            is_done = true;
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

impl<T: 'static + Send + ODataLink + Clone> DispatchDelta<T, reqwest::RequestBuilder>
where
    for<'de> T: serde::Deserialize<'de>,
{
    pub async fn send(self) -> tokio::sync::mpsc::Receiver<Delta<T>> {
        let (sender, receiver) = tokio::sync::mpsc::channel(100);

        if self.error.is_some() {
            sender
                .send(Delta::Done(Some(self.error.unwrap_or_default())))
                .await
                .unwrap();
            return receiver;
        }

        let initial_res: GraphResult<reqwest::Response> =
            self.request.send().await.map_err(GraphFailure::from);
        let response: GraphResult<GraphResponse<T>> =
            AsyncTryFrom::<GraphResult<reqwest::Response>>::async_try_from(initial_res).await;
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
                    match res.unwrap().with_graph_error().await {
                        Ok(response) => {
                            let url = GraphUrl::from(response.url());
                            let headers = response.headers().clone();
                            let status = response.status();
                            match response.json::<T>().await {
                                Ok(value) => {
                                    next_link = value.next_link();
                                    sender
                                        .send(Delta::Next(GraphResponse::new(
                                            url, value, status, headers,
                                        )))
                                        .await
                                        .unwrap();
                                }
                                Err(err) => {
                                    next_link = None;
                                    sender.send(Delta::Done(Some(err.into()))).await.unwrap();
                                    is_done = true;
                                }
                            }
                        }

                        Err(err) => {
                            next_link = None;
                            sender.send(Delta::Done(Some(err.into()))).await.unwrap();
                            is_done = true;
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
