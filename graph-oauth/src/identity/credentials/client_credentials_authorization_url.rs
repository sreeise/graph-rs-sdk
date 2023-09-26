use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{Authority, AzureCloudInstance};
use graph_error::{AuthorizationFailure, AuthorizationResult};
use reqwest::IntoUrl;
use url::form_urlencoded::Serializer;
use url::Url;
use uuid::Uuid;

#[derive(Clone)]
pub struct ClientCredentialsAuthorizationUrl {
    /// The client (application) ID of the service principal
    pub(crate) app_config: AppConfig,
    pub(crate) state: Option<String>,
}

impl ClientCredentialsAuthorizationUrl {
    pub fn new<T: AsRef<str>, U: IntoUrl>(
        client_id: T,
        redirect_uri: U,
    ) -> AuthorizationResult<ClientCredentialsAuthorizationUrl> {
        let redirect_uri_result = Url::parse(redirect_uri.as_str());
        let redirect_uri = redirect_uri.into_url().or(redirect_uri_result)?;

        Ok(ClientCredentialsAuthorizationUrl {
            app_config: AppConfig {
                tenant_id: None,
                client_id: Uuid::try_parse(client_id.as_ref())?,
                authority: Default::default(),
                azure_cloud_instance: Default::default(),
                extra_query_parameters: Default::default(),
                extra_header_parameters: Default::default(),
                redirect_uri: Some(redirect_uri),
            },
            state: None,
        })
    }

    pub fn builder<T: AsRef<str>>(client_id: T) -> ClientCredentialsAuthorizationUrlBuilder {
        ClientCredentialsAuthorizationUrlBuilder::new(client_id)
    }

    pub fn url(&self) -> AuthorizationResult<Url> {
        self.url_with_host(&self.app_config.azure_cloud_instance)
    }

    pub fn url_with_host(
        &self,
        azure_cloud_instance: &AzureCloudInstance,
    ) -> AuthorizationResult<Url> {
        let mut serializer = OAuthSerializer::new();
        let client_id = self.app_config.client_id.to_string();
        if client_id.trim().is_empty() || self.app_config.client_id.is_nil() {
            return AuthorizationFailure::result(OAuthParameter::ClientId.alias());
        }

        if self.app_config.redirect_uri.is_none() {
            return AuthorizationFailure::result(OAuthParameter::RedirectUri.alias());
        }

        if let Some(redirect_uri) = self.app_config.redirect_uri.as_ref() {
            serializer.redirect_uri(redirect_uri.as_str());
        }

        serializer.client_id(client_id.as_str());

        if let Some(state) = self.state.as_ref() {
            serializer.state(state.as_ref());
        }

        serializer.authority_admin_consent(azure_cloud_instance, &self.app_config.authority);

        let mut encoder = Serializer::new(String::new());
        serializer.form_encode_credentials(
            vec![
                OAuthParameter::ClientId,
                OAuthParameter::RedirectUri,
                OAuthParameter::State,
            ],
            &mut encoder,
        );

        let mut url = Url::parse(
            serializer
                .get(OAuthParameter::AuthorizationUrl)
                .ok_or(AuthorizationFailure::required(
                    OAuthParameter::AuthorizationUrl.alias(),
                ))?
                .as_str(),
        )
        .or(AuthorizationFailure::result(
            OAuthParameter::AuthorizationUrl.alias(),
        ))?;
        url.set_query(Some(encoder.finish().as_str()));
        Ok(url)
    }
}

pub struct ClientCredentialsAuthorizationUrlBuilder {
    parameters: ClientCredentialsAuthorizationUrl,
}

impl ClientCredentialsAuthorizationUrlBuilder {
    pub fn new<T: AsRef<str>>(client_id: T) -> Self {
        Self {
            parameters: ClientCredentialsAuthorizationUrl {
                app_config: AppConfig::new_with_client_id(client_id),
                state: None,
            },
        }
    }

    pub fn new_with_app_config(app_config: AppConfig) -> Self {
        Self {
            parameters: ClientCredentialsAuthorizationUrl {
                app_config,
                state: None,
            },
        }
    }

    pub fn with_client_id<T: AsRef<str>>(
        &mut self,
        client_id: T,
    ) -> AuthorizationResult<&mut Self> {
        self.parameters.app_config.client_id = Uuid::try_parse(client_id.as_ref())?;
        Ok(self)
    }

    pub fn with_redirect_uri<T: IntoUrl>(
        &mut self,
        redirect_uri: T,
    ) -> AuthorizationResult<&mut Self> {
        let redirect_uri_result = Url::parse(redirect_uri.as_str());
        let redirect_uri = redirect_uri.into_url().or(redirect_uri_result)?;
        self.parameters.app_config.redirect_uri = Some(redirect_uri);
        Ok(self)
    }

    /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
    pub fn with_tenant<T: AsRef<str>>(&mut self, tenant: T) -> &mut Self {
        self.parameters.app_config.authority = Authority::TenantId(tenant.as_ref().to_owned());
        self
    }

    pub fn with_authority<T: Into<Authority>>(&mut self, authority: T) -> &mut Self {
        self.parameters.app_config.authority = authority.into();
        self
    }

    pub fn with_state<T: AsRef<str>>(&mut self, state: T) -> &mut Self {
        self.parameters.state = Some(state.as_ref().to_owned());
        self
    }

    pub fn build(&self) -> ClientCredentialsAuthorizationUrl {
        self.parameters.clone()
    }

    pub fn url(&self) -> AuthorizationResult<Url> {
        self.parameters.url()
    }
}
