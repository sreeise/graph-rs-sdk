use crate::auth::OAuthCredential;

#[derive(
    Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize, EnumIter,
)]
pub enum GrantRequest {
    Authorization,
    AccessToken,
    RefreshToken,
}

#[derive(
    Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize, EnumIter,
)]
pub enum GrantType {
    TokenFlow,
    CodeFlow,
    AuthorizationCode,
    Implicit,
    DeviceCode,
    OpenId,
    ClientCredentials,
    ResourceOwnerPasswordCredentials,
}

impl GrantType {
    pub fn available_credentials(self, grant_request: GrantRequest) -> Vec<OAuthCredential> {
        match self {
            GrantType::TokenFlow => match grant_request {
                GrantRequest::Authorization
                | GrantRequest::AccessToken
                | GrantRequest::RefreshToken => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::RedirectUri,
                    OAuthCredential::ResponseType,
                    OAuthCredential::Scope,
                ],
            },
            GrantType::CodeFlow => match grant_request {
                GrantRequest::Authorization => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::RedirectUri,
                    OAuthCredential::State,
                    OAuthCredential::ResponseType,
                    OAuthCredential::Scope,
                ],
                GrantRequest::AccessToken => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::ClientSecret,
                    OAuthCredential::RedirectUri,
                    OAuthCredential::ResponseType,
                    OAuthCredential::GrantType,
                    OAuthCredential::AuthorizationCode,
                ],
                GrantRequest::RefreshToken => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::ClientSecret,
                    OAuthCredential::RedirectUri,
                    OAuthCredential::GrantType,
                    OAuthCredential::AuthorizationCode,
                    OAuthCredential::RefreshToken,
                ],
            },
            GrantType::AuthorizationCode => match grant_request {
                GrantRequest::Authorization => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::RedirectUri,
                    OAuthCredential::State,
                    OAuthCredential::ResponseMode,
                    OAuthCredential::ResponseType,
                    OAuthCredential::Scope,
                    OAuthCredential::Prompt,
                    OAuthCredential::DomainHint,
                    OAuthCredential::LoginHint,
                    OAuthCredential::CodeChallenge,
                    OAuthCredential::CodeChallengeMethod,
                ],
                GrantRequest::AccessToken => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::ClientSecret,
                    OAuthCredential::RedirectUri,
                    OAuthCredential::AuthorizationCode,
                    OAuthCredential::Scope,
                    OAuthCredential::GrantType,
                    OAuthCredential::CodeVerifier,
                ],
                GrantRequest::RefreshToken => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::ClientSecret,
                    OAuthCredential::RefreshToken,
                    OAuthCredential::GrantType,
                    OAuthCredential::Scope,
                ],
            },
            GrantType::Implicit => match grant_request {
                GrantRequest::Authorization
                | GrantRequest::AccessToken
                | GrantRequest::RefreshToken => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::RedirectUri,
                    OAuthCredential::Scope,
                    OAuthCredential::ResponseType,
                    OAuthCredential::ResponseMode,
                    OAuthCredential::State,
                    OAuthCredential::Nonce,
                    OAuthCredential::Prompt,
                    OAuthCredential::LoginHint,
                    OAuthCredential::DomainHint,
                ],
            },
            GrantType::OpenId => match grant_request {
                GrantRequest::Authorization => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::ResponseType,
                    OAuthCredential::RedirectUri,
                    OAuthCredential::ResponseMode,
                    OAuthCredential::Scope,
                    OAuthCredential::State,
                    OAuthCredential::Nonce,
                    OAuthCredential::Prompt,
                    OAuthCredential::LoginHint,
                    OAuthCredential::DomainHint,
                    OAuthCredential::Resource,
                ],
                GrantRequest::AccessToken => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::ClientSecret,
                    OAuthCredential::RedirectUri,
                    OAuthCredential::GrantType,
                    OAuthCredential::Scope,
                    OAuthCredential::AuthorizationCode,
                    OAuthCredential::CodeVerifier,
                ],
                GrantRequest::RefreshToken => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::ClientSecret,
                    OAuthCredential::RefreshToken,
                    OAuthCredential::GrantType,
                    OAuthCredential::Scope,
                ],
            },
            GrantType::ClientCredentials => match grant_request {
                GrantRequest::Authorization => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::RedirectUri,
                    OAuthCredential::State,
                ],
                GrantRequest::AccessToken | GrantRequest::RefreshToken => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::ClientSecret,
                    OAuthCredential::GrantType,
                    OAuthCredential::Scope,
                    OAuthCredential::ClientAssertion,
                    OAuthCredential::ClientAssertionType,
                ],
            },
            GrantType::ResourceOwnerPasswordCredentials => match grant_request {
                GrantRequest::Authorization
                | GrantRequest::AccessToken
                | GrantRequest::RefreshToken => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::ClientSecret,
                    OAuthCredential::GrantType,
                    OAuthCredential::Username,
                    OAuthCredential::Password,
                    OAuthCredential::Scope,
                    OAuthCredential::RedirectUri,
                    OAuthCredential::ClientAssertion,
                ],
            },
            GrantType::DeviceCode => match grant_request {
                GrantRequest::Authorization => {
                    vec![OAuthCredential::ClientId, OAuthCredential::Scope]
                }
                GrantRequest::AccessToken => vec![
                    OAuthCredential::GrantType,
                    OAuthCredential::ClientId,
                    OAuthCredential::DeviceCode,
                ],
                GrantRequest::RefreshToken => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::Scope,
                    OAuthCredential::GrantType,
                    OAuthCredential::RefreshToken,
                ],
            },
        }
    }
}
