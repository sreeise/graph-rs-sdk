use graph_core::identity::ClientApplication;
use graph_rs_sdk::identity::{
    AuthorizationCodeCredential, ConfidentialClientApplication, Token, TokenCredentialExecutor,
};
use url::Url;

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
            .with_redirect_uri(Url::parse(redirect_uri).unwrap())
            .build();

    let response = confidential_client.execute_async().await.unwrap();
    println!("{response:#?}");

    let token: Token = response.json().await.unwrap();
    println!("{:#?}", token.access_token);
}

// Client Credentials Grant
async fn client_credentials() {
    let mut confidential_client = ConfidentialClientApplication::builder("CLIENT_ID")
        .with_client_secret("CLIENT_SECRET")
        .with_tenant("TENANT_ID")
        .build();

    let response = confidential_client.execute_async().await.unwrap();
    println!("{response:#?}");

    let token: Token = response.json().await.unwrap();
    println!("{:#?}", token.access_token);
}

// Use get_token_silent and get_token_silent_async to have the
// credential client check the in memory token cache before making
// an http request to get a new token.

// The execute and execute_async methods do not store or retrieve any
// tokens from the cache.
async fn using_token_cache() {
    let mut confidential_client = ConfidentialClientApplication::builder("CLIENT_ID")
        .with_client_secret("CLIENT_SECRET")
        .with_tenant("TENANT_ID")
        .build();

    let access_token = confidential_client.get_token_silent_async().await.unwrap();
    println!("{access_token:#?}");
}
