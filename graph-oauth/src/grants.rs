use crate::auth::OAuthReq;
use std::process::Output;

pub enum GrantRequest {
    Authorization,
    AccessToken,
    RefreshToken,
}

pub enum GrantType {
    ClientCredentials(GrantRequest),
    Implicit(GrantRequest),
    OpenId(GrantRequest),
}

pub trait ClientCredentialsGrant {
    fn request_authorization(&mut self) -> OAuthReq<Output>;
    fn request_access_token(&mut self) -> OAuthReq<()>;
    fn request_refresh_token(&mut self) -> OAuthReq<()>;
}

pub trait ImplicitGrant {
    fn request_access_token(&mut self) -> OAuthReq<Output>;
}

pub trait OpenId {
    fn request_authorization(&mut self) -> OAuthReq<Output>;
    fn request_access_token(&mut self) -> OAuthReq<()>;
}
