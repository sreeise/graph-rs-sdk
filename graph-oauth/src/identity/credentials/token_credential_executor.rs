use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{Authority, AzureCloudInstance};

use async_trait::async_trait;
use dyn_clone::DynClone;
use graph_error::{AuthExecutionResult, AuthorizationResult};
use http::header::ACCEPT;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::tls::Version;
use reqwest::ClientBuilder;
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

dyn_clone::clone_trait_object!(TokenCredentialExecutor);

#[async_trait]
pub trait TokenCredentialExecutor: DynClone {
    fn is_initialized(&self) -> bool {
        true
    }

    fn uri(&mut self) -> AuthorizationResult<Url>;

    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>>;

    fn client_id(&self) -> &Uuid {
        &self.app_config().client_id
    }

    fn authority(&self) -> Authority {
        self.app_config().authority.clone()
    }

    fn azure_cloud_instance(&self) -> AzureCloudInstance {
        self.app_config().azure_cloud_instance
    }

    fn basic_auth(&self) -> Option<(String, String)> {
        None
    }

    fn app_config(&self) -> &AppConfig;

    fn extra_header_parameters(&self) -> &HeaderMap {
        &self.app_config().extra_header_parameters
    }

    fn extra_query_parameters(&self) -> &HashMap<String, String> {
        &self.app_config().extra_query_parameters
    }

    fn openid_configuration_url(&self) -> AuthorizationResult<Url> {
        Ok(Url::parse(
            format!(
                "{}/{}/v2.0/.well-known/openid-configuration",
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
        let mut uri = self.uri()?;
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

        let extra_headers = self.extra_header_parameters();
        if !extra_headers.is_empty() {
            if extra_headers.contains_key(ACCEPT) {
                panic!("extra header parameters cannot contain header key ACCEPT")
            }

            for (header_name, header_value) in extra_headers.iter() {
                headers.insert(header_name, header_value.clone());
            }
        }

        let extra_query_params = self.extra_query_parameters();
        if !extra_query_params.is_empty() {
            for (key, value) in extra_query_params.iter() {
                uri.query_pairs_mut()
                    .append_pair(key.as_ref(), value.as_ref());
            }
        }

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
        let mut uri = self.uri()?;
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

        let extra_headers = self.extra_header_parameters();
        if !extra_headers.is_empty() {
            if extra_headers.contains_key(ACCEPT) {
                panic!("extra header parameters cannot contain header key ACCEPT")
            }

            for (header_name, header_value) in extra_headers.iter() {
                headers.insert(header_name, header_value.clone());
            }
        }

        let extra_query_params = self.extra_query_parameters();
        if !extra_query_params.is_empty() {
            for (key, value) in extra_query_params.iter() {
                uri.query_pairs_mut()
                    .append_pair(key.as_ref(), value.as_ref());
            }
        }

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

#[derive(Clone)]
pub struct UnInitializedCredentialExecutor;

impl TokenCredentialExecutor for UnInitializedCredentialExecutor {
    fn is_initialized(&self) -> bool {
        false
    }

    fn uri(&mut self) -> AuthorizationResult<Url> {
        panic!("TokenCredentialExecutor is UnInitialized");
    }

    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        panic!("TokenCredentialExecutor is UnInitialized");
    }

    fn app_config(&self) -> &AppConfig {
        panic!("TokenCredentialExecutor is UnInitialized");
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
            "https://login.microsoftonline.com/tenant-id/v2.0/.well-known/openid-configuration",
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
            "https://login.microsoftonline.com/common/v2.0/.well-known/openid-configuration",
            url.as_str()
        )
    }
}
