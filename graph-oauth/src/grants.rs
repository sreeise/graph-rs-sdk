use crate::auth::OAuthReq;
use std::process::Output;

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
    ClientCredentials,
    Implicit,
    OpenId,
}

pub trait Grant {
    fn request_authorization(&mut self) -> OAuthReq<Output>;
    fn request_access_token(&mut self) -> OAuthReq<()>;
    fn request_refresh_token(&mut self) -> OAuthReq<()>;
}
