use graph_rs_sdk::{
    identity::{
        web::WithInteractiveAuth, AuthorizationCodeCredential, MapCredentialBuilder, Secret,
    },
    GraphClient,
};
use url::Url;

// Requires feature=interactive_authentication

// Interactive Authentication WebView Using Wry library https://github.com/tauri-apps/wry
// See the wry documentation for platform specific installation. Linux and macOS require
// installation of platform specific dependencies. These are not included by default.

// This example executes the Authorization Code OAuth flow and handles
// sign in/redirect using WebView as well as authorization and token retrieval.

// The WebView window will load on the sign in page for Microsoft Graph
// Log in with a user and upon redirect the window will close automatically.
// The credential_builder will store the authorization code returned on the
// redirect url after logging in and then build a ConfidentialClient<AuthorizationCodeCredential>

// The ConfidentialClient<AuthorizationCodeCredential> handles authorization to get an access token
// on the first request made using the Graph client. The token is stored in an in memory cache
// and subsequent calls will use this token. If a refresh token is included, which you can get
// by requesting the offline_access scope, then the confidential client will take care of refreshing
// the token.
async fn authenticate(
    tenant_id: &str,
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
    scope: Vec<&str>,
) -> anyhow::Result<GraphClient> {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let (authorization_query_response, credential_builder) =
        AuthorizationCodeCredential::authorization_url_builder(client_id)
            .with_tenant(tenant_id)
            .with_scope(scope) // Adds offline_access as a scope which is needed to get a refresh token.
            .with_redirect_uri(Url::parse(redirect_uri)?)
            .with_interactive_auth(Secret("secret".to_string()), Default::default())
            .map_to_credential_builder()?;

    debug!("{authorization_query_response:#?}");

    let confidential_client = credential_builder.build();

    Ok(GraphClient::from(&confidential_client))
}
