use crate::auth::OAuthReq;
use std::process::Output;

pub enum GrantRequest {
    Authorization,
    AccessToken,
    RefreshToken,
}

pub enum GrantType {
    TokenFlow(GrantRequest),
    CodeFlow(GrantRequest),
    AuthorizationCode(GrantRequest),
    ClientCredentials(GrantRequest),
    Implicit(GrantRequest),
    OpenId(GrantRequest),
}

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online
pub trait TokenFlow {
    fn request_access_token(&mut self) -> OAuthReq<Output>;
}

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online
pub trait CodeFlow {
    fn request_authorization(&mut self) -> OAuthReq<Output>;
    fn request_access_token(&mut self) -> OAuthReq<()>;
    fn request_refresh_token(&mut self) -> OAuthReq<()>;
}

// https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow
pub trait AuthorizationCodeGrant {
    fn request_authorization(&mut self) -> OAuthReq<Output>;
}

// https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow
pub trait ClientCredentialsGrant {
    fn request_authorization(&mut self) -> OAuthReq<Output>;
    fn request_access_token(&mut self) -> OAuthReq<()>;
}

// https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-implicit-grant-flow
pub trait ImplicitGrant {
    fn request_access_token(&mut self) -> OAuthReq<Output>;
}

// https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc
pub trait OpenIdConnect {
    fn request_authorization(&mut self) -> OAuthReq<Output>;
    fn request_access_token(&mut self) -> OAuthReq<()>;
    fn request_refresh_token(&mut self) -> OAuthReq<()>;
}
