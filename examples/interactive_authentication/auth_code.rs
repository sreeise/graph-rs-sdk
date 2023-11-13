use graph_rs_sdk::{oauth::AuthorizationCodeCredential, GraphClient};

static CLIENT_ID: &str = "CLIENT_ID";
static CLIENT_SECRET: &str = "CLIENT_SECRET";
static TENANT_ID: &str = "TENANT_ID";

// This should be the user id for the user you are logging in as.
static USER_ID: &str = "USER_ID";

static REDIRECT_URI: &str = "http://localhost:8000/redirect";

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
async fn authenticate() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let (authorization_query_response, mut credential_builder) =
        AuthorizationCodeCredential::authorization_url_builder(CLIENT_ID)
            .with_tenant(TENANT_ID)
            .with_scope(vec!["user.read", "offline_access"]) // Adds offline_access as a scope which is needed to get a refresh token.
            .with_redirect_uri(REDIRECT_URI)
            .with_interactive_authentication(None)
            .unwrap();

    debug!("{authorization_query_response:#?}");

    let mut confidential_client = credential_builder.with_client_secret(CLIENT_SECRET).build();

    let client = GraphClient::from(&confidential_client);

    let response = client.user(USER_ID).get_user().send().await.unwrap();

    debug!("{response:#?}");
    let body: serde_json::Value = response.json().await.unwrap();
    debug!("{body:#?}");
}
