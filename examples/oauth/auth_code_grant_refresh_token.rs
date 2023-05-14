use graph_oauth::identity::AuthorizationCodeCredentialBuilder;
use graph_rs_sdk::oauth::{
    AuthorizationCodeCredential, ConfidentialClientApplication, CredentialBuilder, TokenRequest,
};

// Use a refresh token to get a new access token.

async fn using_auth_code_credential(
    credential: &mut AuthorizationCodeCredential,
    refresh_token: &str,
) {
    credential.with_refresh_token(refresh_token);

    let _response = credential.get_token_async().await;
}

async fn using_confidential_client(
    mut credential: AuthorizationCodeCredential,
    refresh_token: &str,
) {
    credential.with_refresh_token(refresh_token);
    let mut confidential_client = ConfidentialClientApplication::from(credential);

    let _response = confidential_client.get_token_async().await;
}

async fn using_auth_code_credential_builder(
    credential: AuthorizationCodeCredential,
    refresh_token: &str,
) {
    let mut credential = AuthorizationCodeCredentialBuilder::from(credential)
        .with_refresh_token(refresh_token)
        .build();

    let _response = credential.get_token_async().await;
}
