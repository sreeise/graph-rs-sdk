use graph_rs_sdk::identity::{
    AuthorizationCodeSpaCredential, GenPkce, ProofKeyCodeExchange, PublicClientApplication,
    TokenCredentialExecutor,
};
use lazy_static::lazy_static;
use url::Url;

// You can also pass your own values for PKCE instead of automatic generation by
// calling ProofKeyCodeExchange::new(code_verifier, code_challenge, code_challenge_method)
lazy_static! {
    static ref PKCE: ProofKeyCodeExchange = ProofKeyCodeExchange::oneshot().unwrap();
}

// This example shows how Spa applications can use a code_challenge and code_verifier
// to perform the authorization code grant flow with proof key for
// code exchange (PKCE).
//
// Spa applications do not use a client secret.
//
// For more info see: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow
// And the PKCE RFC: https://tools.ietf.org/html/rfc7636

/// This is the first step in the flow. Build a url that you can use to send an end user
/// to in order to sign in. Then wait for the redirect after sign in to the redirect url
/// you specified in your app. To see a server example listening for the redirect see
/// [Auth Code Grant PKCE Server Example](https://github.com/sreeise/graph-rs-sdk/examples/oauth/auth_code_grant/auth_code_grant_pkce.rs)
fn authorization_sign_in_url(
    client_id: &str,
    redirect_uri: &str,
    scope: Vec<String>,
) -> anyhow::Result<Url> {
    Ok(
        AuthorizationCodeSpaCredential::authorization_url_builder(client_id)
            .with_scope(scope)
            .with_redirect_uri(Url::parse(redirect_uri)?)
            .with_pkce(&PKCE)
            .url()?,
    )
}

fn build_confidential_client(
    authorization_code: &str,
    client_id: &str,
    redirect_uri: &str,
    scope: Vec<String>,
) -> anyhow::Result<PublicClientApplication<AuthorizationCodeSpaCredential>> {
    Ok(PublicClientApplication::builder(client_id)
        .with_auth_code(authorization_code)
        .with_scope(scope)
        .with_redirect_uri(Url::parse(redirect_uri)?)
        .with_pkce(&PKCE)
        .build())
}
