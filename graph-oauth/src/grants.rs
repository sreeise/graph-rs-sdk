use crate::auth::OAuthParameter;

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
    pub fn available_credentials(self, grant_request: GrantRequest) -> Vec<OAuthParameter> {
        match self {
            GrantType::TokenFlow => match grant_request {
                GrantRequest::Authorization
                | GrantRequest::AccessToken
                | GrantRequest::RefreshToken => vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::RedirectUri,
                    OAuthParameter::ResponseType,
                    OAuthParameter::Scope,
                ],
            },
            GrantType::CodeFlow => match grant_request {
                GrantRequest::Authorization => vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::RedirectUri,
                    OAuthParameter::State,
                    OAuthParameter::ResponseType,
                    OAuthParameter::Scope,
                ],
                GrantRequest::AccessToken => vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::ClientSecret,
                    OAuthParameter::RedirectUri,
                    OAuthParameter::ResponseType,
                    OAuthParameter::GrantType,
                    OAuthParameter::AuthorizationCode,
                ],
                GrantRequest::RefreshToken => vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::ClientSecret,
                    OAuthParameter::RedirectUri,
                    OAuthParameter::GrantType,
                    OAuthParameter::AuthorizationCode,
                    OAuthParameter::RefreshToken,
                ],
            },
            GrantType::AuthorizationCode => match grant_request {
                GrantRequest::Authorization => vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::RedirectUri,
                    OAuthParameter::State,
                    OAuthParameter::ResponseMode,
                    OAuthParameter::ResponseType,
                    OAuthParameter::Scope,
                    OAuthParameter::Prompt,
                    OAuthParameter::DomainHint,
                    OAuthParameter::LoginHint,
                    OAuthParameter::CodeChallenge,
                    OAuthParameter::CodeChallengeMethod,
                ],
                GrantRequest::AccessToken => vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::ClientSecret,
                    OAuthParameter::RedirectUri,
                    OAuthParameter::AuthorizationCode,
                    OAuthParameter::Scope,
                    OAuthParameter::GrantType,
                    OAuthParameter::CodeVerifier,
                ],
                GrantRequest::RefreshToken => vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::ClientSecret,
                    OAuthParameter::RefreshToken,
                    OAuthParameter::GrantType,
                    OAuthParameter::Scope,
                ],
            },
            GrantType::Implicit => match grant_request {
                GrantRequest::Authorization
                | GrantRequest::AccessToken
                | GrantRequest::RefreshToken => vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::RedirectUri,
                    OAuthParameter::Scope,
                    OAuthParameter::ResponseType,
                    OAuthParameter::ResponseMode,
                    OAuthParameter::State,
                    OAuthParameter::Nonce,
                    OAuthParameter::Prompt,
                    OAuthParameter::LoginHint,
                    OAuthParameter::DomainHint,
                ],
            },
            GrantType::OpenId => match grant_request {
                GrantRequest::Authorization => vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::ResponseType,
                    OAuthParameter::RedirectUri,
                    OAuthParameter::ResponseMode,
                    OAuthParameter::Scope,
                    OAuthParameter::State,
                    OAuthParameter::Nonce,
                    OAuthParameter::Prompt,
                    OAuthParameter::LoginHint,
                    OAuthParameter::DomainHint,
                    OAuthParameter::Resource,
                ],
                GrantRequest::AccessToken => vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::ClientSecret,
                    OAuthParameter::RedirectUri,
                    OAuthParameter::GrantType,
                    OAuthParameter::Scope,
                    OAuthParameter::AuthorizationCode,
                    OAuthParameter::CodeVerifier,
                ],
                GrantRequest::RefreshToken => vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::ClientSecret,
                    OAuthParameter::RefreshToken,
                    OAuthParameter::GrantType,
                    OAuthParameter::Scope,
                ],
            },
            GrantType::ClientCredentials => match grant_request {
                GrantRequest::Authorization => vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::RedirectUri,
                    OAuthParameter::State,
                ],
                GrantRequest::AccessToken | GrantRequest::RefreshToken => vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::ClientSecret,
                    OAuthParameter::GrantType,
                    OAuthParameter::Scope,
                    OAuthParameter::ClientAssertion,
                    OAuthParameter::ClientAssertionType,
                ],
            },
            GrantType::ResourceOwnerPasswordCredentials => match grant_request {
                GrantRequest::Authorization
                | GrantRequest::AccessToken
                | GrantRequest::RefreshToken => vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::ClientSecret,
                    OAuthParameter::GrantType,
                    OAuthParameter::Username,
                    OAuthParameter::Password,
                    OAuthParameter::Scope,
                    OAuthParameter::RedirectUri,
                    OAuthParameter::ClientAssertion,
                ],
            },
            GrantType::DeviceCode => match grant_request {
                GrantRequest::Authorization => {
                    vec![OAuthParameter::ClientId, OAuthParameter::Scope]
                }
                GrantRequest::AccessToken => vec![
                    OAuthParameter::GrantType,
                    OAuthParameter::ClientId,
                    OAuthParameter::DeviceCode,
                ],
                GrantRequest::RefreshToken => vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::Scope,
                    OAuthParameter::GrantType,
                    OAuthParameter::RefreshToken,
                ],
            },
        }
    }
}
