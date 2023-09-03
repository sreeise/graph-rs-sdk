use crate::http::HttpResponseBuilderExt;
use async_trait::async_trait;
use graph_error::{ErrorMessage, GraphResult};
use http::Response;
use serde::de::DeserializeOwned;

/*
pub async fn into_http_response_async<T: DeserializeOwned>(response: reqwest::Response) -> GraphResult<http::Response<Result<T, ErrorMessage>>> {
    let status = response.status();
    let url = response.url().clone();
    let headers = response.headers().clone();
    let version = response.version();

    let body: serde_json::Value = response.json().await?;
    let json = body.clone();

    let body_result: Result<T, ErrorMessage> = serde_json::from_value(body)
        .map_err(|_| serde_json::from_value(json.clone()).unwrap_or(ErrorMessage::default()));

    let mut builder = http::Response::builder()
        .url(url)
        .json(&json)
        .status(http::StatusCode::from(&status))
        .version(version);

    Ok(builder.body(body_result)?)
}
 */

#[async_trait]
pub trait ResponseConverterExt {
    async fn into_json_http_response_async<T: DeserializeOwned>(
        self,
    ) -> GraphResult<http::Response<Result<T, ErrorMessage>>>;
}

#[async_trait]
impl ResponseConverterExt for reqwest::Response {
    async fn into_json_http_response_async<T: DeserializeOwned>(
        self,
    ) -> GraphResult<Response<Result<T, ErrorMessage>>> {
        let status = self.status();
        let url = self.url().clone();
        let _headers = self.headers().clone();
        let version = self.version();

        let body: serde_json::Value = self.json().await?;
        let json = body.clone();

        let body_result: Result<T, ErrorMessage> = serde_json::from_value(body)
            .map_err(|_| serde_json::from_value(json.clone()).unwrap_or(ErrorMessage::default()));

        let builder = http::Response::builder()
            .url(url)
            .json(&json)
            .status(http::StatusCode::from(&status))
            .version(version);

        Ok(builder.body(body_result)?)
    }
}
