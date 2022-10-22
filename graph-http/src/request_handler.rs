use reqwest::header::HeaderMap;
use reqwest::{Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use graph_error::{GraphFailure, GraphResult};
use crate::GraphResponse;
use async_trait::async_trait;

pub trait RequestBuilderDefinition {
    type Request : RequestDefinition;

    fn build_request(self) -> GraphResult<Self::Request>;
}

pub trait RequestDefinition {
    type Response : ResponseDefinition;

    fn get_url(&self) -> String;
}

#[async_trait]
pub trait ResponseDefinition {
    fn get_url(&self) -> String;

    fn get_headers(&self) -> &HeaderMap;

    async fn convert_json<T>(self) -> GraphResult<T>
        where
                for<'de> T: serde::Deserialize<'de>;

    fn status_code(&self) -> StatusCode;
}

/*
        let request = self.client.build();
        let response = request.send().await?;
        let headers = response.headers().clone();
        let status = response.status();
        let url = GraphUrl::from(response.url());
        let json = response.json().await.map_err(GraphFailure::from)?;
        Ok(GraphResponse::new(url, json, status, headers))
 */

#[async_trait]
pub trait RequestHandler {
    type Request : RequestDefinition;
    type Response : ResponseDefinition;

    async fn handle_get_request(&self, request: Self::Request) -> GraphResult<Self::Response>;
}

#[async_trait]
impl ResponseDefinition for reqwest::Response {
    fn get_url(&self) -> String {
        self.url().to_string()
    }

    fn get_headers(&self) -> &HeaderMap {
        self.headers()
    }

    async fn convert_json<T>(self) -> GraphResult<T> where for<'de> T: Deserialize<'de> {
        self.json().await.map_err(GraphFailure::from)
    }

    fn status_code(&self) -> StatusCode {
        self.status()
    }
}

impl RequestDefinition for reqwest::Request {
    type Response = reqwest::Response;

    fn get_url(&self) -> String {
        self.url().to_string()
    }
}

impl RequestBuilderDefinition for reqwest::RequestBuilder {
    type Request = reqwest::Request;

    fn build_request(self) -> GraphResult<Self::Request> {
        self.build().map_err(GraphFailure::from)
    }
}

#[async_trait]
impl RequestHandler for reqwest::Client {
    type Request = reqwest::Request;
    type Response = reqwest::Response;

    async fn handle_get_request(&self, request: Self::Request) -> GraphResult<Self::Response> {
        self.execute(request).await.map_err(GraphFailure::from)
    }
}
