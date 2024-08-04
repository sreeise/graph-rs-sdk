use graph_rs_sdk::identity::{
    ConfidentialClientApplication, IdToken, OpenIdCredential, Prompt, ResponseMode, ResponseType,
    Token, TokenCredentialExecutor,
};
use url::Url;

use graph_rs_sdk::GraphClient;
/// # Example
/// ```
/// use graph_rs_sdk::oauth::{AccessToken, IdToken, OAuth};
///
/// #[tokio::main]
/// async fn main() {
///   start_server_main().await;
/// }
/// ```
///
/// [Microsoft Open ID Connect](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc)
/// OpenID Connect (OIDC) extends the OAuth 2.0 authorization protocol for use also as an
/// authentication protocol. You can use OIDC to enable single sign-on (SSO) between your
/// OAuth-enabled applications by using a security token called an ID token.
use warp::Filter;

// The client id and client secret must be changed before running this example.
static CLIENT_ID: &str = "";
static CLIENT_SECRET: &str = "";
static TENANT_ID: &str = "";

static REDIRECT_URI: &str = "http://localhost:8000/redirect";

// Use the form post response mode when listening on a server instead
// of the URL query because the the query does not get sent to servers.
fn openid_authorization_url() -> anyhow::Result<Url> {
    Ok(OpenIdCredential::authorization_url_builder(CLIENT_ID)
        .with_tenant(TENANT_ID)
        .with_redirect_uri(Url::parse(REDIRECT_URI).unwrap())
        .with_response_mode(ResponseMode::FormPost)
        .with_response_type([ResponseType::IdToken, ResponseType::Code])
        .with_prompt(Prompt::SelectAccount)
        .with_state("1234")
        .with_scope(vec!["User.Read", "User.ReadWrite"])
        .build()
        .url()?)
}

async fn list_users(confidential_client: &ConfidentialClientApplication<OpenIdCredential>) {
    let graph_client = GraphClient::from(confidential_client);

    let response = graph_client.users().list_user().send().await.unwrap();

    debug!("{response:#?}");

    let users: serde_json::Value = response.json().await.unwrap();
    debug!("{:#?}", users);
}

async fn handle_redirect(mut id_token: IdToken) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    id_token.enable_pii_logging(true);
    debug!("{id_token:#?}");

    let code = id_token.code.unwrap();

    let mut confidential_client = ConfidentialClientApplication::builder(CLIENT_ID)
        .with_openid(code, CLIENT_SECRET)
        .with_tenant(TENANT_ID)
        .with_redirect_uri(Url::parse(REDIRECT_URI).unwrap())
        .with_scope(vec!["User.Read", "User.ReadWrite"]) // OpenIdCredential automatically sets the openid scope
        .build();

    list_users(&confidential_client);

    Ok(Box::new(
        "Successfully Logged In! You can close your browser.",
    ))
}

/// # Example
/// ```
/// use graph_rs_sdk::oauth::{AccessToken, IdToken, OAuth};
///
/// #[tokio::main]
/// async fn main() {
///   start_server_main().await;
/// }
/// ```
pub async fn start_server_main() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let routes = warp::post()
        .and(warp::path("redirect"))
        .and(warp::body::form())
        .and_then(handle_redirect)
        .with(warp::trace::named("executor"));

    let url = openid_authorization_url().unwrap();
    webbrowser::open(url.as_ref());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
