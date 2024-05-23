use graph_rs_sdk::error::IdentityResult;
use graph_rs_sdk::identity::{
    ConfidentialClientApplication, OpenIdCredential, Prompt, ResponseMode, ResponseType,
};
use graph_rs_sdk::GraphClient;
use url::Url;

// The authorization request is the initial request to sign in where the user
// is taken to the sign in page and enters their credentials.
// If successful the user will be redirected back to your app and the authorization
// code will be in the query of the URL.

// If you are listening on a server use the response mod ResponseMode::FormPost.
// Servers do not get sent the URL query and so in order to get what would normally be in
// the query of URL use a form post which sends the data as a POST http request.
// Furthermore, openid does not support the query response mode but does support fragment.

// The URL builder below will create the full URL with the query that you will
// need to send the user to such as redirecting the page they are on when using
// your app to the URL.

// See examples/oauth/openid for a full example.

// Use the form post response mode when listening on a server instead
// of the URL query because the the query does not get sent to servers.
fn openid_authorization_url(
    client_id: &str,
    tenant: &str,
    redirect_uri: &str,
    state: &str,
    scope: Vec<&str>,
) -> IdentityResult<Url> {
    OpenIdCredential::authorization_url_builder(client_id)
        .with_tenant(tenant)
        //.with_default_scope()?
        .with_redirect_uri(Url::parse(redirect_uri)?)
        .with_response_mode(ResponseMode::FormPost)
        .with_response_type([ResponseType::IdToken, ResponseType::Code])
        .with_prompt(Prompt::SelectAccount)
        .with_state(state)
        .with_scope(scope)
        .build()
        .url()
}

fn map_to_credential(
    client_id: &str,
    tenant: &str,
    redirect_uri: &str,
    state: &str,
    scope: Vec<&str>,
    client_secret: &str,
) -> IdentityResult<()> {
    let auth_url_builder = OpenIdCredential::authorization_url_builder(client_id)
        .with_tenant(tenant)
        //.with_default_scope()?
        .with_redirect_uri(Url::parse(redirect_uri)?)
        .with_response_mode(ResponseMode::FormPost)
        .with_response_type([ResponseType::IdToken, ResponseType::Code])
        .with_prompt(Prompt::SelectAccount)
        .with_state(state)
        .with_scope(scope)
        .build();

    // Open the url in a web browser, sign in, and get the authorization code
    // returned in the POST to the redirect uri.
    let _url = auth_url_builder.url().unwrap();

    // Code returned on redirect uri.
    let authorization_code = "...";

    // Use the authorization url builder to create the credential builder.
    let mut credential_builder = auth_url_builder.into_credential(authorization_code);
    let mut confidential_client = credential_builder.with_client_secret(client_secret).build();

    let _graph_client = GraphClient::from(&confidential_client);
    Ok(())
}

fn auth_url_using_confidential_client_builder(
    client_id: &str,
    tenant: &str,
    redirect_uri: &str,
    scope: Vec<&str>,
) -> IdentityResult<Url> {
    ConfidentialClientApplication::builder(client_id)
        .openid_url_builder()
        .with_tenant(tenant)
        .with_redirect_uri(Url::parse(redirect_uri)?)
        .with_scope(scope)
        .build()
        .url()
}

// Same as above
fn auth_url_using_open_id_credential(
    client_id: &str,
    tenant: &str,
    redirect_uri: &str,
    scope: Vec<&str>,
) -> IdentityResult<Url> {
    OpenIdCredential::authorization_url_builder(client_id)
        .with_tenant(tenant)
        .with_redirect_uri(Url::parse(redirect_uri)?)
        .with_scope(scope)
        .build()
        .url()
}
