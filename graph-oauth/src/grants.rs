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
                    OAuthCredential::Scopes,
                ],
            },
            GrantType::CodeFlow => match grant_request {
                GrantRequest::Authorization => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::RedirectUri,
                    OAuthCredential::State,
                    OAuthCredential::ResponseType,
                    OAuthCredential::Scopes,
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
                    OAuthCredential::Scopes,
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
                    OAuthCredential::Scopes,
                    OAuthCredential::GrantType,
                    OAuthCredential::CodeVerifier,
                ],
                GrantRequest::RefreshToken => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::ClientSecret,
                    OAuthCredential::RefreshToken,
                    OAuthCredential::GrantType,
                    OAuthCredential::Scopes,
                ],
            },
            GrantType::Implicit => match grant_request {
                GrantRequest::Authorization
                | GrantRequest::AccessToken
                | GrantRequest::RefreshToken => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::RedirectUri,
                    OAuthCredential::Scopes,
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
                    OAuthCredential::Scopes,
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
                    OAuthCredential::Scopes,
                    OAuthCredential::AuthorizationCode,
                    OAuthCredential::CodeVerifier,
                ],
                GrantRequest::RefreshToken => vec![
                    OAuthCredential::ClientId,
                    OAuthCredential::ClientSecret,
                    OAuthCredential::RefreshToken,
                    OAuthCredential::GrantType,
                    OAuthCredential::Scopes,
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
                    OAuthCredential::Scopes,
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
                    OAuthCredential::Scopes,
                    OAuthCredential::RedirectUri,
                    OAuthCredential::ClientAssertion,
                ],
            },
        }
    }
}
