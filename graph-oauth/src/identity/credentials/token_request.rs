use crate::identity::{AzureCloudInstance, TokenCredential};
use crate::oauth::{AuthorizationSerializer, TokenCredentialOptions};
use async_trait::async_trait;
use graph_error::AuthorizationResult;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::tls::Version;
use reqwest::ClientBuilder;
use url::Url;

#[async_trait]
pub trait TokenRequest: AuthorizationSerializer {
    fn token_credential_options(&self) -> &TokenCredentialOptions;

    fn get_token(&mut self) -> anyhow::Result<reqwest::blocking::Response> {
        let options = self.token_credential_options().clone();
        let uri = self.uri(&options.azure_authority_host)?;
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

        // https://datatracker.ietf.org/doc/html/rfc6749#section-2.3.1
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

    async fn get_token_async(&mut self) -> anyhow::Result<reqwest::Response> {
        let options = self.token_credential_options().clone();
        let uri = self.uri(&options.azure_authority_host)?;
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

        // https://datatracker.ietf.org/doc/html/rfc6749#section-2.3.1
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
