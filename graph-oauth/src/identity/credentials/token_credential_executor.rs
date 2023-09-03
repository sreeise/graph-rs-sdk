use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{Authority, AzureCloudInstance};

use async_trait::async_trait;
use graph_error::{AuthExecutionResult, AuthorizationResult};
use http::header::ACCEPT;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::tls::Version;
use reqwest::ClientBuilder;
use std::collections::HashMap;
use url::Url;

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

    fn openid_configuration_url(&self) -> AuthorizationResult<Url> {
        Ok(Url::parse(
            format!(
                "{}/{}/2.0/.well-known/openid-configuration",
                self.azure_cloud_instance().as_ref(),
                self.authority().as_ref()
            )
            .as_str(),
        )?)
    }

    fn get_openid_config(&mut self) -> AuthExecutionResult<reqwest::blocking::Response> {
        let open_id_url = self.openid_configuration_url()?;
        let http_client = reqwest::blocking::ClientBuilder::new()
            .min_tls_version(Version::TLS_1_2)
            .https_only(true)
            .build()?;
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let response = http_client
            .get(open_id_url)
            .headers(headers)
            .send()
            .expect("Error on header");

        Ok(response)
    }

    async fn get_openid_config_async(&mut self) -> AuthExecutionResult<reqwest::Response> {
        let open_id_config_url = self.openid_configuration_url()?;
        let http_client = ClientBuilder::new()
            .min_tls_version(Version::TLS_1_2)
            .https_only(true)
            .build()?;
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let response = http_client
            .get(open_id_config_url)
            .headers(headers)
            .send()
            .await?;

        println!("{:#?}", response);

        Ok(response)
    }

    fn execute(&mut self) -> AuthExecutionResult<reqwest::blocking::Response> {
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

    async fn execute_async(&mut self) -> AuthExecutionResult<reqwest::Response> {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::identity::credentials::application_builder::ConfidentialClientApplicationBuilder;

    #[test]
    fn open_id_configuration_url_authority_tenant_id() {
        let open_id = ConfidentialClientApplicationBuilder::new("client-id")
            .with_openid("auth-code", "client-secret")
            .with_tenant("tenant-id")
            .build();

        let url = open_id.openid_configuration_url().unwrap();
        assert_eq!(
            "https://login.microsoftonline.com/tenant-id/2.0/.well-known/openid-configuration",
            url.as_str()
        )
    }

    #[test]
    fn open_id_configuration_url_authority_common() {
        let open_id = ConfidentialClientApplicationBuilder::new("client-id")
            .with_openid("auth-code", "client-secret")
            .build();

        let url = open_id.openid_configuration_url().unwrap();
        assert_eq!(
            "https://login.microsoftonline.com/common/2.0/.well-known/openid-configuration",
            url.as_str()
        )
    }
}
