//! # Setup
//!
//! You will first need to setup an application in the azure portal.
//!
//! Microsoft Identity Platform: https://docs.microsoft.com/en-us/azure/active-directory/develop/authentication-vs-authorization
#![allow(dead_code, unused, unused_imports)]

#[macro_use]
extern crate serde;

mod legacy;
mod openid_connect;

use graph_rs_sdk::oauth::{
    AuthorizationCodeCertificateCredential, AuthorizationCodeCredential,
    ClientCertificateCredential, ClientSecretCredential, ConfidentialClientApplication,
    DeviceCodeCredential, MsalToken, ProofKeyForCodeExchange, PublicClientApplication,
    TokenCredentialExecutor, TokenRequest,
};

fn main() {}

static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";
static REDIRECT_URI: &str = "http://localhost:8000/redirect";
static SCOPE: &str = "User.Read"; // or pass more values to vec![] below

// Authorization Code Grant Auth URL Builder
pub fn auth_code_grant_authorization() {
    let url = AuthorizationCodeCredential::authorization_url_builder(CLIENT_ID)
        .with_redirect_uri(REDIRECT_URI)
        .with_scope(vec![SCOPE])
        .url()
        .unwrap();

    // web browser crate opens default browser.
    webbrowser::open(url.as_str()).unwrap();
}

// Authorization Code Grant PKCE

// This example shows how to generate a code_challenge and code_verifier
// to perform the authorization code grant flow with proof key for
// code exchange (PKCE) otherwise known as an assertion.
//
// You can also use values of your own for the assertion.
//
// For more info see: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow
// And the PKCE RFC: https://tools.ietf.org/html/rfc7636

// Open the default system web browser to the sign in url for authorization.
// This method uses AuthorizationCodeAuthorizationUrl to build the sign in
// url and query needed to get an authorization code and opens the default system
// web browser to this Url.
fn auth_code_grant_pkce_authorization() {
    let pkce = ProofKeyForCodeExchange::generate().unwrap();

    let url = AuthorizationCodeCredential::authorization_url_builder(CLIENT_ID)
        .with_scope(vec![SCOPE])
        .with_redirect_uri(REDIRECT_URI)
        .with_pkce(&pkce)
        .url()
        .unwrap();

    webbrowser::open(url.as_str()).unwrap();
}
