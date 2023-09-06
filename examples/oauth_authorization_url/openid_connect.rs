use graph_oauth::identity::OpenIdCredential;
use url::Url;

// The authorization request is the initial request to sign in where the user
// is taken to the sign in page and enters their credentials.
// If successful the user will be redirected back to your app and the authorization
// code will be in the query of the URL.

// The URL builder below will create the full URL with the query that you will
// need to send the user to such as redirecting the page they are on when using
// your app to the URL.

// See examples/oauth/openid_connect for a full example.

fn open_id_authorization_url(
    client_id: &str,
    tenant: &str,
    redirect_uri: &str,
    scope: Vec<&str>,
) -> anyhow::Result<Url> {
    Ok(OpenIdCredential::authorization_url_builder()?
        .with_client_id(client_id)
        .with_tenant(tenant)
        .with_redirect_uri(redirect_uri)?
        .extend_scope(scope)
        .build()
        .url()?)
}