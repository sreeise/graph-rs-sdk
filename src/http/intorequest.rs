use crate::http::{
    AsyncHttpClient, AsyncTryFrom, BlockingHttpClient, GraphResponse, UploadSessionClient,
};
use crate::types::content::Content;
use graph_error::{ErrorMessage, GraphError, GraphFailure, GraphResult};
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::path::PathBuf;

pub struct IntoRequest<T, Builder> {
    client: Builder,
    ident: PhantomData<T>,
    file: Option<PathBuf>,
    error: Option<GraphFailure>,
}

pub type IntoReqBlocking<T> = IntoRequest<T, reqwest::blocking::RequestBuilder>;
pub type IntoReqAsync<T> = IntoRequest<T, reqwest::RequestBuilder>;

impl<T> IntoReqBlocking<T> {
    pub fn new(
        req: reqwest::blocking::RequestBuilder,
        file: Option<PathBuf>,
        error: Option<GraphFailure>,
    ) -> IntoReqBlocking<T> {
        IntoReqBlocking {
            client: req,
            ident: Default::default(),
            file,
            error,
        }
    }
}

impl<T> IntoReqBlocking<T> {
    pub fn json<U>(self) -> GraphResult<U>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.client.send()?;
        Ok(response.json()?)
    }
}

impl<T> IntoReqBlocking<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    pub fn send(self) -> GraphResult<GraphResponse<T>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.client.send()?;
        Ok(std::convert::TryFrom::try_from(response)?)
    }
}

impl IntoReqBlocking<GraphResponse<Content>> {
    pub fn send(self) -> GraphResult<GraphResponse<Content>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.client.send()?;
        Ok(std::convert::TryFrom::try_from(response)?)
    }
}

impl IntoReqBlocking<UploadSessionClient<BlockingHttpClient>> {
    pub fn send(self) -> GraphResult<UploadSessionClient<BlockingHttpClient>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }

        let file = self
            .file
            .ok_or_else(|| GraphFailure::invalid("file for upload session"))?;

        let response = self.client.send()?;
        if let Ok(mut error) = GraphError::try_from(&response) {
            let error_message: GraphResult<ErrorMessage> =
                response.json().map_err(GraphFailure::from);
            if let Ok(message) = error_message {
                error.set_error_message(message);
            }
            return Err(GraphFailure::GraphError(error));
        }

        let upload_session: serde_json::Value = response.json()?;
        let mut session = UploadSessionClient::new(upload_session)?;
        session.set_file(file)?;
        Ok(session)
    }
}

// Async Impl

impl<T> IntoReqAsync<T> {
    pub fn new(
        req: reqwest::RequestBuilder,
        file: Option<PathBuf>,
        error: Option<GraphFailure>,
    ) -> IntoReqAsync<T> {
        IntoReqAsync {
            client: req,
            ident: Default::default(),
            file,
            error,
        }
    }
}

impl<T> IntoReqAsync<T> {
    pub async fn json<U>(self) -> GraphResult<U>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.client.send().await?;
        Ok(response.json().await?)
    }
}

impl<T> IntoReqAsync<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    pub async fn send(self) -> GraphResult<GraphResponse<T>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.client.send().await?;
        AsyncTryFrom::<reqwest::Response>::try_from(response).await
    }
}

impl IntoReqAsync<GraphResponse<Content>> {
    pub async fn send(self) -> GraphResult<GraphResponse<Content>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.client.send().await?;
        AsyncTryFrom::<reqwest::Response>::try_from(response).await
    }
}

impl IntoReqAsync<UploadSessionClient<AsyncHttpClient>> {
    pub async fn send(self) -> GraphResult<UploadSessionClient<AsyncHttpClient>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }

        let file = self
            .file
            .ok_or_else(|| GraphFailure::invalid("file for upload session"))?;

        let response = self.client.send().await?;
        if let Ok(mut error) = GraphError::try_from(&response) {
            let error_message: GraphResult<ErrorMessage> =
                response.json().await.map_err(GraphFailure::from);
            if let Ok(message) = error_message {
                error.set_error_message(message);
            }
            return Err(GraphFailure::GraphError(error));
        }

        let upload_session: serde_json::Value = response.json().await?;
        let mut session = UploadSessionClient::new_async(upload_session)?;
        session.set_file_async(file).await?;
        Ok(session)
    }
}
