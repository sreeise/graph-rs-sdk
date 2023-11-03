use graph_rs_sdk::oauth::{
    AuthorizationCodeCredential, ConfidentialClientApplication, Token, TokenCredentialExecutor,
};

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

    let response = confidential_client.execute_async().await.unwrap();
    println!("{response:#?}");

    let access_token: Token = response.json().await.unwrap();
    println!("{:#?}", access_token.access_token);
}

// Client Credentials Grant
async fn client_credentials() {
    let mut confidential_client = ConfidentialClientApplication::builder("CLIENT_ID")
        .with_client_secret("CLIENT_SECRET")
        .build();

    let response = confidential_client.execute_async().await.unwrap();
    println!("{response:#?}");

    let access_token: Token = response.json().await.unwrap();
    println!("{:#?}", access_token.access_token);
}
