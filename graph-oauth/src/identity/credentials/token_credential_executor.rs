use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{Authority, AzureCloudInstance, TokenCredentialOptions};
use crate::oauth::MsalTokenResponse;
use async_trait::async_trait;
use graph_error::AuthorizationResult;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::tls::Version;
use reqwest::{ClientBuilder, ResponseBuilderExt};
use std::collections::HashMap;
use url::Url;

/*
fn http_response(response: reqwest::blocking::Response) {
    let status = response.status();
    let url = response.url().clone();
    let headers = response.headers().clone();
    let version = response.version();

    let mut builder = http::Response::builder()
        .url(url)
        .status(http::StatusCode::from(&status))
        .version(version);

    for builder_header in builder.headers_mut().iter_mut() {
        builder_header.extend(headers.clone());
    }

    let body_result: reqwest::Result<serde_json::Value> = response.json();
// MsalTokenResponse
    if let Ok(body) = body_result.as_ref() {
        let token: serde_json::Result<MsalTokenResponse> = serde_json::from_value(body.clone());
        builder.json(body.clone());
        builder.body(token)
    } else {

    }
}

 */

#[async_trait]
pub trait TokenCredentialExecutor {
    fn uri(&mut self, azure_authority_host: &AzureCloudInstance) -> AuthorizationResult<Url>;
    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>>;
    fn client_id(&self) -> &String;
    fn authority(&self) -> Authority;
    fn azure_cloud_instance(&self) -> AzureCloudInstance {
        AzureCloudInstance::AzurePublic
    }
    fn basic_auth(&self) -> Option<(String, String)> {
        None
    }
    fn app_config(&self) -> &AppConfig;

    fn execute(&mut self) -> anyhow::Result<reqwest::blocking::Response> {
        let options = self.azure_cloud_instance();
        let uri = self.uri(&options)?;
        let form = self.form_urlencode()?;
        let http_client = reqwest::blocking::ClientBuilder::new()
            .min_tls_version(Version::TLS_1_2)
            .https_only(true)
            .build()?;
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let basic_auth = self.basic_auth();
        if let Some((client_identifier, secret)) = basic_auth {
            Ok(http_client
                .post(uri)
                .basic_auth(client_identifier, Some(secret))
                .headers(headers)
                .form(&form)
                .send()?)
        } else {
            Ok(http_client.post(uri).form(&form).send()?)
        }
    }

    async fn execute_async(&mut self) -> anyhow::Result<reqwest::Response> {
        let azure_cloud_instance = self.azure_cloud_instance();
        let uri = self.uri(&azure_cloud_instance)?;
        let form = self.form_urlencode()?;
        let http_client = ClientBuilder::new()
            .min_tls_version(Version::TLS_1_2)
            .https_only(true)
            .build()?;
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let basic_auth = self.basic_auth();
        if let Some((client_identifier, secret)) = basic_auth {
            Ok(http_client
                .post(uri)
                .basic_auth(client_identifier, Some(secret))
                .headers(headers)
                .form(&form)
                .send()
                .await?)
        } else {
            Ok(http_client
                .post(uri)
                .headers(headers)
                .form(&form)
                .send()
                .await?)
        }
    }
}
