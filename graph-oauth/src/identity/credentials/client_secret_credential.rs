use crate::auth::{OAuth, OAuthCredential};
use crate::identity::form_credential::FormCredential;
use crate::identity::{Authority, AuthorizationSerializer, AzureAuthorityHost, TokenRequest};
use crate::oauth::TokenCredentialOptions;
use graph_error::{AuthorizationFailure, AuthorizationResult};
use std::collections::HashMap;
use url::Url;

/// Client Credentials flow using a client secret.
///
/// The OAuth 2.0 client credentials grant flow permits a web service (confidential client)
/// to use its own credentials, instead of impersonating a user, to authenticate when calling
/// another web service. The grant specified in RFC 6749, sometimes called two-legged OAuth,
/// can be used to access web-hosted resources by using the identity of an application.
/// This type is commonly used for server-to-server interactions that must run in the background,
/// without immediate interaction with a user, and is often referred to as daemons or service accounts.
///
/// See [Microsoft identity platform and the OAuth 2.0 client credentials flow](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow)
#[derive(Clone)]
pub struct ClientSecretCredential {
    /// The client (application) ID of the service principal
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    /// The value passed for the scope parameter in this request should be the resource
    /// identifier (application ID URI) of the resource you want, affixed with the .default
    /// suffix. For the Microsoft Graph example, the value is https://graph.microsoft.com/.default.
    /// Default is https://graph.microsoft.com/.default.
    pub(crate) scopes: Vec<String>,
    pub(crate) authority: Authority,
    pub(crate) token_credential_options: TokenCredentialOptions,
    serializer: OAuth,
}

impl ClientSecretCredential {
    pub fn new<T: AsRef<str>>(client_id: T, client_secret: T) -> ClientSecretCredential {
        ClientSecretCredential {
            client_id: client_id.as_ref().to_owned(),
            client_secret: client_secret.as_ref().to_owned(),
            scopes: vec!["https://graph.microsoft.com/.default".to_owned()],
            authority: Default::default(),
            token_credential_options: Default::default(),
            serializer: OAuth::new(),
        }
    }

    pub fn builder() -> ClientSecretCredentialBuilder {
        ClientSecretCredentialBuilder::new()
    }
}

impl TokenRequest for ClientSecretCredential {
    fn azure_authority_host(&self) -> &AzureAuthorityHost {
        &self.token_credential_options.azure_authority_host
    }
}

impl AuthorizationSerializer for ClientSecretCredential {
    fn uri(&mut self, azure_authority_host: &AzureAuthorityHost) -> AuthorizationResult<Url> {
        self.serializer
            .authority(azure_authority_host, &self.authority);

        let uri = self.serializer.get(OAuthCredential::AccessTokenUrl).ok_or(
            AuthorizationFailure::required_value_msg("access_token_url", Some("Internal Error")),
        )?;
        Url::parse(uri.as_str()).map_err(AuthorizationFailure::from)
    }

    fn form(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        if self.client_id.trim().is_empty() {
            return AuthorizationFailure::required_value_result(OAuthCredential::ClientId);
        }

        if self.client_secret.trim().is_empty() {
            return AuthorizationFailure::required_value_result(OAuthCredential::ClientSecret);
        }

        self.serializer
            .client_id(self.client_id.as_str())
            .client_secret(self.client_secret.as_str())
            .grant_type("client_credentials");

        if self.scopes.is_empty() {
            self.serializer
                .extend_scopes(vec!["https://graph.microsoft.com/.default".to_owned()]);
        } else {
            self.serializer.extend_scopes(&self.scopes);
        }

        self.serializer.authorization_form(vec![
            FormCredential::Required(OAuthCredential::ClientId),
            FormCredential::Required(OAuthCredential::ClientSecret),
            FormCredential::Required(OAuthCredential::GrantType),
            FormCredential::NotRequired(OAuthCredential::Scope),
        ])
    }
}

pub struct ClientSecretCredentialBuilder {
    credential: ClientSecretCredential,
}

impl ClientSecretCredentialBuilder {
    fn new() -> Self {
        Self {
            credential: ClientSecretCredential {
                client_id: String::new(),
                client_secret: String::new(),
                scopes: vec![],
                authority: Default::default(),
                token_credential_options: Default::default(),
                serializer: OAuth::new(),
            },
        }
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> &mut Self {
        self.credential.client_id = client_id.as_ref().to_owned();
        self
    }

    pub fn with_client_secret<T: AsRef<str>>(&mut self, client_secret: T) -> &mut Self {
        self.credential.client_secret = client_secret.as_ref().to_owned();
        self
    }

    /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
    pub fn with_tenant<T: AsRef<str>>(&mut self, tenant: T) -> &mut Self {
        self.credential.authority = Authority::TenantId(tenant.as_ref().to_owned());
        self
    }

    pub fn with_authority<T: Into<Authority>>(&mut self, authority: T) -> &mut Self {
        self.credential.authority = authority.into();
        self
    }

    /// Defaults to "https://graph.microsoft.com/.default"
    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scopes: I) -> &mut Self {
        self.credential.scopes = scopes.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_token_credential_options(
        &mut self,
        token_credential_options: TokenCredentialOptions,
    ) {
        self.credential.token_credential_options = token_credential_options;
    }
}

impl Default for ClientSecretCredentialBuilder {
    fn default() -> Self {
        Self::new()
    }
}
