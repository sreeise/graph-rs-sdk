use crate::client::*;
use crate::http::{GraphResponse, UploadSessionClient};
use crate::url::UrlOrdering;
use graph_error::{GraphError, GraphFailure, GraphResult};
use graph_rs_types::complextypes::UploadSession;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::path::Path;

/// A trait for sending an API request and converting the response
/// to a suitable Rust type.
pub trait ToResponse {
    type Output;

    fn send(&self) -> Self::Output;
}

pub struct ResponseClient<'a, I, T> {
    client: &'a Graph,
    item: PhantomData<T>,
    ident: PhantomData<I>,
}

impl<'a, I, T> ResponseClient<'a, I, T> {
    pub fn new(client: &'a Graph) -> ResponseClient<'a, I, T> {
        ResponseClient {
            client,
            item: PhantomData,
            ident: PhantomData,
        }
    }

    pub fn by_id(&'a self, id: &str) -> IntoResponse<'a, I, T> {
        if self.client.ident().eq(&Ident::Me) {
            self.client
                .request()
                .insert(UrlOrdering::Id(id.into()))
                .format_ord();
        } else {
            self.client
                .request()
                .insert(UrlOrdering::ResourceId(id.into()))
                .format_ord();
        }
        IntoResponse::new(self.client)
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

impl<'a, T> ResponseClient<'a, IdentifyMe, T> {
    pub fn by_path<P: AsRef<Path>>(&self, path: P) -> IntoResponse<'a, IdentifyMe, T> {
        self.client
            .request()
            .remove(UrlOrdering::FileName("".into()))
            .replace(UrlOrdering::RootOrItem("root:".into()))
            .replace(UrlOrdering::Path(path.as_ref().to_path_buf()))
            .format_ord();
        IntoResponse::new(self.client)
    }
}

impl<'a, T> ResponseClient<'a, IdentifyCommon, T> {
    pub fn by_ids(
        &'a self,
        item_id: &str,
        resource_id: &str,
    ) -> IntoResponse<'a, IdentifyCommon, T> {
        self.client
            .request()
            .insert(UrlOrdering::ResourceId(resource_id.into()))
            .insert(UrlOrdering::Id(item_id.into()))
            .format_ord();
        IntoResponse::new(self.client)
    }

    pub fn by_path_id<P: AsRef<Path>>(
        &'a self,
        resource_id: &str,
        path: P,
    ) -> IntoResponse<'a, IdentifyCommon, T> {
        self.client
            .request()
            .insert(UrlOrdering::ResourceId(resource_id.into()))
            .remove(UrlOrdering::FileName("".into()))
            .replace(UrlOrdering::RootOrItem("root:".into()))
            .insert(UrlOrdering::Path(path.as_ref().to_path_buf()))
            .format_ord();
        IntoResponse::new(self.client)
    }
}

// For requests that don't return JSON in the body but does return information
// on triggering status events.
impl<'a, I> ToResponse for ResponseClient<'a, I, GraphResponse<()>> {
    type Output = GraphResult<GraphResponse<()>>;

    fn send(&self) -> Self::Output {
        Ok(GraphResponse::new(self.client.request().response()?, ()))
    }
}

impl<'a, I, T> ToResponse for ResponseClient<'a, I, T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    type Output = GraphResult<GraphResponse<T>>;

    fn send(&self) -> Self::Output {
        self.client.request().graph_response()
    }
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
        let builder = self.client.request().builder();
        let mut response = builder.send()?;
        let status = response.status().as_u16();
        if GraphError::is_error(status) {
            return Err(GraphFailure::try_from(&mut response).unwrap_or_default());
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
