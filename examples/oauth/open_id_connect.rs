use graph_oauth::identity::{
    ConfidentialClientApplication, Prompt, ResponseMode, ResponseType, TokenCredentialExecutor,
    TokenRequest,
};
use graph_oauth::oauth::{OpenIdAuthorizationUrl, OpenIdCredential};
use graph_rs_sdk::oauth::{IdToken, MsalTokenResponse, OAuthSerializer};
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

// The client id and client secret must be changed before running this example.
static CLIENT_ID: &str = "";
static CLIENT_SECRET: &str = "";
static TENANT_ID: &str = "";

fn open_id_authorization_url(client_id: &str, client_secret: &str) -> anyhow::Result<Url> {
    Ok(OpenIdCredential::authorization_url_builder()?
        .with_client_id(client_id)
        .with_response_mode(ResponseMode::FormPost)
        .with_response_type([ResponseType::IdToken, ResponseType::Code])
        .with_prompt(Prompt::SelectAccount)
        .with_default_scope()?
        .extend_scope(vec!["User.Read", "User.ReadWrite"])
        .build()
        .url()?)
}

async fn handle_redirect(mut id_token: IdToken) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    id_token.enable_pii_logging(true);
    println!("{id_token:#?}");

    let code = id_token.code.unwrap();

    let mut confidential_client = OpenIdCredential::builder()
        .with_authorization_code(id_token.code)
        .with_client_id(CLIENT_ID)
        .with_client_secret(CLIENT_SECRET)
        .with_tenant(TENANT_ID)
        .with_redirect_uri("http://localhost:8000/redirect")?
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
    let routes = warp::post()
        .and(warp::path("redirect"))
        .and(warp::body::form())
        .and_then(handle_redirect)
        .with(warp::trace::named("executor"));

    let url = open_id_authorization_url(CLIENT_ID, CLIENT_SECRET).unwrap();
    webbrowser::open(url.as_ref());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
