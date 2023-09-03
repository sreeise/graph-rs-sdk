use graph_oauth::identity::{DeviceCodeCredential, TokenCredentialExecutor};
use graph_oauth::oauth::DeviceCodeCredentialBuilder;
use graph_rs_sdk::oauth::{MsalTokenResponse, OAuthSerializer};
use graph_rs_sdk::GraphResult;
use std::time::Duration;

// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code

// Update the client id with your own.
fn get_oauth() -> OAuthSerializer {
    let client_id = "CLIENT_ID";
    let mut oauth = OAuthSerializer::new();

    oauth
        .client_id(client_id)
        .authorization_url("https://login.microsoftonline.com/common/oauth2/v2.0/devicecode")
        .refresh_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .token_uri("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .add_scope("files.read")
        .add_scope("offline_access");

    oauth
}
