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

mod client_credentials_secret;
mod server_examples;

use graph_rs_sdk::{identity::ConfidentialClientApplication, Graph};
