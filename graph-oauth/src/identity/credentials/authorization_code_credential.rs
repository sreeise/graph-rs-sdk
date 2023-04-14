use crate::grants::GrantType;
use crate::identity::Authority;

/// Creates an instance of the ClientSecretCredential with the details needed to authenticate
/// against Azure Active Directory with a prefetched authorization code.
///
/// <param name="clientSecret">A client secret that was generated for the App Registration used to authenticate the client.</param>
/// <param name="authorizationCode">The authorization code obtained from a call to authorize. The code should be obtained with all required scopes.
/// See https://docs.microsoft.com/azure/active-directory/develop/v2-oauth2-auth-code-flow for more information.</param>

#[derive(Clone)]
pub struct AuthorizationCodeCredential {
    /// The authorization code obtained from a call to authorize. The code should be obtained with all required scopes.
    pub(crate) authorization_code: String,
    /// The client (application) ID of the service principal
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) redirect_uri: String,
    pub(crate) scopes: Vec<String>,
    /// The Azure Active Directory tenant (directory) Id of the service principal.
    pub(crate) tenant_id: Authority,
    pub(crate) code_verifier: Option<String>,
}

impl AuthorizationCodeCredential {
    pub fn new(
        client_id: &str,
        client_secret: &str,
        authorization_code: &str,
        redirect_uri: &str,
    ) -> AuthorizationCodeCredential {
        AuthorizationCodeCredential {
            authorization_code: authorization_code.to_owned(),
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
            redirect_uri: redirect_uri.to_owned(),
            scopes: vec![],
            tenant_id: Default::default(),
            code_verifier: None,
        }
    }

    pub fn grant_type(&self) -> GrantType {
        GrantType::AuthorizationCode
    }

    pub fn builder(authorization_code: &str) -> AuthorizationCodeCredentialBuilder {
        let credential_builder = AuthorizationCodeCredentialBuilder::create(authorization_code);
        credential_builder
    }
}

pub struct AuthorizationCodeCredentialBuilder {
    authorization_code_credential: AuthorizationCodeCredential,
}

impl AuthorizationCodeCredentialBuilder {
    pub fn create(authorization_code: &str) -> AuthorizationCodeCredentialBuilder {
        Self {
            authorization_code_credential: AuthorizationCodeCredential {
                authorization_code: authorization_code.to_owned(),
                client_id: Default::default(),
                client_secret: Default::default(),
                redirect_uri: Default::default(),
                scopes: vec![],
                tenant_id: Default::default(),
                code_verifier: None,
            },
        }
    }

    pub fn with_redirect_uri(
        &mut self,
        redirect_uri: &str,
    ) -> &mut AuthorizationCodeCredentialBuilder {
        self.authorization_code_credential.redirect_uri = redirect_uri.to_owned();
        self
    }

    pub fn with_client_id(&mut self, client_id: &str) -> &mut AuthorizationCodeCredentialBuilder {
        self.authorization_code_credential.client_id = client_id.to_owned();
        self
    }

    pub fn with_client_secret(
        &mut self,
        client_secret: &str,
    ) -> &mut AuthorizationCodeCredentialBuilder {
        self.authorization_code_credential.client_secret = client_secret.to_owned();
        self
    }

    pub fn with_tenant_id<T: Into<Authority>>(
        &mut self,
        tenant_id: T,
    ) -> &mut AuthorizationCodeCredentialBuilder {
        self.authorization_code_credential.tenant_id = tenant_id.into();
        self
    }

    pub fn with_code_verifier(
        &mut self,
        code_verifier: &str,
    ) -> &mut AuthorizationCodeCredentialBuilder {
        self.authorization_code_credential.code_verifier = Some(code_verifier.to_owned());
        self
    }

    pub fn with_scopes<T: ToString, I: IntoIterator<Item = T>>(
        &mut self,
        scopes: I,
    ) -> &mut AuthorizationCodeCredentialBuilder {
        self.authorization_code_credential.scopes =
            scopes.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn build(&mut self) -> AuthorizationCodeCredential {
        self.authorization_code_credential.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use url::Url;

    #[test]
    fn with_tenant_id_common() {
        let credential = AuthorizationCodeCredential::builder("")
            .with_tenant_id(Authority::TenantId("common".into()))
            .build();

        assert_eq!(credential.tenant_id, Authority::TenantId("common".into()))
    }

    #[test]
    fn with_tenant_id_adfs() {
        let credential = AuthorizationCodeCredential::builder("")
            .with_tenant_id(Authority::AzureDirectoryFederatedServices)
            .build();

        assert_eq!(credential.tenant_id.as_ref(), "adfs");
    }
}
