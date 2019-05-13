use crate::accesstoken::AccessToken;
use crate::auth::OAuthReq;
use reqwest::{header, RequestBuilder};
use std::process::Output;
use transform_request::Transform;

pub struct OAuthTooling;

impl OAuthTooling {
    pub fn open_in_browser(url: &str) -> OAuthReq<Output> {
        Ok(webbrowser::open(url)?)
    }

    pub fn bearer_request_builder(
        url: &str,
        code_body: &str,
        access_code: &str,
    ) -> OAuthReq<RequestBuilder> {
        let client = reqwest::Client::builder().build()?;
        Ok(client
            .post(url)
            .header(header::AUTHORIZATION, access_code)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(code_body.to_string()))
    }

    pub fn token_request_builder(url: &str, code_body: &str) -> OAuthReq<RequestBuilder> {
        let client = reqwest::Client::builder().build()?;
        Ok(client
            .post(url)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(code_body.to_string()))
    }

    pub fn bearer_access_token(url: &str, body: &str, access_code: &str) -> OAuthReq<AccessToken> {
        let builder = OAuthTooling::bearer_request_builder(url.trim(), body, access_code)?;
        let access_token = AccessToken::transform(builder)?;
        Ok(access_token)
    }

    pub fn post_access_token(url: &str, body: &str) -> OAuthReq<AccessToken> {
        let builder = OAuthTooling::token_request_builder(url.trim(), body)?;
        let access_token = AccessToken::transform(builder)?;
        Ok(access_token)
    }
}
