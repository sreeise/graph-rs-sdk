use crate::client::*;
use crate::http::{AsyncClient, BlockingClient, GraphResponse, RequestClient, UploadSessionClient};
use crate::types::delta::{Delta, NextLink};
use crate::types::{content::Content, delta::DeltaRequest};
use graph_error::{GraphFailure, GraphResult};
use reqwest::header::{HeaderValue, IntoHeaderName, CONTENT_TYPE};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

pub struct IntoResponse<'a, T, Client>
where
    Client: RequestClient,
{
    client: &'a Graph<Client>,
    ident: PhantomData<T>,
    error: RefCell<Option<GraphFailure>>,
}

pub type IntoResBlocking<'a, T> = IntoResponse<'a, T, BlockingClient>;
pub type IntoResAsync<'a, T> = IntoResponse<'a, T, AsyncClient>;

impl<'a, T, Client> IntoResponse<'a, T, Client>
where
    Client: RequestClient,
{
    pub fn new(client: &'a Graph<Client>) -> IntoResponse<'a, T, Client> {
        IntoResponse {
            client,
            ident: PhantomData,
            error: RefCell::new(None),
        }
    }

    pub(crate) fn new_error(
        client: &'a Graph<Client>,
        error: GraphFailure,
    ) -> IntoResponse<'a, T, Client> {
        IntoResponse {
            client,
            ident: PhantomData,
            error: RefCell::new(Some(error)),
        }
    }

    pub fn query(&self, key: &str, value: &str) -> &Self {
        self.client.request().as_mut().append_query_pair(key, value);
        self
    }

    pub fn select(&self, value: &[&str]) -> &Self {
        self.client.request().as_mut().select(value);
        self
    }

    pub fn expand(&self, value: &[&str]) -> &Self {
        self.client.request().as_mut().expand(value);
        self
    }

    pub fn filter(&self, value: &[&str]) -> &Self {
        self.client.request().as_mut().filter(value);
        self
    }

    pub fn order_by(&self, value: &[&str]) -> &Self {
        self.client.request().as_mut().order_by(value);
        self
    }

    pub fn search(&self, value: &str) -> &Self {
        self.client.request().as_mut().search(value);
        self
    }

    pub fn format(&self, value: &str) -> &Self {
        self.client.request().as_mut().format(value);
        self
    }

    pub fn skip(&self, value: &str) -> &Self {
        self.client.request().as_mut().skip(value);
        self
    }

    pub fn top(&self, value: &str) -> &Self {
        self.client.request().as_mut().top(value);
        self
    }

    pub fn header(&self, name: impl IntoHeaderName, value: HeaderValue) -> &Self {
        self.client.request().header(name, value);
        self
    }
}

impl<'a, T> IntoResBlocking<'a, T> {
    fn delta<U: 'static + Send + NextLink + Clone>(&self) -> Receiver<Delta<U>>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        let (sender, receiver) = channel();
        if self.error.borrow().is_some() {
            sender.send(Delta::Done(self.error.replace(None))).unwrap();
            return receiver;
        }

        let response: GraphResult<GraphResponse<U>> = self.client.request().execute();
        if let Err(err) = response {
            sender.send(Delta::Done(Some(err))).unwrap();
            return receiver;
        }

        let token = self.client.request().token().clone();
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
                        let value_res: GraphResult<U> = response.json().map_err(GraphFailure::from);
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

    pub fn json<U>(&self) -> GraphResult<U>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        if self.error.borrow().is_some() {
            return Err(self.error.replace(None).unwrap());
        }
        let response = self.client.request().response()?;
        Ok(response.json()?)
    }
}

impl<'a, T> IntoResBlocking<'a, T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    pub fn send(&self) -> GraphResult<GraphResponse<T>> {
        if self.error.borrow().is_some() {
            return Err(self.error.replace(None).unwrap());
        }
        self.client.request().execute()
    }
}

impl<'a> IntoResBlocking<'a, UploadSessionClient<BlockingClient>> {
    pub fn send(&self) -> GraphResult<UploadSessionClient<BlockingClient>> {
        if self.error.borrow().is_some() {
            return Err(self.error.replace(None).unwrap());
        }
        self.client.request().upload_session()
    }
}

impl<'a> IntoResBlocking<'a, GraphResponse<Content>> {
    pub fn send(&self) -> GraphResult<GraphResponse<Content>> {
        if self.error.borrow().is_some() {
            return Err(self.error.replace(None).unwrap());
        }
        let response = self.client.request().response()?;
        Ok(GraphResponse::try_from(response)?)
    }
}

impl<'a, T: 'static + Send + NextLink + Clone> IntoResBlocking<'a, DeltaRequest<T>>
where
    for<'de> T: serde::Deserialize<'de>,
{
    pub fn send(&self) -> Receiver<Delta<T>> {
        self.delta::<T>()
    }
}

// Async Impl

// We are ok to do async fn's here as long as we don't hold a Send
// type across an await. This makes easier to do async fn's event though
// IntoResponse stores non-Send types and is itself a non-Send type.
// See: https://rust-lang.github.io/async-book/07_workarounds/04_send_approximation.html

impl<'a, T> IntoResAsync<'a, T> {
    pub async fn json<U>(&self) -> GraphResult<U>
    where
        for<'de> U: serde::Deserialize<'de> + Send + Sync,
    {
        if self.error.borrow().is_some() {
            return Err(self.error.replace(None).unwrap());
        }
        let request = self.client.request().build();
        let response = request.send().await?;
        response.json().await.map_err(GraphFailure::from)
    }
}

impl<'a, T> IntoResAsync<'a, T>
where
    for<'de> T: serde::Deserialize<'de> + Send + Sync,
{
    pub async fn send(&self) -> GraphResult<GraphResponse<T>> {
        if self.error.borrow().is_some() {
            return Err(self.error.replace(None).unwrap());
        }
        let request = self.client.request().build();
        let response = request.send().await?;
        GraphResponse::try_from_async(response).await
    }
}

impl<'a> IntoResAsync<'a, GraphResponse<Content>> {
    pub async fn send(&self) -> GraphResult<GraphResponse<Content>> {
        if self.error.borrow().is_some() {
            return Err(self.error.replace(None).unwrap());
        }
        let request = self.client.request().build();
        let response = request.send().await?;
        GraphResponse::try_from_async_content(response).await
    }
}

impl<'a> IntoResAsync<'a, UploadSessionClient<AsyncClient>> {
    pub async fn send(&self) -> GraphResult<UploadSessionClient<AsyncClient>> {
        if self.error.borrow().is_some() {
            return Err(self.error.replace(None).unwrap());
        }
        let mut request = self.client.request();
        request.upload_session().await
    }
}
