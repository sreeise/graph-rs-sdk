//! # Setup
//!
//! You will first need to setup an application in the azure portal.
//!
//! Microsoft Identity Platform: https://docs.microsoft.com/en-us/azure/active-directory/develop/authentication-vs-authorization
#![allow(dead_code, unused, unused_imports)]

#[macro_use]
extern crate serde;

mod implicit_grant;
mod open_id_connect;

use graph_rs_sdk::oauth::{
    AuthorizationCodeCertificateCredential, AuthorizationCodeCredential,
    ClientCertificateCredential, ClientSecretCredential, ConfidentialClientApplication,
    DeviceCodeCredential, MsalTokenResponse, ProofKeyForCodeExchange, PublicClientApplication,
    TokenCredentialExecutor, TokenRequest,
};

fn main() {}

static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";

// Authorization Code Grant Auth URL Builder
pub fn auth_code_grant_authorization() {
    let url = AuthorizationCodeCredential::authorization_url_builder()
        .with_client_id(CLIENT_ID)
        .with_redirect_uri("http://localhost:8000/redirect")
        .with_scope(vec!["offline_access", "files.read"])
        .url()
        .unwrap();

    // web browser crate in dev dependencies will open to default browser in the system.
    webbrowser::open(url.as_str()).unwrap();
}

// Authorization Code Grant PKCE

// This example shows how to use a code_challenge and code_verifier
// to perform the authorization code grant flow with proof key for
// code exchange (PKCE).
//
// For more info see: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow
// And the PKCE RFC: https://tools.ietf.org/html/rfc7636

// Open the default system web browser to the sign in url for authorization.
// This method uses AuthorizationCodeAuthorizationUrl to build the sign in
// url and query needed to get an authorization code and opens the default system
// web browser to this Url.
fn auth_code_grant_pkce_authorization() {
    let pkce = ProofKeyForCodeExchange::generate().unwrap();

    let url = AuthorizationCodeCredential::authorization_url_builder()
        .with_client_id(CLIENT_ID)
        .with_scope(vec!["user.read"])
        .with_redirect_uri("http://localhost:8000/redirect")
        .with_pkce(&pkce)
        .url()
        .unwrap();

    webbrowser::open(url.as_str()).unwrap();
}
