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
mod auth_code_grant_certificate;
mod auth_code_grant_pkce;
mod client_credentials;
mod client_credentials_admin_consent;
mod code_flow;
mod device_code;
mod implicit_grant;
mod is_access_token_expired;
mod logout;
mod open_id_connect;
mod signing_keys;

use graph_rs_sdk::oauth::{
    AccessToken, AuthorizationCodeCredential, ClientSecretCredential,
    ConfidentialClientApplication, ProofKeyForCodeExchange, TokenRequest,
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
    code_flow::start_server_main().await;

    // https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc
    open_id_connect::start_server_main().await;
}

// Examples

// Authorization Code Grant
fn auth_code_grant(authorization_code: &str) -> AuthorizationCodeCredential {
    let pkce = ProofKeyForCodeExchange::generate().unwrap();

    AuthorizationCodeCredential::builder()
        .with_authorization_code(authorization_code)
        .with_client_id("CLIENT_ID")
        .with_client_secret("CLIENT_SECRET")
        .with_redirect_uri("http://localhost:8000/redirect")
        .with_proof_key_for_code_exchange(&pkce)
        .build()
}

// Client Credentials Grant
fn client_credentials() {
    pub async fn get_token_silent() {
        let client_secret_credential = ClientSecretCredential::new("CLIENT_ID", "CLIENT_SECRET");
        let mut confidential_client_application =
            ConfidentialClientApplication::from(client_secret_credential);

        let response = confidential_client_application
            .get_token_async()
            .await
            .unwrap();
        println!("{response:#?}");

        let access_token: AccessToken = response.json().await.unwrap();
        println!("{:#?}", access_token.bearer_token());
    }
}
