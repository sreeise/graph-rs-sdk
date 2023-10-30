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

mod client_credentials_admin_consent;

pub use client_credentials_admin_consent::*;

use graph_rs_sdk::{oauth::ConfidentialClientApplication, Graph};

// This example shows programmatically getting an access token using the client credentials
// flow after admin consent has been granted. If you have not granted admin consent, see
// examples/client_credentials_admin_consent.rs for more info.

// Replace client id, client secret, and tenant id with your own values.
static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";
static TENANT_ID: &str = "<TENANT_ID>";

pub async fn get_graph_client() -> Graph {
    let mut confidential_client_application = ConfidentialClientApplication::builder(CLIENT_ID)
        .with_client_secret(CLIENT_SECRET)
        .with_tenant(TENANT_ID)
        .build();

    Graph::from(&confidential_client_application)
}
