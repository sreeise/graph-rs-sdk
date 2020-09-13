use crate::uploadsession::UploadSessionClient;
use crate::async_client::AsyncHttpClient;
use crate::traits::AsyncTryFrom;
use crate::{DispatchAsync, GraphResponse, DispatchDelta, DispatchBlocking, RequestClient};
use crate::types::{Content, DeltaPhantom, Delta};
use graph_error::{GraphResult, GraphFailure};
use crate::traits::NextLink;
use crate::blocking_client::BlockingHttpClient;
use reqwest::header::{IntoHeaderName, HeaderValue};
use std::marker::PhantomData;
use std::sync::mpsc::Receiver;
use std::convert::TryFrom;

pub struct IntoResponse<'a, T, Client>
    where
        Client: RequestClient,
{
    client: &'a Client,
    ident: PhantomData<T>,
    error: Option<GraphFailure>,
}

pub type IntoResBlocking<'a, T> = IntoResponse<'a, T, BlockingHttpClient>;
pub type IntoResAsync<'a, T> = IntoResponse<'a, T, AsyncHttpClient>;

impl<'a, T, Client> IntoResponse<'a, T, Client>
    where
        Client: RequestClient,
{
    pub fn new(client: &'a Client) -> IntoResponse<'a, T, Client> {
        IntoResponse {
            client,
            ident: PhantomData,
            error: None,
        }
    }

    pub(crate) fn new_error(
        client: &'a Client,
        error: GraphFailure,
    ) -> IntoResponse<'a, T, Client> {
        IntoResponse {
            client,
            ident: PhantomData,
            error: Some(error),
        }
    }

    pub fn query(&self, key: &str, value: &str) -> &Self {
        self.client.url_mut(|url| {
            url.append_query_pair(key, value);
        });
        self
    }

    pub fn select(&self, value: &[&str]) -> &Self {
        self.client.url_mut(|url| {
            url.select(value);
        });
        self
    }

    pub fn expand(&self, value: &[&str]) -> &Self {
        self.client.url_mut(|url| {
            url.expand(value);
        });
        self
    }

    pub fn filter(&self, value: &[&str]) -> &Self {
        self.client.url_mut(|url| {
            url.filter(value);
        });
        self
    }

    pub fn order_by(&self, value: &[&str]) -> &Self {
        self.client.url_mut(|url| {
            url.order_by(value);
        });
        self
    }

    pub fn search(&self, value: &str) -> &Self {
        self.client.url_mut(|url| {
            url.search(value);
        });
        self
    }

    pub fn format(&self, value: &str) -> &Self {
        self.client.url_mut(|url| {
            url.format(value);
        });
        self
    }

    pub fn skip(&self, value: &str) -> &Self {
        self.client.url_mut(|url| {
            url.skip(value);
        });
        self
    }

    pub fn top(&self, value: &str) -> &Self {
        self.client.url_mut(|url| {
            url.top(value);
        });
        self
    }

    pub fn header<H: IntoHeaderName>(&self, name: H, value: HeaderValue) -> &Self {
        self.client.header(name, value);
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
        let response = self.client.response()?;
        Ok(response.json()?)
    }
}

impl<'a, T> IntoResBlocking<'a, T>
    where
            for<'de> T: serde::Deserialize<'de>,
{
    pub fn build(self) -> DispatchBlocking<T> {
        let builder = self.client.build();
        DispatchBlocking::new(builder, None, self.error)
    }

    pub fn send(self) -> GraphResult<GraphResponse<T>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        self.client.execute()
    }
}

impl<'a> IntoResBlocking<'a, UploadSessionClient<BlockingHttpClient>> {
    pub fn build(self) -> DispatchBlocking<UploadSessionClient<BlockingHttpClient>> {
        let (file, builder) = self.client.build_upload_session();
        DispatchBlocking::new(builder, file, self.error)
    }

    pub fn send(self) -> GraphResult<UploadSessionClient<BlockingHttpClient>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        self.client.upload_session()
    }
}

impl<'a> IntoResBlocking<'a, GraphResponse<Content>> {
    pub fn build(self) -> DispatchBlocking<GraphResponse<Content>> {
        let builder = self.client.build();
        DispatchBlocking::new(builder, None, self.error)
    }

    pub fn send(self) -> GraphResult<GraphResponse<Content>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.client.response()?;
        Ok(std::convert::TryFrom::try_from(response)?)
    }
}

impl<'a, T: 'static + Send + NextLink + Clone> IntoResBlocking<'a, DeltaPhantom<T>>
    where
            for<'de> T: serde::Deserialize<'de>,
{
    pub fn build(self) -> DispatchDelta<T, reqwest::blocking::RequestBuilder> {
        let client = self.client;
        let builder = client.build();
        let token = client.token();
        DispatchDelta::<T, reqwest::blocking::RequestBuilder>::new(token, builder, self.error)
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
        let request = self.client.build().await;
        let response = request.send().await?;
        response.json().await.map_err(GraphFailure::from)
    }
}

impl<'a, T> IntoResAsync<'a, T>
    where
            for<'de> T: serde::Deserialize<'de>,
{
    pub async fn build(self) -> DispatchAsync<T> {
        let builder = self.client.build().await;
        DispatchAsync::new(builder, None, self.error)
    }

    pub async fn send(self) -> GraphResult<GraphResponse<T>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.client.response().await?;
        AsyncTryFrom::<reqwest::Response>::async_try_from(response).await
    }
}

impl<'a> IntoResAsync<'a, GraphResponse<Content>> {
    pub async fn build(self) -> DispatchAsync<GraphResponse<Content>> {
        let builder = self.client.build().await;
        DispatchAsync::new(builder, None, self.error)
    }

    pub async fn send(self) -> GraphResult<GraphResponse<Content>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.client.response().await?;
        AsyncTryFrom::<reqwest::Response>::async_try_from(response).await
    }
}

impl<'a> IntoResAsync<'a, UploadSessionClient<AsyncHttpClient>> {
    pub async fn build(self) -> DispatchAsync<UploadSessionClient<AsyncHttpClient>> {
        let (file, builder) = self.client.build_upload_session().await;
        DispatchAsync::new(builder, file, self.error)
    }

    pub async fn send(self) -> GraphResult<UploadSessionClient<AsyncHttpClient>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        self.client.upload_session().await
    }
}

impl<'a, T: 'static + Send + NextLink + Clone> IntoResAsync<'a, DeltaPhantom<T>>
    where
            for<'de> T: serde::Deserialize<'de>,
{
    pub async fn build(self) -> DispatchDelta<T, reqwest::RequestBuilder> {
        let builder = self.client.build().await;
        let token = self.client.token();
        DispatchDelta::<T, reqwest::RequestBuilder>::new(token, builder, self.error)
    }

    pub async fn send(self) -> tokio::sync::mpsc::Receiver<Delta<T>> {
        let request = self.build().await;
        request.send().await
    }
}
