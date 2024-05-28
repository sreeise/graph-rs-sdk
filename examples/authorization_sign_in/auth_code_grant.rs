use graph_rs_sdk::identity::{
    AuthorizationCodeCredential, ClientCertificateCredential, ClientSecretCredential,
    ConfidentialClientApplication, DeviceCodeCredential, GenPkce, ProofKeyCodeExchange, Token,
    TokenCredentialExecutor,
};
use url::Url;

static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";
const REDIRECT_URI: &str = "http://localhost:8000/redirect";
static SCOPE: &str = "User.Read"; // or pass more values to vec![] below

// Authorization Code Grant Auth URL Builder
pub fn auth_code_grant_authorization() {
    let url = AuthorizationCodeCredential::authorization_url_builder(CLIENT_ID)
        .with_redirect_uri(Url::parse(REDIRECT_URI).unwrap())
        .with_scope(vec![SCOPE])
        .url()
        .unwrap();

    // web browser crate opens default browser.
    webbrowser::open(url.as_str()).unwrap();
}

// Authorization Code Grant PKCE

// This example shows how to generate a code_challenge and code_verifier
// to perform the authorization code grant flow with proof key for
// code exchange (PKCE) otherwise known as an assertion.
//
// You can also use values of your own for the assertion.
//
// For more info see: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow
// And the PKCE RFC: https://tools.ietf.org/html/rfc7636

// Open the default system web browser to the sign in url for authorization.
// This method uses AuthorizationCodeAuthorizationUrl to build the sign in
// url and query needed to get an authorization code and opens the default system
// web browser to this Url.
fn auth_code_grant_pkce_authorization() {
    let pkce = ProofKeyCodeExchange::oneshot().unwrap();

    let url = AuthorizationCodeCredential::authorization_url_builder(CLIENT_ID)
        .with_scope(vec![SCOPE])
        .with_redirect_uri(Url::parse(REDIRECT_URI).unwrap())
        .with_pkce(&pkce)
        .url()
        .unwrap();

    webbrowser::open(url.as_str()).unwrap();
}
