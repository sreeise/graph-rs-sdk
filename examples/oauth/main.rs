//! # Overview
//!
//! Most of these examples use a local server in order to listen for the redirect
//! after a user signs into microsoft. There are a few oauth flows that may use
//! other means of getting an access token such as the client credentials flow.
//!
//! # Setup
//!
//! In everyone of these examples you will first need to setup an application in the
//! azure portal.
//!
//! Microsoft Identity Platform: https://docs.microsoft.com/en-us/azure/active-directory/develop/authentication-vs-authorization
#![allow(dead_code, unused, unused_imports)]

#[macro_use]
extern crate serde;

mod auth_code_grant;
mod client_credentials;
mod device_code;
mod environment_credential;
mod is_access_token_expired;
mod openid_connect;

use crate::is_access_token_expired::is_access_token_expired;
use graph_extensions::crypto::GenPkce;
use graph_rs_sdk::oauth::{
    AuthorizationCodeCertificateCredential, AuthorizationCodeCredential,
    ClientCertificateCredential, ClientSecretCredential, ConfidentialClientApplication,
    DeviceCodeCredential, ProofKeyCodeExchange, PublicClientApplication, Token,
    TokenCredentialExecutor, TokenRequest,
};

fn main() {
    is_access_token_expired();
}

/*
   // Some examples of what you can use for authentication and getting access tokens. There are
   // more ways to perform oauth authorization.

   // https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow
   auth_code_grant::start_server_main().await;
   auth_code_grant_pkce::start_server_main().await;

   // https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow
   client_credentials_admin_consent::start_server_main().await;

   // https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code
   device_code::device_code();

   // https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc
   open_id_connect::start_server_main().await;
*/

// Quick Examples

// Authorization Code Grant
async fn auth_code_grant(authorization_code: &str) {
    let pkce = ProofKeyCodeExchange::oneshot().unwrap();

    let credential =
        AuthorizationCodeCredential::builder("CLIENT_ID", "CLIENT_SECRET", authorization_code)
            .with_redirect_uri("http://localhost:8000/redirect")
            .unwrap()
            .with_pkce(&pkce)
            .build();

    let mut confidential_client = credential;

    let response = confidential_client.execute_async().await.unwrap();
    println!("{response:#?}");

    let access_token: Token = response.json().await.unwrap();
    println!("{:#?}", access_token.access_token);
}

// Client Credentials Grant
async fn client_credentials() {
    let mut confidential_client = ConfidentialClientApplication::builder("CLIENT_ID")
        .with_client_secret("CLIENT_SECRET")
        .build();

    let response = confidential_client.execute_async().await.unwrap();
    println!("{response:#?}");

    let access_token: Token = response.json().await.unwrap();
    println!("{:#?}", access_token.access_token);
}
