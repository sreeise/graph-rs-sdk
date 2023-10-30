use crate::http::HttpResponseBuilderExt;
use async_trait::async_trait;
use graph_error::{AuthExecutionResult, ErrorMessage};
use http::Response;
use serde::de::DeserializeOwned;

pub type JsonHttpResponse = http::Response<Result<serde_json::Value, ErrorMessage>>;

#[async_trait]
pub trait AsyncResponseConverterExt {
    async fn into_http_response_async<T: DeserializeOwned>(
        self,
    ) -> AuthExecutionResult<http::Response<Result<T, ErrorMessage>>>;
}

#[async_trait]
impl AsyncResponseConverterExt for reqwest::Response {
    async fn into_http_response_async<T: DeserializeOwned>(
        self,
    ) -> AuthExecutionResult<Response<Result<T, ErrorMessage>>> {
        let status = self.status();
        let url = self.url().clone();
        let headers = self.headers().clone();
        let version = self.version();

        let body: serde_json::Value = self.json().await?;
        let json = body.clone();

        let body_result: Result<T, ErrorMessage> = serde_json::from_value(body)
            .map_err(|_| serde_json::from_value(json.clone()).unwrap_or(ErrorMessage::default()));

        let mut builder = http::Response::builder()
            .url(url)
            .json(&json)
            .status(http::StatusCode::from(&status))
            .version(version);

        for builder_header in builder.headers_mut().iter_mut() {
            builder_header.extend(headers.clone());
        }

        Ok(builder.body(body_result)?)
    }
}

pub trait ResponseConverterExt {
    fn into_http_response<T: DeserializeOwned>(
        self,
    ) -> AuthExecutionResult<http::Response<Result<T, ErrorMessage>>>;
}

impl ResponseConverterExt for reqwest::blocking::Response {
    fn into_http_response<T: DeserializeOwned>(
        self,
    ) -> AuthExecutionResult<Response<Result<T, ErrorMessage>>> {
        let status = self.status();
        let url = self.url().clone();
        let headers = self.headers().clone();
        let version = self.version();

        let body: serde_json::Value = self.json()?;
        let json = body.clone();

        let body_result: Result<T, ErrorMessage> = serde_json::from_value(body)
            .map_err(|_| serde_json::from_value(json.clone()).unwrap_or(ErrorMessage::default()));

        let mut builder = http::Response::builder()
            .url(url)
            .json(&json)
            .status(http::StatusCode::from(&status))
            .version(version);

        for builder_header in builder.headers_mut().iter_mut() {
            builder_header.extend(headers.clone());
        }

        Ok(builder.body(body_result)?)
    }
}
