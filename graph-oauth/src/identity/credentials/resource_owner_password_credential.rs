use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::{
    Authority, AuthorizationSerializer, AzureAuthorityHost, TokenCredentialOptions,
};
use graph_error::{AuthorizationFailure, AuthorizationResult};
use std::collections::HashMap;
use url::Url;

/// Allows an application to sign in the user by directly handling their password.
/// Not recommended. ROPC can also be done using a client secret or assertion,
/// however this client implementation does not offer this use case. This is the
/// same as all MSAL clients.
/// https://datatracker.ietf.org/doc/html/rfc6749#section-1.3.3
#[derive(Clone)]
pub struct ResourceOwnerPasswordCredential {
    /// Required.
    /// The Application (client) ID that the Azure portal - App registrations page assigned
    /// to your app
    pub(crate) client_id: String,
    /// Required
    /// The user's email address.
    pub(crate) username: String,
    /// Required
    /// The user's password.
    pub(crate) password: String,
    /// The value passed for the scope parameter in this request should be the resource
    /// identifier (application ID URI) of the resource you want, affixed with the .default
    /// suffix. For the Microsoft Graph example, the value is https://graph.microsoft.com/.default.
    /// Default is https://graph.microsoft.com/.default.
    pub(crate) scope: Vec<String>,
    pub(crate) authority: Authority,
    pub(crate) token_credential_options: TokenCredentialOptions,
    serializer: OAuthSerializer,
}

impl ResourceOwnerPasswordCredential {
    pub fn new<T: AsRef<str>>(
        client_id: T,
        username: T,
        password: T,
    ) -> ResourceOwnerPasswordCredential {
        ResourceOwnerPasswordCredential {
            client_id: client_id.as_ref().to_owned(),
            username: username.as_ref().to_owned(),
            password: password.as_ref().to_owned(),
            scope: vec![],
            authority: Default::default(),
            token_credential_options: Default::default(),
            serializer: Default::default(),
        }
    }

    pub fn new_with_tenant<T: AsRef<str>>(
        tenant: T,
        client_id: T,
        username: T,
        password: T,
    ) -> ResourceOwnerPasswordCredential {
        ResourceOwnerPasswordCredential {
            client_id: client_id.as_ref().to_owned(),
            username: username.as_ref().to_owned(),
            password: password.as_ref().to_owned(),
            scope: vec![],
            authority: Authority::TenantId(tenant.as_ref().to_owned()),
            token_credential_options: Default::default(),
            serializer: Default::default(),
        }
    }
}

impl AuthorizationSerializer for ResourceOwnerPasswordCredential {
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
            return AuthorizationFailure::result(OAuthParameter::ClientId.alias());
        }

        if self.username.trim().is_empty() {
            return AuthorizationFailure::result(OAuthParameter::Username.alias());
        }

        if self.password.trim().is_empty() {
            return AuthorizationFailure::result(OAuthParameter::Password.alias());
        }

        self.serializer
            .client_id(self.client_id.as_str())
            .grant_type("password")
            .extend_scopes(self.scope.iter());

        self.serializer.as_credential_map(
            vec![OAuthParameter::Scope],
            vec![OAuthParameter::ClientId, OAuthParameter::GrantType],
        )
    }

    fn basic_auth(&self) -> Option<(String, String)> {
        Some((self.username.to_string(), self.password.to_string()))
    }
}

#[derive(Clone)]
pub struct ResourceOwnerPasswordCredentialBuilder {
    credential: ResourceOwnerPasswordCredential,
}

impl ResourceOwnerPasswordCredentialBuilder {
    pub fn new() -> ResourceOwnerPasswordCredentialBuilder {
        ResourceOwnerPasswordCredentialBuilder {
            credential: ResourceOwnerPasswordCredential {
                client_id: String::new(),
                username: String::new(),
                password: String::new(),
                scope: vec![],
                authority: Authority::Organizations,
                token_credential_options: Default::default(),
                serializer: Default::default(),
            },
        }
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> &mut Self {
        self.credential.client_id = client_id.as_ref().to_owned();
        self
    }

    pub fn with_username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
        self.credential.username = username.as_ref().to_owned();
        self
    }

    pub fn with_password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
        self.credential.password = password.as_ref().to_owned();
        self
    }

    /// The grant type isn't supported on the /common or /consumers authentication contexts.
    /// Use /organizations or a tenant ID instead.
    /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
    pub fn with_tenant<T: AsRef<str>>(&mut self, tenant: T) -> &mut Self {
        self.credential.authority = Authority::TenantId(tenant.as_ref().to_owned());
        self
    }

    /// The grant type isn't supported on the /common or /consumers authentication contexts.
    /// Use /organizations or a tenant ID instead.
    /// Authority defaults to /organizations if no tenant id or authority is given.
    pub fn with_authority<T: Into<Authority>>(
        &mut self,
        authority: T,
    ) -> AuthorizationResult<&mut Self> {
        let authority = authority.into();
        if authority.eq(&Authority::Common)
            || authority.eq(&Authority::AzureActiveDirectory)
            || authority.eq(&Authority::Consumers)
        {
            return AuthorizationFailure::msg_result(
                "tenant_id",
                "Authority Azure Active Directory, common, and consumers are not supported authentication contexts for ROPC"
            );
        }

        self.credential.authority = authority;
        Ok(self)
    }

    /// Defaults to "https://graph.microsoft.com/.default"
    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scope: I) -> &mut Self {
        self.credential.scope = scope.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_token_credential_options(
        &mut self,
        token_credential_options: TokenCredentialOptions,
    ) {
        self.credential.token_credential_options = token_credential_options;
    }

    pub fn build(&self) -> ResourceOwnerPasswordCredential {
        self.credential.clone()
    }
}

impl Default for ResourceOwnerPasswordCredentialBuilder {
    fn default() -> Self {
        ResourceOwnerPasswordCredentialBuilder::new()
    }
}
