use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::{
    Authority, AuthorizationSerializer, AzureAuthorityHost,
    ClientCredentialsAuthorizationUrlBuilder, CredentialBuilder, TokenRequest,
};
use crate::oauth::TokenCredentialOptions;
use graph_error::{AuthorizationFailure, AuthorizationResult};
use std::collections::HashMap;
use url::Url;

credential_builder_impl!(ClientSecretCredentialBuilder, ClientSecretCredential);

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
#[derive(Clone, Debug)]
pub struct ClientSecretCredential {
    /// Required.
    /// The Application (client) ID that the Azure portal - App registrations page assigned
    /// to your app
    pub(crate) client_id: String,
    /// Required
    /// The application secret that you created in the app registration portal for your app.
    /// Don't use the application secret in a native app or single page app because a
    /// client_secret can't be reliably stored on devices or web pages. It's required for web
    /// apps and web APIs, which can store the client_secret securely on the server side. Like
    /// all parameters here, the client secret must be URL-encoded before being sent. This step
    /// is done by the SDK. For more information on URI encoding, see the URI Generic Syntax
    /// specification. The Basic auth pattern of instead providing credentials in the Authorization
    /// header, per RFC 6749 is also supported.
    pub(crate) client_secret: String,
    /// The value passed for the scope parameter in this request should be the resource
    /// identifier (application ID URI) of the resource you want, affixed with the .default
    /// suffix. For the Microsoft Graph example, the value is https://graph.microsoft.com/.default.
    /// Default is https://graph.microsoft.com/.default.
    pub(crate) scope: Vec<String>,
    pub(crate) authority: Authority,
    pub(crate) token_credential_options: TokenCredentialOptions,
    serializer: OAuthSerializer,
}

impl ClientSecretCredential {
    pub fn new<T: AsRef<str>>(client_id: T, client_secret: T) -> ClientSecretCredential {
        ClientSecretCredential {
            client_id: client_id.as_ref().to_owned(),
            client_secret: client_secret.as_ref().to_owned(),
            scope: vec!["https://graph.microsoft.com/.default".into()],
            authority: Default::default(),
            token_credential_options: Default::default(),
            serializer: OAuthSerializer::new(),
        }
    }

    pub fn new_with_tenant<T: AsRef<str>>(
        tenant_id: T,
        client_id: T,
        client_secret: T,
    ) -> ClientSecretCredential {
        ClientSecretCredential {
            client_id: client_id.as_ref().to_owned(),
            client_secret: client_secret.as_ref().to_owned(),
            scope: vec!["https://graph.microsoft.com/.default".into()],
            authority: Authority::TenantId(tenant_id.as_ref().to_owned()),
            token_credential_options: Default::default(),
            serializer: OAuthSerializer::new(),
        }
    }

    pub fn builder() -> ClientSecretCredentialBuilder {
        ClientSecretCredentialBuilder::new()
    }

    pub fn authorization_url_builder() -> ClientCredentialsAuthorizationUrlBuilder {
        ClientCredentialsAuthorizationUrlBuilder::new()
    }
}

impl TokenRequest for ClientSecretCredential {
    fn token_credential_options(&self) -> &TokenCredentialOptions {
        &self.token_credential_options
    }
}

impl AuthorizationSerializer for ClientSecretCredential {
    fn uri(&mut self, azure_authority_host: &AzureAuthorityHost) -> AuthorizationResult<Url> {
        self.serializer
            .authority(azure_authority_host, &self.authority);

        let uri = self.serializer.get(OAuthParameter::AccessTokenUrl).ok_or(
            AuthorizationFailure::msg_err("access_token_url", "Internal Error"),
        )?;
        Url::parse(uri.as_str()).map_err(AuthorizationFailure::from)
    }

    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        if self.client_id.trim().is_empty() {
            return AuthorizationFailure::result(OAuthParameter::ClientId);
        }

        if self.client_secret.trim().is_empty() {
            return AuthorizationFailure::result(OAuthParameter::ClientSecret);
        }

        self.serializer.grant_type("client_credentials");

        if self.scope.is_empty() {
            self.serializer
                .extend_scopes(vec!["https://graph.microsoft.com/.default".to_owned()]);
        } else {
            self.serializer.extend_scopes(&self.scope);
        }

        self.serializer
            .as_credential_map(vec![OAuthParameter::Scope], vec![OAuthParameter::GrantType])
    }

    ///
    fn basic_auth(&self) -> Option<(String, String)> {
        Some((self.client_id.clone(), self.client_secret.clone()))
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
                scope: vec![],
                authority: Default::default(),
                token_credential_options: Default::default(),
                serializer: Default::default(),
            },
        }
    }

    pub fn with_client_secret<T: AsRef<str>>(&mut self, client_secret: T) -> &mut Self {
        self.credential.client_secret = client_secret.as_ref().to_owned();
        self
    }
}

impl Default for ClientSecretCredentialBuilder {
    fn default() -> Self {
        Self::new()
    }
}
