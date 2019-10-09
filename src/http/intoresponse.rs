use crate::client::*;
use crate::http::{GraphResponse, UploadSessionClient};
use crate::types::content::Content;
use graph_error::{GraphFailure, GraphResult};
use reqwest::header::{HeaderValue, ACCEPT};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

/// A trait for sending an API request and converting the response
/// to a suitable Rust type.
pub trait ToResponse {
    type Output;

    fn send(&self) -> Self::Output;
    fn value(&self) -> GraphResult<GraphResponse<serde_json::Value>>;
}

pub struct IntoResponse<'a, I, T> {
    client: &'a Graph,
    ident: PhantomData<I>,
    phantom: PhantomData<T>,
    error: RefCell<Option<GraphFailure>>,
}

impl<'a, I, T> IntoResponse<'a, I, T> {
    pub fn new(client: &'a Graph) -> IntoResponse<I, T> {
        IntoResponse {
            client,
            ident: PhantomData,
            phantom: PhantomData,
            error: RefCell::new(None),
        }
    }

    pub(crate) fn new_error(client: &'a Graph, error: GraphFailure) -> IntoResponse<I, T> {
        IntoResponse {
            client,
            ident: PhantomData,
            phantom: PhantomData,
            error: RefCell::new(Some(error)),
        }
    }

    pub fn select(&self, value: &[&str]) -> &Self {
        self.client.builder().as_mut().select(value);
        self
    }

    pub fn expand(&self, value: &[&str]) -> &Self {
        self.client.builder().as_mut().expand(value);
        self
    }

    pub fn filter(&self, value: &[&str]) -> &Self {
        self.client.builder().as_mut().filter(value);
        self
    }

    pub fn order_by(&self, value: &[&str]) -> &Self {
        self.client.builder().as_mut().order_by(value);
        self
    }

    pub fn search(&self, value: &str) -> &Self {
        self.client.builder().as_mut().search(value);
        self
    }

    pub fn format(&self, value: &str) -> &Self {
        self.client.builder().as_mut().format(value);
        self
    }

    pub fn skip(&self, value: &str) -> &Self {
        self.client.builder().as_mut().skip(value);
        self
    }

    pub fn top(&self, value: &str) -> &Self {
        self.client.builder().as_mut().top(value);
        self
    }

    fn to_serde_json(&self) -> GraphResult<GraphResponse<serde_json::Value>> {
        if self.error.borrow().is_some() {
            return Err(self.error.replace(None).unwrap());
        }
        let mut response = self.client.request().response(self.client.take_builder())?;
        let value: serde_json::Value = response.json()?;
        Ok(GraphResponse::new(response, value))
    }

    pub fn json<U>(&self) -> GraphResult<U>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        if self.error.borrow().is_some() {
            return Err(self.error.replace(None).unwrap());
        }
        let mut response = self.client.request().response(self.client.take_builder())?;
        Ok(response.json()?)
    }
}

impl<'a, I, T> ToResponse for IntoResponse<'a, I, T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    type Output = GraphResult<GraphResponse<T>>;

    fn send(&self) -> Self::Output {
        if self.error.borrow().is_some() {
            return Err(self.error.replace(None).unwrap());
        }
        let builder = self.client.take_builder();
        self.client.request().execute(builder)
    }

    fn value(&self) -> GraphResult<GraphResponse<serde_json::Value>> {
        self.to_serde_json()
    }
}

impl<'a, I> ToResponse for IntoResponse<'a, I, UploadSessionClient> {
    type Output = GraphResult<UploadSessionClient>;

    fn send(&self) -> Self::Output {
        if self.error.borrow().is_some() {
            return Err(self.error.replace(None).unwrap());
        }
        self.client
            .request()
            .upload_session(self.client.take_builder())
    }

    fn value(&self) -> GraphResult<GraphResponse<serde_json::Value>> {
        self.to_serde_json()
    }
}

impl<'a, I> ToResponse for IntoResponse<'a, I, GraphResponse<Content>> {
    type Output = GraphResult<GraphResponse<Content>>;

    fn send(&self) -> Self::Output {
        if self.error.borrow().is_some() {
            return Err(self.error.replace(None).unwrap());
        }
        let builder = self.client.take_builder();
        let response = self.client.request().response(builder)?;
        Ok(GraphResponse::try_from(response)?)
    }

    fn value(&self) -> GraphResult<GraphResponse<serde_json::Value>> {
        if self.error.borrow().is_some() {
            return Err(self.error.replace(None).unwrap());
        }
        let builder = self.client.take_builder();
        let mut response = self.client.request().response(builder)?;
        if let Ok(content) = response.text() {
            Ok(GraphResponse::new(
                response,
                serde_json::json!({ "content": content }),
            ))
        } else {
            Ok(GraphResponse::new(
                response,
                serde_json::json!({ "content": "" }),
            ))
        }
    }
}

pub struct DeltaRequest;

impl<'a, I> ToResponse for IntoResponse<'a, I, DeltaRequest> {
    type Output = GraphResult<Receiver<serde_json::Value>>;

    fn send(&self) -> Self::Output {
        if self.error.borrow().is_some() {
            return Err(self.error.replace(None).unwrap());
        }
        let builder = self.client.take_builder();
        let response: GraphResponse<serde_json::Value> = self.client.request().execute(builder)?;
        let (sender, receiver) = channel();
        sender.send(response.value().clone()).unwrap();
        let token = self.client.request().token().clone();

        thread::spawn(move || {
            let mut next_link = response.value()["@odata.nextLink"]
                .as_str()
                .map(|s| s.to_string());
            let client = reqwest::Client::new();
            while let Some(next) = next_link {
                let mut res = client
                    .post(next.as_str())
                    .header(ACCEPT, HeaderValue::from_static("application/json"))
                    .bearer_auth(token.as_str())
                    .send()
                    .unwrap();
                let value: serde_json::Value = res.json().unwrap();
                next_link = value["@odata.nextLink"].as_str().map(|s| s.to_string());
                sender.send(value).unwrap();
            }
        });

        Ok(receiver)
    }

    fn value(&self) -> GraphResult<GraphResponse<serde_json::Value>> {
        self.to_serde_json()
    }
}
