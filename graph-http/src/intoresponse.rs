use crate::async_client::AsyncHttpClient;
use crate::blocking_client::BlockingHttpClient;
use crate::traits::{AsyncTryFrom, ODataLink, ODataNextLink, ODataNextLinkBlocking};
use crate::types::{Delta, DeltaPhantom, NoContent};
use crate::uploadsession::UploadSessionClient;
use crate::url::GraphUrl;
use crate::{
    AsyncDownload, BlockingDownload, DispatchAsync, DispatchBlocking, DispatchDelta, GraphResponse,
    RequestClient,
};
use bytes::Bytes;
use graph_error::{GraphFailure, GraphResult, WithGraphError};
use reqwest::header::{CONTENT_TYPE, HeaderValue, IntoHeaderName};
use std::marker::PhantomData;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;

pub struct IntoResponse<'a, T, Client>
where
    Client: RequestClient,
{
    client: &'a Client,
    ident: PhantomData<T>,
    error: Option<GraphFailure>,
}

pub type IntoResponseBlocking<'a, T> = IntoResponse<'a, T, BlockingHttpClient>;
pub type IntoResponseAsync<'a, T> = IntoResponse<'a, T, AsyncHttpClient>;

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

    pub fn new_error(client: &'a Client, error: GraphFailure) -> IntoResponse<'a, T, Client> {
        IntoResponse {
            client,
            ident: PhantomData,
            error: Some(error),
        }
    }

    /// Using this method set the timeout not only for this API call but also for the entire lifetime of the Client.
    pub fn set_timeout(self, duration: Duration) -> Self {
        self.client.set_timeout(duration);
        self
    }

    pub fn with_next_link_calls(self) -> Self {
        self.client.follow_next_links(true);
        self
    }

    pub fn query(self, key: &str, value: &str) -> Self {
        self.client.url_mut(|url| {
            url.append_query_pair(key, value);
        });
        self
    }

    /// Filters properties (columns).
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#select-parameter)
    pub fn select(self, value: &[&str]) -> Self {
        self.client.url_mut(|url| {
            url.select(value);
        });
        self
    }

    /// Retrieves related resources.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#expand-parameter)
    pub fn expand(self, value: &[&str]) -> Self {
        self.client.url_mut(|url| {
            url.expand(value);
        });
        self
    }

    /// Filters results (rows).
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#filter-parameter)
    pub fn filter(self, value: &[&str]) -> Self {
        self.client.url_mut(|url| {
            url.filter(value);
        });
        self
    }

    /// Orders results.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#orderby-parameter)
    pub fn order_by(self, value: &[&str]) -> Self {
        self.client.url_mut(|url| {
            url.order_by(value);
        });
        self
    }

    /// Returns results based on search criteria.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#search-parameter)
    pub fn search(self, value: &str) -> Self {
        self.client.url_mut(|url| {
            url.search(value);
        });
        self
    }

    /// Returns the results in the specified media format.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#format-parameter)
    pub fn format(self, value: &str) -> Self {
        self.client.url_mut(|url| {
            url.format(value);
        });
        self
    }

    /// Indexes into a result set. Also used by some APIs to implement paging and can be used
    /// together with $top to manually page results.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#skip-parameter)
    pub fn skip(self, value: &str) -> Self {
        self.client.url_mut(|url| {
            url.skip(value);
        });
        self
    }

    /// Sets the page size of results.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#top-parameter)
    pub fn top(self, value: &str) -> Self {
        self.client.url_mut(|url| {
            url.top(value);
        });
        self
    }

    /// Retrieves the next page of results from result sets that span multiple pages.
    /// (Some APIs use $skip instead.)
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#skiptoken-parameter)
    pub fn skip_token(self, value: &str) -> Self {
        self.client.url_mut(|url| {
            url.skip_token(value);
        });
        self
    }

    /// Retrieves the total count of matching resources.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#count-parameter)
    pub fn count(self, value: &str) -> Self {
        self.client.url_mut(|url| {
            url.count(value);
        });
        self
    }

    pub fn cast(self, value: &str) -> Self {
        self.client.url_mut(|url| {
            url.cast(value);
        });
        self
    }

    pub fn header<H: IntoHeaderName>(self, name: H, value: HeaderValue) -> Self {
        self.client.header(name, value);
        self
    }
}

