use crate::auth::{OAuth, OAuthCredential};
use crate::identity::credentials::{AuthorizationCodeCredential, TokenCredentialOptions};
use crate::identity::TokenRequest;
use crate::oauth::OAuthError;
use async_trait::async_trait;
use graph_error::{GraphFailure, GraphResult};
use reqwest::Response;

#[derive(Default)]
pub struct ConfidentialClient {
    http_client: OAuth,
    token_credential_options: TokenCredentialOptions,
}

impl ConfidentialClient {
    pub fn new<T>(credential: T, options: TokenCredentialOptions) -> GraphResult<ConfidentialClient>
    where
        T: TryInto<ConfidentialClient, Error = GraphFailure>,
    {
        let mut cred = credential.try_into()?;
        cred.token_credential_options = options;
        Ok(cred)
    }
}

#[async_trait]
impl TokenRequest for ConfidentialClient {
    async fn get_token_silent(&self) -> GraphResult<Response> {
        // self.http_client.build_async().authorization_code_grant().
    }
}

impl TryFrom<AuthorizationCodeCredential> for ConfidentialClient {
    type Error = GraphFailure;

    fn try_from(value: AuthorizationCodeCredential) -> Result<Self, Self::Error> {
        let mut client = ConfidentialClient::default();

        if value.authorization_code.trim().is_empty() {
            return OAuthError::error_from(OAuthCredential::AuthorizeURL);
        }

        if value.client_id.trim().is_empty() {
            return OAuthError::error_from(OAuthCredential::ClientId);
        }

        if value.client_secret.trim().is_empty() {
            return OAuthError::error_from(OAuthCredential::ClientSecret);
        }

        client
            .http_client
            .access_code(value.authorization_code.as_str())
            .client_id(value.client_id.as_str())
            .client_secret(value.client_secret.as_str())
            .redirect_uri(value.redirect_uri.as_str())
            .extend_scopes(value.scopes)
            .authority(
                &client.token_credential_options.azure_cloud_endpoint,
                &value.tenant_id,
            );

        if let Some(code_verifier) = value.code_verifier.as_ref() {
            client.http_client.code_verifier(code_verifier.as_str());
        }

        Ok(client)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::grants::{GrantRequest, GrantType};
    use crate::identity::credentials::AuthorizationCodeCredentialBuilder;
    use url::Url;

    #[test]
    fn test_auth_code_grant_serialization() {
        let mut oauth = OAuth::new();
        oauth
            .client_id("bb301aaa-1201-4259-a230923fds32")
            .client_secret("CLDIE3F")
            .redirect_uri("http://localhost:8888/redirect")
            .grant_type("authorization_code")
            .add_scope("Read.Write")
            .add_scope("Fall.Down")
            .access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");

        let credential = AuthorizationCodeCredential::builder("ALDSKFJLKERLKJALSDKJF2209LAKJGFL")
            .with_client_id("bb301aaa-1201-4259-a230923fds32")
            .with_client_secret("CLDIE3F")
            .with_scopes(vec!["Read.Write", "Fall.Down"])
            .with_redirect_uri("http://localhost:8888/redirect")
            .build();

        let mut confidential_client = ConfidentialClient::try_from(credential).unwrap();

        let oauth_uri = oauth
            .encode_uri(GrantType::AuthorizationCode, GrantRequest::AccessToken)
            .unwrap();
        let credential_uri = confidential_client
            .http_client
            .encode_uri(GrantType::AuthorizationCode, GrantRequest::AccessToken)
            .unwrap();

        assert_eq!(oauth_uri, credential_uri);
    }

    #[test]
    fn confidential_client_new() {
        let mut oauth = OAuth::new();
        oauth
            .client_id("bb301aaa-1201-4259-a230923fds32")
            .client_secret("CLDIE3F")
            .redirect_uri("http://localhost:8888/redirect")
            .grant_type("authorization_code")
            .add_scope("Read.Write")
            .add_scope("Fall.Down")
            .access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");

        let credential = AuthorizationCodeCredential::builder("ALDSKFJLKERLKJALSDKJF2209LAKJGFL")
            .with_client_id("bb301aaa-1201-4259-a230923fds32")
            .with_client_secret("CLDIE3F")
            .with_scopes(vec!["Read.Write", "Fall.Down"])
            .with_redirect_uri("http://localhost:8888/redirect")
            .build();

        let mut confidential_client =
            ConfidentialClient::new(credential, TokenCredentialOptions::default()).unwrap();
        let oauth_uri = oauth
            .encode_uri(GrantType::AuthorizationCode, GrantRequest::AccessToken)
            .unwrap();
        let credential_uri = confidential_client
            .http_client
            .encode_uri(GrantType::AuthorizationCode, GrantRequest::AccessToken)
            .unwrap();

        assert_eq!(oauth_uri, credential_uri);
    }
}
