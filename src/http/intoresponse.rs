use crate::client::*;
use crate::http::{GraphResponse, UploadSessionClient};
use graph_error::{GraphFailure, GraphResult};
use graph_rs_types::complextypes::UploadSession;
use std::marker::PhantomData;

/// A trait for sending an API request and converting the response
/// to a suitable Rust type.
pub trait ToResponse {
    type Output;

    fn send(&self) -> Self::Output;
}

pub struct IntoResponse<'a, I, T> {
    client: &'a Graph,
    ident: PhantomData<I>,
    phantom: PhantomData<T>,
}

impl<'a, I, T> IntoResponse<'a, I, T> {
    pub fn new(client: &'a Graph) -> IntoResponse<I, T> {
        IntoResponse {
            client,
            ident: PhantomData,
            phantom: PhantomData,
        }
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

    pub fn value(&self) -> GraphResult<GraphResponse<serde_json::Value>> {
        let mut response = self.client.request().response()?;
        let value: serde_json::Value = response.json()?;
        Ok(GraphResponse::new(response, value))
    }

    pub fn json<U>(&self) -> GraphResult<U>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        self.client.request().json()
    }
}

impl<'a, I> ToResponse for IntoResponse<'a, I, GraphResponse<()>> {
    type Output = GraphResult<GraphResponse<()>>;

    fn send(&self) -> Self::Output {
        Ok(GraphResponse::new(self.client.request().response()?, ()))
    }
}

impl<'a, I, T> ToResponse for IntoResponse<'a, I, T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    type Output = GraphResult<GraphResponse<T>>;

    fn send(&self) -> Self::Output {
        self.client.request().graph_response()
    }
}

impl<'a, I> ToResponse for IntoResponse<'a, I, UploadSessionClient> {
    type Output = GraphResult<UploadSessionClient>;

    fn send(&self) -> Self::Output {
        let mut response = self.client.request().builder().send()?;
        if let Some(err) = GraphFailure::from_response(&mut response) {
            return Err(err);
        }

        let upload_session: UploadSession = response.json()?;
        let mut session = UploadSessionClient::new(upload_session)?;
        let file = self
            .client
            .request()
            .upload_session_file
            .clone()
            .ok_or_else(|| GraphFailure::none_err("file for upload session"))?;
        session.set_file(file)?;
        Ok(session)
    }
}