impl<'a, T> IntoResponseBlocking<'a, T> {
    pub fn json<U>(self) -> GraphResult<GraphResponse<U>>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }

        let response = self.client.response()?;
        let headers = response.headers().clone();
        let status = response.status();
        let url = GraphUrl::from(response.url());
        let json = response.json().map_err(GraphFailure::from)?;
        Ok(GraphResponse::new(url, json, status, headers))
    }

    pub fn json_with_next_links<U: 'static + Send + ODataNextLinkBlocking + Clone>(self) -> Receiver<Option<GraphResult<GraphResponse<U>>>>
        where
                for<'de> U: serde::Deserialize<'de>,
    {
        let (sender, receiver) = channel();
        if self.error.is_some() {
            sender
                .send(Some(Err(self.error.unwrap_or_default())))
                .unwrap();
            return receiver;
        }

        let initial_res: GraphResult<reqwest::blocking::Response> = self.client.response();
        let response: GraphResult<GraphResponse<U>> = std::convert::TryFrom::try_from(initial_res);

        if let Err(err) = response {
            sender.send(Some(Err(err))).unwrap();
            return receiver;
        }

        let token = self.client.token();
        let response = response.unwrap();
        let mut next_link = response.body().next_link();
        sender.send(Some(Ok(response))).unwrap();

        thread::spawn(move || {
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
                    sender.send(Some(Err(err))).unwrap();
                } else {
                    match res.unwrap().with_graph_error() {
                        Ok(response) => {
                            let url = GraphUrl::from(response.url());
                            let headers = response.headers().clone();
                            let status = response.status();
                            match response.json::<U>() {
                                Ok(value) => {
                                    next_link = value.next_link();
                                    sender
                                        .send(Some(Ok(GraphResponse::new(
                                            url, value, status, headers,
                                        ))))
                                        .unwrap();
                                }
                                Err(err) => {
                                    next_link = None;
                                    sender.send(Some(Err(err.into()))).unwrap();
                                }
                            }
                        }
                        Err(err) => {
                            next_link = None;
                            sender.send(Some(Err(GraphFailure::GraphError(err)))).unwrap();
                        }
                    }
                }
            }
        });

        receiver
    }

    pub fn text(self) -> GraphResult<GraphResponse<String>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }

        let response = self.client.response()?;
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

        let response = self.client.response()?;
        let headers = response.headers().clone();
        let status = response.status();
        let url = GraphUrl::from(response.url());
        let bytes = response.bytes().map_err(GraphFailure::from)?;
        Ok(GraphResponse::new(url, bytes, status, headers))
    }
}

impl<'a, T> IntoResponseBlocking<'a, T>
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

impl<'a> IntoResponseBlocking<'a, UploadSessionClient<BlockingHttpClient>> {
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

impl<'a> IntoResponseBlocking<'a, NoContent> {
    pub fn build(self) -> DispatchBlocking<GraphResponse<NoContent>> {
        let builder = self.client.build();
        DispatchBlocking::new(builder, None, self.error)
    }

    pub fn send(self) -> GraphResult<GraphResponse<serde_json::Value>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.client.response()?;
        GraphResponse::<serde_json::Value>::from_no_content(response)
    }
}

impl<'a, T: 'static + Send + ODataLink + Clone> IntoResponseBlocking<'a, DeltaPhantom<T>>
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

impl<'a> IntoResponseBlocking<'a, BlockingDownload> {
    pub fn download<P: AsRef<Path>>(self, path: P) -> GraphResult<BlockingDownload> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        self.client.set_download_dir(path.as_ref().to_path_buf());
        Ok(self.client.download())
    }
}

// Async Impl

impl<'a, T> IntoResponseAsync<'a, T> {
    pub async fn json<U, V>(self) -> GraphResult<GraphResponse<U>>
    where
        for<'de> U: serde::Deserialize<'de> + ODataNextLink<V>,
    {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }

        let request = self.client.build().await;
        let response = request.send().await?;
        let headers = response.headers().clone();
        let status = response.status();
        let url = GraphUrl::from(response.url());
        let mut json: U = response.json().await.map_err(GraphFailure::from)?;
        let mut values: Vec<V> = vec![];

        if self.client.client.lock().follow_next_links {
            loop {
                values.append(&mut json.value());
                match json.next_link() {
                    Some(next_link) => {
                        let url = GraphUrl::parse(&next_link)?;
                        self.client.set_url(url);
                        let request = self.client.build().await;
                        let response = request.send().await?;
                        json = response.json().await.map_err(GraphFailure::from)?;
                    }
                    None => {
                        break;
                    }
                }
            }
            json.value().append(&mut values);
            Ok(GraphResponse::new(url, json, status, headers))
        } else {
            Ok(GraphResponse::new(url, json, status, headers))
        }
    }

    pub async fn text(self) -> GraphResult<GraphResponse<String>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }

        let request = self.client.build().await;
        let response = request.send().await?;
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

        let request = self.client.build().await;
        let response = request.send().await?;
        let headers = response.headers().clone();
        let status = response.status();
        let url = GraphUrl::from(response.url());
        let bytes = response.bytes().await.map_err(GraphFailure::from)?;
        Ok(GraphResponse::new(url, bytes, status, headers))
    }
}

impl<'a, T> IntoResponseAsync<'a, T>
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
        let request = self.client.build().await;
        let response = request.send().await?;
        AsyncTryFrom::<reqwest::Response>::async_try_from(response).await
    }
}

impl<'a> IntoResponseAsync<'a, NoContent> {
    pub async fn build(self) -> DispatchAsync<GraphResponse<NoContent>> {
        let builder = self.client.build().await;
        DispatchAsync::new(builder, None, self.error)
    }

    pub async fn send(self) -> GraphResult<GraphResponse<serde_json::Value>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }

        let request = self.client.build().await;
        let response = request.send().await?;
        GraphResponse::<serde_json::Value>::async_from_no_content(response).await
    }
}

impl<'a> IntoResponseAsync<'a, UploadSessionClient<AsyncHttpClient>> {
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

impl<'a, T: 'static + Send + ODataLink + Clone> IntoResponseAsync<'a, DeltaPhantom<T>>
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

impl<'a> IntoResponseAsync<'a, AsyncDownload> {
    pub async fn download<P: AsRef<Path>>(self, path: P) -> GraphResult<AsyncDownload> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        self.client.set_download_dir(path.as_ref().to_path_buf());
        Ok(self.client.download().await)
    }
}
