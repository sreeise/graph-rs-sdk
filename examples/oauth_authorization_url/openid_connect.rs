use graph_error::IdentityResult;
use graph_rs_sdk::oauth::{
    ConfidentialClientApplication, OpenIdCredential, Prompt, ResponseMode, ResponseType,
};
use url::Url;

// The authorization request is the initial request to sign in where the user
// is taken to the sign in page and enters their credentials.
// If successful the user will be redirected back to your app and the authorization
// code will be in the query of the URL.

// If you are listening on a server use the response mod ResponseMode::FormPost.
// Servers do not get sent the URL query and so in order to get what would normally be in
// the query of URL use a form post which sends the data as a POST http request.

// The URL builder below will create the full URL with the query that you will
// need to send the user to such as redirecting the page they are on when using
// your app to the URL.

// See examples/oauth/openid for a full example.

// Use the form post response mode when listening on a server instead
// of the URL query because the the query does not get sent to servers.
fn openid_authorization_url3(
    client_id: &str,
    tenant: &str,
    redirect_uri: &str,
    state: &str,
    scope: Vec<&str>,
) -> IdentityResult<Url> {
    OpenIdCredential::authorization_url_builder(client_id)
        .with_tenant(tenant)
        //.with_default_scope()?
        .with_redirect_uri(redirect_uri)?
        .with_response_mode(ResponseMode::FormPost)
        .with_response_type([ResponseType::IdToken, ResponseType::Code])
        .with_prompt(Prompt::SelectAccount)
        .with_state(state)
        .with_scope(scope)
        .build()
        .url()
}
fn open_id_authorization_url(
    client_id: &str,
    tenant: &str,
    redirect_uri: &str,
    scope: Vec<&str>,
) -> IdentityResult<Url> {
    ConfidentialClientApplication::builder(client_id)
        .openid_url_builder()
        .with_tenant(tenant)
        .with_redirect_uri(redirect_uri)?
        .with_scope(scope)
        .build()
        .url()
}

// Same as above
fn open_id_authorization_url2(
    client_id: &str,
    tenant: &str,
    redirect_uri: &str,
    scope: Vec<&str>,
) -> IdentityResult<Url> {
    OpenIdCredential::authorization_url_builder(client_id)
        .with_tenant(tenant)
        .with_redirect_uri(redirect_uri)?
        .with_scope(scope)
        .build()
        .url()
}
