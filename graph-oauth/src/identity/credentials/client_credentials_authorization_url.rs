use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::{Authority, AzureAuthorityHost};
use graph_error::{AuthorizationFailure, AuthorizationResult};
use url::form_urlencoded::Serializer;
use url::Url;

#[derive(Clone)]
pub struct ClientCredentialsAuthorizationUrl {
    /// The client (application) ID of the service principal
    pub(crate) client_id: String,
    pub(crate) redirect_uri: String,
    pub(crate) state: Option<String>,
    pub(crate) authority: Authority,
}

impl ClientCredentialsAuthorizationUrl {
    pub fn new<T: AsRef<str>>(client_id: T, redirect_uri: T) -> ClientCredentialsAuthorizationUrl {
        ClientCredentialsAuthorizationUrl {
            client_id: client_id.as_ref().to_owned(),
            redirect_uri: redirect_uri.as_ref().to_owned(),
            state: None,
            authority: Default::default(),
        }
    }

    pub fn builder() -> ClientCredentialsAuthorizationUrlBuilder {
        ClientCredentialsAuthorizationUrlBuilder::new()
    }

    pub fn url(&self) -> AuthorizationResult<Url> {
        self.url_with_host(&AzureAuthorityHost::AzurePublic)
    }

    pub fn url_with_host(
        &self,
        azure_authority_host: &AzureAuthorityHost,
    ) -> AuthorizationResult<Url> {
        let mut serializer = OAuthSerializer::new();
        if self.client_id.trim().is_empty() {
            return AuthorizationFailure::required_value_result(OAuthParameter::ClientId.alias());
        }

        if self.redirect_uri.trim().is_empty() {
            return AuthorizationFailure::required_value_result(
                OAuthParameter::RedirectUri.alias(),
            );
        }

        serializer
            .client_id(self.client_id.as_str())
            .redirect_uri(self.redirect_uri.as_str());

        if let Some(state) = self.state.as_ref() {
            serializer.state(state.as_ref());
        }

        serializer.authority_admin_consent(azure_authority_host, &self.authority);

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
                .ok_or(AuthorizationFailure::required_value(
                    OAuthParameter::AuthorizationUrl.alias(),
                ))?
                .as_str(),
        )
        .or(AuthorizationFailure::required_value_result(
            OAuthParameter::AuthorizationUrl.alias(),
        ))?;
        url.set_query(Some(encoder.finish().as_str()));
        Ok(url)
    }
}

pub struct ClientCredentialsAuthorizationUrlBuilder {
    client_credentials_authorization_url: ClientCredentialsAuthorizationUrl,
}

impl ClientCredentialsAuthorizationUrlBuilder {
    pub fn new() -> Self {
        Self {
            client_credentials_authorization_url: ClientCredentialsAuthorizationUrl {
                client_id: String::new(),
                redirect_uri: String::new(),
                state: None,
                authority: Default::default(),
            },
        }
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> &mut Self {
        self.client_credentials_authorization_url.client_id = client_id.as_ref().to_owned();
        self
    }

    pub fn with_redirect_uri<T: AsRef<str>>(&mut self, redirect_uri: T) -> &mut Self {
        self.client_credentials_authorization_url.redirect_uri = redirect_uri.as_ref().to_owned();
        self
    }

    /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
    pub fn with_tenant<T: AsRef<str>>(&mut self, tenant: T) -> &mut Self {
        self.client_credentials_authorization_url.authority =
            Authority::TenantId(tenant.as_ref().to_owned());
        self
    }

    pub fn with_authority<T: Into<Authority>>(&mut self, authority: T) -> &mut Self {
        self.client_credentials_authorization_url.authority = authority.into();
        self
    }

    pub fn with_state<T: AsRef<str>>(&mut self, state: T) -> &mut Self {
        self.client_credentials_authorization_url.state = Some(state.as_ref().to_owned());
        self
    }

    pub fn build(&self) -> ClientCredentialsAuthorizationUrl {
        self.client_credentials_authorization_url.clone()
    }

    pub fn url(&self) -> AuthorizationResult<Url> {
        self.client_credentials_authorization_url.url()
    }
}

impl Default for ClientCredentialsAuthorizationUrlBuilder {
    fn default() -> Self {
        ClientCredentialsAuthorizationUrlBuilder::new()
    }
}
