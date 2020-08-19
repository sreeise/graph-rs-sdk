use crate::client::*;
use crate::http::{
    AsyncHttpClient, AsyncTryFrom, BlockingHttpClient, GraphResponse, IntoDeltaRequest,
    IntoReqAsync, IntoReqBlocking, RequestClient, UploadSessionClient,
};
use crate::types::delta::{Delta, NextLink};
use crate::types::{content::Content, delta::DeltaRequest};
use graph_error::{GraphFailure, GraphResult};
use reqwest::header::{HeaderValue, IntoHeaderName};
use std::marker::PhantomData;
use std::sync::mpsc::Receiver;

pub struct IntoResponse<'a, T, Client>
where
    Client: RequestClient,
{
    client: &'a Graph<Client>,
    ident: PhantomData<T>,
    error: Option<GraphFailure>,
}

pub type IntoResBlocking<'a, T> = IntoResponse<'a, T, BlockingHttpClient>;
pub type IntoResAsync<'a, T> = IntoResponse<'a, T, AsyncHttpClient>;

impl<'a, T, Client> IntoResponse<'a, T, Client>
where
    Client: RequestClient,
{
    pub fn new(client: &'a Graph<Client>) -> IntoResponse<'a, T, Client> {
        IntoResponse {
            client,
            ident: PhantomData,
            error: None,
        }
    }

    pub(crate) fn new_error(
        client: &'a Graph<Client>,
        error: GraphFailure,
    ) -> IntoResponse<'a, T, Client> {
        IntoResponse {
            client,
            ident: PhantomData,
            error: Some(error),
        }
    }

    pub fn query(&self, key: &str, value: &str) -> &Self {
        self.client.request().url_mut(|url| {
            url.append_query_pair(key, value);
        });
        self
    }

    pub fn select(&self, value: &[&str]) -> &Self {
        self.client.request().url_mut(|url| {
            url.select(value);
        });
        self
    }

    pub fn expand(&self, value: &[&str]) -> &Self {
        self.client.request().url_mut(|url| {
            url.expand(value);
        });
        self
    }

    pub fn filter(&self, value: &[&str]) -> &Self {
        self.client.request().url_mut(|url| {
            url.filter(value);
        });
        self
    }

    pub fn order_by(&self, value: &[&str]) -> &Self {
        self.client.request().url_mut(|url| {
            url.order_by(value);
        });
        self
    }

    pub fn search(&self, value: &str) -> &Self {
        self.client.request().url_mut(|url| {
            url.search(value);
        });
        self
    }

    pub fn format(&self, value: &str) -> &Self {
        self.client.request().url_mut(|url| {
            url.format(value);
        });
        self
    }

    pub fn skip(&self, value: &str) -> &Self {
        self.client.request().url_mut(|url| {
            url.skip(value);
        });
        self
    }

    pub fn top(&self, value: &str) -> &Self {
        self.client.request().url_mut(|url| {
            url.top(value);
        });
        self
    }

    pub fn header<H: IntoHeaderName>(&self, name: H, value: HeaderValue) -> &Self {
        self.client.request().header(name, value);
        self
    }
}

impl<'a, T> IntoResBlocking<'a, T> {
    pub fn json<U>(self) -> GraphResult<U>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.client.request().response()?;
        Ok(response.json()?)
    }
}

impl<'a, T> IntoResBlocking<'a, T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    pub fn build(self) -> IntoReqBlocking<T> {
        let builder = self.client.request().build();
        IntoReqBlocking::new(builder, None, self.error)
    }

    pub fn send(self) -> GraphResult<GraphResponse<T>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        self.client.request().execute()
    }
}

impl<'a> IntoResBlocking<'a, UploadSessionClient<BlockingHttpClient>> {
    pub fn build(self) -> IntoReqBlocking<UploadSessionClient<BlockingHttpClient>> {
        let (file, builder) = self.client.request().build_upload_session();
        IntoReqBlocking::new(builder, file, self.error)
    }

    pub fn send(self) -> GraphResult<UploadSessionClient<BlockingHttpClient>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        self.client.request().upload_session()
    }
}

impl<'a> IntoResBlocking<'a, GraphResponse<Content>> {
    pub fn build(self) -> IntoReqBlocking<GraphResponse<Content>> {
        let builder = self.client.request().build();
        IntoReqBlocking::new(builder, None, self.error)
    }

    pub fn send(self) -> GraphResult<GraphResponse<Content>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.client.request().response()?;
        Ok(std::convert::TryFrom::try_from(response)?)
    }
}

impl<'a, T: 'static + Send + NextLink + Clone> IntoResBlocking<'a, DeltaRequest<T>>
where
    for<'de> T: serde::Deserialize<'de>,
{
    pub fn build(self) -> IntoDeltaRequest<T, reqwest::blocking::RequestBuilder> {
        let client = self.client.request();
        let builder = client.build();
        let token = client.token();
        IntoDeltaRequest::<T, reqwest::blocking::RequestBuilder>::new(token, builder, self.error)
    }

    pub fn send(self) -> Receiver<Delta<T>> {
        let request = self.build();
        request.send()
    }
}

// Async Impl

impl<'a, T> IntoResAsync<'a, T> {
    pub async fn json<U>(self) -> GraphResult<U>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let request = self.client.request().build().await;
        let response = request.send().await?;
        response.json().await.map_err(GraphFailure::from)
    }
}

impl<'a, T> IntoResAsync<'a, T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    pub async fn build(self) -> IntoReqAsync<T> {
        let builder = self.client.request().build().await;
        IntoReqAsync::new(builder, None, self.error)
    }

    pub async fn send(self) -> GraphResult<GraphResponse<T>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.client.request().response().await?;
        AsyncTryFrom::<reqwest::Response>::try_from(response).await
    }
}

impl<'a> IntoResAsync<'a, GraphResponse<Content>> {
    pub async fn build(self) -> IntoReqAsync<GraphResponse<Content>> {
        let builder = self.client.request().build().await;
        IntoReqAsync::new(builder, None, self.error)
    }

    pub async fn send(self) -> GraphResult<GraphResponse<Content>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.client.request().response().await?;
        AsyncTryFrom::<reqwest::Response>::try_from(response).await
    }
}

impl<'a> IntoResAsync<'a, UploadSessionClient<AsyncHttpClient>> {
    pub async fn build(self) -> IntoReqAsync<UploadSessionClient<AsyncHttpClient>> {
        let (file, builder) = self.client.request().build_upload_session().await;
        IntoReqAsync::new(builder, file, self.error)
    }

    pub async fn send(self) -> GraphResult<UploadSessionClient<AsyncHttpClient>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        self.client.request().upload_session().await
    }
}

impl<'a, T: 'static + Send + NextLink + Clone> IntoResAsync<'a, DeltaRequest<T>>
where
    for<'de> T: serde::Deserialize<'de>,
{
    pub async fn build(self) -> IntoDeltaRequest<T, reqwest::RequestBuilder> {
        let client = self.client.request();
        let builder = client.build().await;
        let token = client.token();
        IntoDeltaRequest::<T, reqwest::RequestBuilder>::new(token, builder, self.error)
    }

    pub async fn send(self) -> tokio::sync::mpsc::Receiver<Delta<T>> {
        let request = self.build().await;
        request.send().await
    }
}
