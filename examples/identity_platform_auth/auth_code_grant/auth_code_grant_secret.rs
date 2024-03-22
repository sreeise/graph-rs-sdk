use graph_rs_sdk::error::ErrorMessage;
use graph_rs_sdk::identity::{AuthorizationCodeCredential, ConfidentialClientApplication};
use graph_rs_sdk::*;
use url::Url;
use warp::Filter;

// Authorization Code Grant Using Client Secret

/// This is the first step in the flow. Build a url that you can use to send an end user
/// to in order to sign in. Then wait for the redirect after sign in to the redirect url
/// you specified in your app. To see a server example listening for the redirect see
/// [Auth Code Grant PKCE Server Example](https://github.com/sreeise/graph-rs-sdk/examples/oauth/auth_code_grant/auth_code_grant_secret.rs)
pub fn authorization_sign_in_url(client_id: &str, redirect_uri: Url, scope: Vec<String>) -> Url {
    AuthorizationCodeCredential::authorization_url_builder(client_id)
        .with_redirect_uri(redirect_uri)
        .with_scope(scope)
        .url()
        .unwrap()
}

async fn auth_code_grant_secret(
    authorization_code: &str,
    client_id: &str,
    client_secret: &str,
    scope: Vec<String>,
    redirect_uri: Url,
) -> anyhow::Result<GraphClient> {
    let mut confidential_client = ConfidentialClientApplication::builder(client_id)
        .with_auth_code(authorization_code) // returns builder type for AuthorizationCodeCredential
        .with_client_secret(client_secret)
        .with_scope(scope)
        .with_redirect_uri(redirect_uri)
        .build();

    let graph_client = GraphClient::from(&confidential_client);

    Ok(graph_client)
}
