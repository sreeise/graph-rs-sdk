use reqwest::IntoUrl;
use url::form_urlencoded::Serializer;
use url::Url;
use uuid::Uuid;

use graph_error::{AuthorizationFailure, IdentityResult};

use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::{credentials::app_config::AppConfig, Authority, AzureCloudInstance};

#[derive(Clone)]
pub struct ClientCredentialsAuthorizationUrlParameters {
    /// The client (application) ID of the service principal
    pub(crate) app_config: AppConfig,
    pub(crate) state: Option<String>,
}

impl ClientCredentialsAuthorizationUrlParameters {
    pub fn new<T: AsRef<str>, U: IntoUrl>(
        client_id: T,
        redirect_uri: U,
    ) -> IdentityResult<ClientCredentialsAuthorizationUrlParameters> {
        let redirect_uri_result = Url::parse(redirect_uri.as_str());
        let redirect_uri = redirect_uri.into_url().or(redirect_uri_result)?;

        Ok(ClientCredentialsAuthorizationUrlParameters {
            app_config: AppConfig::new_init(
                Uuid::try_parse(client_id.as_ref()).unwrap_or_default(),
                Option::<String>::None,
                Some(redirect_uri),
            ),
            state: None,
        })
    }

    pub fn builder<T: AsRef<str>>(
        client_id: T,
    ) -> ClientCredentialsAuthorizationUrlParameterBuilder {
        ClientCredentialsAuthorizationUrlParameterBuilder::new(client_id)
    }

    pub fn url(&self) -> IdentityResult<Url> {
        self.url_with_host(&self.app_config.azure_cloud_instance)
    }

    pub fn url_with_host(&self, azure_cloud_instance: &AzureCloudInstance) -> IdentityResult<Url> {
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

#[derive(Clone)]
pub struct ClientCredentialsAuthorizationUrlParameterBuilder {
    parameters: ClientCredentialsAuthorizationUrlParameters,
}

impl ClientCredentialsAuthorizationUrlParameterBuilder {
    pub fn new<T: AsRef<str>>(client_id: T) -> Self {
        Self {
            parameters: ClientCredentialsAuthorizationUrlParameters {
                app_config: AppConfig::new_with_client_id(client_id),
                state: None,
            },
        }
    }

    pub fn new_with_app_config(app_config: AppConfig) -> Self {
        Self {
            parameters: ClientCredentialsAuthorizationUrlParameters {
                app_config,
                state: None,
            },
        }
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> IdentityResult<&mut Self> {
        self.parameters.app_config.client_id = Uuid::try_parse(client_id.as_ref())?;
        Ok(self)
    }

    pub fn with_redirect_uri<T: IntoUrl>(&mut self, redirect_uri: T) -> IdentityResult<&mut Self> {
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

    pub fn build(&self) -> ClientCredentialsAuthorizationUrlParameters {
        self.parameters.clone()
    }

    pub fn url(&self) -> IdentityResult<Url> {
        self.parameters.url()
    }
}
