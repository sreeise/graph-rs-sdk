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
#![allow(dead_code, unused, unused_imports, clippy::module_inception)]

#[macro_use]
extern crate serde;

mod auth_code_grant;
mod client_credentials;
mod device_code;
mod environment_credential;
mod getting_tokens_manually;
mod is_access_token_expired;
mod openid;

use graph_rs_sdk::oauth::{
    AuthorizationCodeCertificateCredential, AuthorizationCodeCredential,
    ClientCertificateCredential, ClientSecretCredential, ConfidentialClientApplication,
    DeviceCodeCredential, GenPkce, ProofKeyCodeExchange, PublicClientApplication, Token,
    TokenCredentialExecutor,
};
use graph_rs_sdk::GraphClient;

fn main() {}

// Authorization Code Grant
async fn auth_code_grant(
    authorization_code: &str,
    client_id: &str,
    client_secret: &str,
    scope: Vec<String>,
    redirect_uri: &str,
) {
    let mut confidential_client =
        AuthorizationCodeCredential::builder(client_id, client_secret, authorization_code)
            .with_scope(scope)
            .with_redirect_uri(redirect_uri)
            .unwrap()
            .build();

    let _graph_client = GraphClient::from(&confidential_client);
}

// Client Credentials Grant
async fn client_credentials() {
    let mut confidential_client = ConfidentialClientApplication::builder("CLIENT_ID")
        .with_client_secret("CLIENT_SECRET")
        .with_tenant("TENANT_ID")
        .build();

    let _graph_client = GraphClient::from(&confidential_client);
}
