mod auth_code_grant;
mod auth_code_grant_pkce;
mod client_credentials;
mod code_flow;
mod implicit_grant;
mod open_id_connect;

// Most of these examples use a local server in order to listen for the redirect
// after a user signs into microsoft. There are a few oauth flows that may use
// other means of getting an access token such as the client credentials flow.
// The client credentials flow first requires an admin consent sign in and
// after that sign in is done the application can use the users email and password
// to sign in.

// In everyone of these examples you will first need to setup an application in the
// azure portal.

// Microsoft Identity Platform: https://docs.microsoft.com/en-us/azure/active-directory/develop/authentication-vs-authorization

#[tokio::main]
fn main() {
    // Some examples of what you can use for authentication and getting access tokens. There are
    // more ways to perform oauth authorization.

    // https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow
    auth_code_grant::start_server_main().await;
    auth_code_grant_pkce::start_server_main().await;

    // https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow
    client_credentials::start_server_main().await;

    // https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code
    code_flow::start_server_main().await;

    // https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc
    open_id_connect::start_server_main().await;
}
