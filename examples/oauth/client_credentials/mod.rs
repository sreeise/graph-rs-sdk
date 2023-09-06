// Microsoft Client Credentials: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow)
// You can use the OAuth 2.0 client credentials grant specified in RFC 6749,
// sometimes called two-legged OAuth, to access web-hosted resources by using the
// identity of an application. This type of grant is commonly used for server-to-server
// interactions that must run in the background, without immediate interaction with a user.
// These types of applications are often referred to as daemons or service accounts.
//
// This OAuth flow example requires signing in as an administrator for Azure, known as admin consent,
// to approve your application to call Microsoft Graph Apis on behalf of a user. Admin consent
// only has to be done once for a user. After admin consent is given, the oauth client can be
// used to continue getting new access tokens programmatically.
use graph_rs_sdk::oauth::{
    ClientSecretCredential, ConfidentialClientApplication, MsalTokenResponse,
    TokenCredentialExecutor, TokenRequest,
};

mod client_credentials_admin_consent;

pub use client_credentials_admin_consent::*;

// This example shows programmatically getting an access token using the client credentials
// flow after admin consent has been granted. If you have not granted admin consent, see
// examples/client_credentials_admin_consent.rs for more info.

// The client_id and client_secret must be changed before running this example.
static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";

pub async fn get_token_silent() {
    let client_secret_credential = ClientSecretCredential::new(CLIENT_ID, CLIENT_SECRET);
    let mut confidential_client_application =
        ConfidentialClientApplication::from(client_secret_credential);

    let response = confidential_client_application
        .execute_async()
        .await
        .unwrap();
    println!("{response:#?}");

    let body: MsalTokenResponse = response.json().await.unwrap();
}

pub async fn get_token_silent2() {
    let mut confidential_client = ConfidentialClientApplication::builder(CLIENT_ID)
        .with_client_secret(CLIENT_SECRET)
        .build();

    let response = confidential_client.execute_async().await.unwrap();
    println!("{response:#?}");

    let body: MsalTokenResponse = response.json().await.unwrap();
}