use graph_oauth::identity::{
    ConfidentialClientApplication, Prompt, ResponseMode, ResponseType, TokenCredentialExecutor,
    TokenRequest,
};
use graph_oauth::oauth::{OpenIdAuthorizationUrl, OpenIdCredential};
use graph_rs_sdk::oauth::{IdToken, MsalTokenResponse, OAuthSerializer};
use tracing_subscriber::fmt::format::FmtSpan;
use url::Url;

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

// Use the form post form post response mode when listening on a server instead
// of the URL query because the the query does not get sent to servers.

// The client id and client secret must be changed before running this example.
static CLIENT_ID: &str = "";
static CLIENT_SECRET: &str = "";
static TENANT_ID: &str = "";

static REDIRECT_URI: &str = "http://localhost:8000/redirect";

fn openid_authorization_url() -> anyhow::Result<Url> {
    Ok(OpenIdCredential::authorization_url_builder()?
        .with_client_id(CLIENT_ID)
        .with_tenant(TENANT_ID)
        //.with_default_scope()?
        .with_redirect_uri(REDIRECT_URI)?
        .with_response_mode(ResponseMode::FormPost)
        .with_response_type([ResponseType::IdToken, ResponseType::Code])
        .with_prompt(Prompt::SelectAccount)
        .with_state("1234")
        .extend_scope(vec!["User.Read", "User.ReadWrite"])
        .build()
        .url()?)
}

async fn handle_redirect(mut id_token: IdToken) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    id_token.enable_pii_logging(true);
    println!("{id_token:#?}");

    let code = id_token.code.unwrap();

    let mut confidential_client = ConfidentialClientApplication::builder(CLIENT_ID)
        .with_openid(code, CLIENT_SECRET)
        .with_tenant(TENANT_ID)
        .with_redirect_uri(REDIRECT_URI)
        .unwrap()
        .with_scope(vec!["User.Read", "User.ReadWrite"]) // OpenIdCredential automatically sets the openid scope
        .build();

    let mut response = confidential_client.execute_async().await.unwrap();

    if response.status().is_success() {
        let mut access_token: MsalTokenResponse = response.json().await.unwrap();
        access_token.enable_pii_logging(true);

        println!("\n{:#?}\n", access_token);
    } else {
        // See if Microsoft Graph returned an error in the Response body
        let result: reqwest::Result<serde_json::Value> = response.json().await;
        println!("{result:#?}");
    }

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
    let filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=debug,warp=debug".to_owned());

    // Configure the default `tracing` subscriber.
    // The `fmt` subscriber from the `tracing-subscriber` crate logs `tracing`
    // events to stdout. Other subscribers are available for integrating with
    // distributed tracing systems such as OpenTelemetry.
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::FULL)
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let routes = warp::post()
        .and(warp::path("redirect"))
        .and(warp::body::form())
        .and_then(handle_redirect)
        .with(warp::trace::named("executor"));

    let url = openid_authorization_url().unwrap();
    webbrowser::open(url.as_ref());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
