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
mod auth_code_grant_pkce;
mod auth_code_grant_refresh_token;
mod client_credentials;
mod client_credentials_admin_consent;
mod device_code;
mod enable_pii_logging;
mod environment_credential;
mod implicit_grant;
mod is_access_token_expired;
mod open_id_connect;
mod signing_keys;

use graph_rs_sdk::oauth::{
    AccessToken, AuthorizationCodeCertificateCredential, AuthorizationCodeCredential,
    ClientCertificateCredential, ClientSecretCredential, ConfidentialClientApplication,
    CredentialBuilder, DeviceCodeCredential, ProofKeyForCodeExchange, PublicClientApplication,
    TokenRequest,
};

#[tokio::main]
async fn main() {
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
}

// Quick Examples

// Authorization Code Grant
async fn auth_code_grant(authorization_code: &str) {
    let pkce = ProofKeyForCodeExchange::generate().unwrap();

    let credential = AuthorizationCodeCredential::builder()
        .with_authorization_code(authorization_code)
        .with_client_id("CLIENT_ID")
        .with_client_secret("CLIENT_SECRET")
        .with_redirect_uri("http://localhost:8000/redirect")
        .unwrap()
        .with_proof_key_for_code_exchange(&pkce)
        .build();

    let mut confidential_client = ConfidentialClientApplication::from(credential);

    let response = confidential_client.get_token_async().await.unwrap();
    println!("{response:#?}");

    let access_token: AccessToken = response.json().await.unwrap();
    println!("{:#?}", access_token.access_token);
}

// Client Credentials Grant
async fn client_credentials() {
    let client_secret_credential = ClientSecretCredential::new("CLIENT_ID", "CLIENT_SECRET");
    let mut confidential_client = ConfidentialClientApplication::from(client_secret_credential);

    let response = confidential_client.get_token_async().await.unwrap();
    println!("{response:#?}");

    let access_token: AccessToken = response.json().await.unwrap();
    println!("{:#?}", access_token.access_token);
}
