use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{Authority, AzureCloudInstance, TokenCredentialExecutor};
use async_trait::async_trait;
use graph_error::{AuthorizationResult, AF};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

/// Allows an application to sign in the user by directly handling their password.
/// Not recommended. ROPC can also be done using a client secret or assertion,
/// however this client implementation does not offer this use case. This is the
/// same as all MSAL clients.
/// https://datatracker.ietf.org/doc/html/rfc6749#section-1.3.3
#[derive(Clone)]
pub struct ResourceOwnerPasswordCredential {
    pub(crate) app_config: AppConfig,
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
    serializer: OAuthSerializer,
}

impl ResourceOwnerPasswordCredential {
    pub fn new<T: AsRef<str>>(
        client_id: T,
        username: T,
        password: T,
    ) -> ResourceOwnerPasswordCredential {
        let mut app_config = AppConfig::new_with_client_id(client_id.as_ref());
        app_config.authority = Authority::Organizations;
        ResourceOwnerPasswordCredential {
            app_config,
            username: username.as_ref().to_owned(),
            password: password.as_ref().to_owned(),
            scope: vec![],
            serializer: Default::default(),
        }
    }

    pub fn new_with_tenant<T: AsRef<str>>(
        tenant_id: T,
        client_id: T,
        username: T,
        password: T,
    ) -> ResourceOwnerPasswordCredential {
        ResourceOwnerPasswordCredential {
            app_config: AppConfig::new_with_tenant_and_client_id(tenant_id.as_ref(), client_id),
            username: username.as_ref().to_owned(),
            password: password.as_ref().to_owned(),
            scope: vec![],
            serializer: Default::default(),
        }
    }

    pub fn builder<T: AsRef<str>>(client_id: T) -> ResourceOwnerPasswordCredentialBuilder {
        ResourceOwnerPasswordCredentialBuilder::new(client_id)
    }
}

#[async_trait]
impl TokenCredentialExecutor for ResourceOwnerPasswordCredential {
    fn uri(&mut self, azure_cloud_instance: &AzureCloudInstance) -> AuthorizationResult<Url> {
        self.serializer
            .authority(azure_cloud_instance, &self.app_config.authority);

        let uri = self
            .serializer
            .get(OAuthParameter::TokenUrl)
            .ok_or(AF::msg_err("access_token_url", "Internal Error"))?;
        Url::parse(uri.as_str()).map_err(AF::from)
    }

    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        let client_id = self.app_config.client_id.to_string();
        if client_id.is_empty() || self.app_config.client_id.is_nil() {
            return AF::result(OAuthParameter::ClientId.alias());
        }

        if self.username.trim().is_empty() {
            return AF::result(OAuthParameter::Username.alias());
        }

        if self.password.trim().is_empty() {
            return AF::result(OAuthParameter::Password.alias());
        }

        self.serializer
            .client_id(client_id.as_str())
            .grant_type("password")
            .extend_scopes(self.scope.iter());

        self.serializer.as_credential_map(
            vec![OAuthParameter::Scope],
            vec![OAuthParameter::ClientId, OAuthParameter::GrantType],
        )
    }

    fn client_id(&self) -> &Uuid {
        &self.app_config.client_id
    }

    fn authority(&self) -> Authority {
        self.app_config.authority.clone()
    }

    fn azure_cloud_instance(&self) -> AzureCloudInstance {
        self.app_config.azure_cloud_instance
    }

    fn app_config(&self) -> &AppConfig {
        todo!()
    }
}

#[derive(Clone)]
pub struct ResourceOwnerPasswordCredentialBuilder {
    credential: ResourceOwnerPasswordCredential,
}

impl ResourceOwnerPasswordCredentialBuilder {
    fn new<T: AsRef<str>>(client_id: T) -> ResourceOwnerPasswordCredentialBuilder {
        ResourceOwnerPasswordCredentialBuilder {
            credential: ResourceOwnerPasswordCredential {
                app_config: AppConfig::new_with_client_id(client_id.as_ref()),
                username: String::new(),
                password: String::new(),
                scope: vec![],
                serializer: Default::default(),
            },
        }
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> &mut Self {
        self.credential.app_config.client_id =
            Uuid::try_parse(client_id.as_ref()).unwrap_or_default();
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
        self.credential.app_config.authority = Authority::TenantId(tenant.as_ref().to_owned());
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
        if vec![
            Authority::Common,
            Authority::Consumers,
            Authority::AzureActiveDirectory,
        ]
        .contains(&authority)
        {
            return AF::msg_result(
                "tenant_id",
                "AzureActiveDirectory, Common, and Consumers are not supported authentication contexts for ROPC"
            );
        }

        self.credential.app_config.authority = authority;
        Ok(self)
    }

    /// Defaults to "https://graph.microsoft.com/.default"
    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scope: I) -> &mut Self {
        self.credential.scope = scope.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn build(&self) -> ResourceOwnerPasswordCredential {
        self.credential.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_on_authority_common() {
        let _ = ResourceOwnerPasswordCredential::builder(Uuid::new_v4().to_string())
            .with_authority(Authority::Common)
            .unwrap()
            .build();
    }

    #[test]
    #[should_panic]
    fn fail_on_authority_adfs() {
        let _ = ResourceOwnerPasswordCredential::builder(Uuid::new_v4().to_string())
            .with_authority(Authority::AzureActiveDirectory)
            .unwrap()
            .build();
    }

    #[test]
    #[should_panic]
    fn fail_on_authority_consumers() {
        let _ = ResourceOwnerPasswordCredential::builder(Uuid::new_v4().to_string())
            .with_authority(Authority::Consumers)
            .unwrap()
            .build();
    }
}
