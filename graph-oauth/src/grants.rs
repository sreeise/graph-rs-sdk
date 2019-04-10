use crate::auth::OAuthReq;
use crate::oautherror::OAuthError;
use reqwest::{RequestBuilder, Response};
use std::process::Output;

pub trait ClientCredentialsGrant {
    fn request_authorization(&mut self) -> OAuthReq<Output>;
    fn request_access_token(&mut self) -> OAuthReq<()>;
    fn request_refresh_token(&mut self) -> OAuthReq<()>;

    fn send(&mut self, builder: RequestBuilder) -> OAuthReq<Response> {
        builder.send().map_err(OAuthError::from)
    }
}

pub trait ImplicitGrant {
    fn request_access_token(&mut self) -> OAuthReq<Output>;
}

pub trait OpenId {
    fn request_authorization(&mut self);
}
