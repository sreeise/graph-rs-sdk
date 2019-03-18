use crate::auth::OAuthReq;
use std::process::Output;

pub trait ClientCredentialsGrant {
    fn authorization_request(&mut self) -> OAuthReq<Output>;
    fn request_access_token(&mut self) -> OAuthReq<()>;
    fn request_refresh_token(&mut self) -> OAuthReq<()>;
}

pub trait ImplicitGrant {
    fn request_access_token(&mut self) -> OAuthReq<Output>;
}
