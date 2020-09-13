use crate::http::{
    AsyncHttpClient, AsyncTryFrom, BlockingHttpClient, GraphResponse, UploadSessionClient,
};
use crate::types::content::Content;
use graph_error::{ErrorMessage, GraphError, GraphFailure, GraphResult};
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::path::PathBuf;

pub struct DispatchRequest<T, Builder> {
    pub client: Builder,
    ident: PhantomData<T>,
    file: Option<PathBuf>,
    error: Option<GraphFailure>,
}

pub type DispatchBlocking<T> = DispatchRequest<T, reqwest::blocking::RequestBuilder>;
pub type DispatchAsync<T> = DispatchRequest<T, reqwest::RequestBuilder>;

impl<T> DispatchBlocking<T> {
    pub fn new(
        req: reqwest::blocking::RequestBuilder,
        file: Option<PathBuf>,
        error: Option<GraphFailure>,
    ) -> DispatchBlocking<T> {
        DispatchBlocking {
            client: req,
            ident: Default::default(),
            file,
            error,
        }
    }
}

impl<T> DispatchBlocking<T> {
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

impl<T> DispatchBlocking<T>
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

impl DispatchBlocking<GraphResponse<Content>> {
    pub fn send(self) -> GraphResult<GraphResponse<Content>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.client.send()?;
        Ok(std::convert::TryFrom::try_from(response)?)
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

impl<T> DispatchAsync<T> {
    pub fn new(
        req: reqwest::RequestBuilder,
        file: Option<PathBuf>,
        error: Option<GraphFailure>,
    ) -> DispatchAsync<T> {
        DispatchAsync {
            client: req,
            ident: Default::default(),
            file,
            error,
        }
    }
}

impl<T> DispatchAsync<T> {
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

impl<T> DispatchAsync<T>
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

impl DispatchAsync<GraphResponse<Content>> {
    pub async fn send(self) -> GraphResult<GraphResponse<Content>> {
        if self.error.is_some() {
            return Err(self.error.unwrap_or_default());
        }
        let response = self.client.send().await?;
        AsyncTryFrom::<reqwest::Response>::try_from(response).await
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
