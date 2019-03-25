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

pub trait OpenId {
    /*

    GET https://login.microsoftonline.com/{tenant}/oauth2/v2.0/authorize?
    client_id=6731de76-14a6-49ae-97bc-6eba6914391e
    &response_type=id_token
    &redirect_uri=http%3A%2F%2Flocalhost%2Fmyapp%2F
    &response_mode=form_post
    &scope=openid
    &state=12345
    &nonce=678910
        */
}
