use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{
    tracing_targets::CREDENTIAL_EXECUTOR, Authority, AuthorizationRequestParts, AzureCloudInstance,
};
use async_trait::async_trait;
use dyn_clone::DynClone;
use graph_error::AuthExecutionError;
use graph_error::{AuthExecutionResult, IdentityResult};
use graph_http::api_impl::{GraphClientConfiguration, MinimalAsyncClient, MinimalBlockingClient};
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::fmt::Debug;
use tower::{Service, ServiceExt};
use url::{ParseError, Url};
use uuid::Uuid;

dyn_clone::clone_trait_object!(TokenCredentialExecutor);

#[async_trait]
pub trait TokenCredentialExecutor: DynClone + Debug {
    fn uri(&mut self) -> IdentityResult<Url> {
        Ok(self.azure_cloud_instance().token_uri(&self.authority())?)
    }

    fn form_urlencode(&mut self) -> IdentityResult<HashMap<String, String>>;

    fn request_parts(&mut self) -> IdentityResult<AuthorizationRequestParts> {
        let uri = self.uri()?;
        let form = self.form_urlencode()?;
        let basic_auth = self.basic_auth();
        let extra_headers = self.extra_header_parameters();
        let extra_query_params = self.extra_query_parameters();

        let mut auth_request = AuthorizationRequestParts::new(uri, form, basic_auth);
        auth_request.with_extra_headers(extra_headers);
        auth_request.with_extra_query_parameters(extra_query_params);

        Ok(auth_request)
    }

    fn build_request(
        &mut self,
    ) -> AuthExecutionResult<(reqwest::blocking::RequestBuilder, MinimalBlockingClient)> {
        let http_client = MinimalBlockingClient::from(self.config().clone());
        let auth_request = self.request_parts()?;
        let basic_auth = auth_request.basic_auth;

        if let Some((client_identifier, secret)) = basic_auth {
            let request_builder = http_client
                .inner
                .post(auth_request.uri)
                .basic_auth(client_identifier, Some(secret))
                .headers(auth_request.headers)
                .form(&auth_request.form_urlencoded);

            tracing::debug!(
                 target: CREDENTIAL_EXECUTOR,
                "authorization request constructed"
            );
            Ok((request_builder, http_client))
        } else {
            let request_builder = http_client
                .inner
                .post(auth_request.uri)
                .headers(auth_request.headers)
                .form(&auth_request.form_urlencoded);

            tracing::debug!(
                 target: CREDENTIAL_EXECUTOR,
                "authorization request constructed"
            );
            Ok((request_builder, http_client))
        }
    }

    async fn build_request_async(
        &mut self,
    ) -> AuthExecutionResult<(reqwest::RequestBuilder, MinimalAsyncClient)> {
        let http_client = MinimalAsyncClient::from(self.config().clone());
        let auth_request = self.request_parts()?;
        let basic_auth = auth_request.basic_auth;

        if let Some((client_identifier, secret)) = basic_auth {
            let request_builder = http_client
                .inner
                .post(auth_request.uri)
                .basic_auth(client_identifier, Some(secret))
                .headers(auth_request.headers)
                .form(&auth_request.form_urlencoded);

            tracing::debug!(
                target: CREDENTIAL_EXECUTOR,
                "authorization request constructed"
            );
            Ok((request_builder, http_client))
        } else {
            let request_builder = http_client
                .inner
                .post(auth_request.uri)
                .headers(auth_request.headers)
                .form(&auth_request.form_urlencoded);

            tracing::debug!(
                target: CREDENTIAL_EXECUTOR,
                "authorization request constructed"
            );
            Ok((request_builder, http_client))
        }
    }

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

    fn issuer(&self) -> Result<Url, ParseError> {
        self.azure_cloud_instance().issuer(&self.authority())
    }

    fn extra_query_parameters(&self) -> &HashMap<String, String> {
        &self.app_config().extra_query_parameters
    }

    fn config(&self) -> &GraphClientConfiguration {
        &self.app_config().config
    }

    fn execute(&mut self) -> AuthExecutionResult<reqwest::blocking::Response> {
        let (request_builder, minimal_blocking_client) = self.build_request()?;
        let request = request_builder.build()?;
        let response = minimal_blocking_client.inner.execute(request)?;
        let status = response.status();
        tracing::debug!(target: CREDENTIAL_EXECUTOR, "authorization response received; status={status:#?}");
        Ok(response)
    }

    async fn execute_async(&mut self) -> AuthExecutionResult<reqwest::Response> {
        let (request_builder, mut minimal_async_client) = self.build_request_async().await?;
        let request = request_builder.build()?;
        let response = minimal_async_client
            .service
            .ready()
            .await
            .map_err(AuthExecutionError::from)?
            .call(request)
            .await
            .map_err(AuthExecutionError::from)?;
        let status = response.status();
        tracing::debug!(target: CREDENTIAL_EXECUTOR, "authorization response received; status={status:#?}");
        Ok(response)
    }
}
