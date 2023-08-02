use graph_oauth::identity::{ResponseType, TokenCredential, TokenRequest};
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

fn open_id_credential(
    authorization_code: &str,
    client_id: &str,
    client_secret: &str,
) -> anyhow::Result<OpenIdCredential> {
    Ok(OpenIdCredential::builder()
        .with_authorization_code(authorization_code)
        .with_client_id(client_id)
        .with_client_secret(client_secret)
        .with_redirect_uri("http://localhost:8000")?
        .with_scope(vec!["offline_access", "Files.Read"]) // OpenIdCredential automatically sets the openid scope
        .build())
}

fn open_id_authorization_url(client_id: &str, client_secret: &str) -> anyhow::Result<Url> {
    Ok(OpenIdCredential::authorization_url_builder()?
        .with_client_id(client_id)
        .with_default_scope()?
        .extend_scope(vec!["Files.Read"])
        .build()
        .url()?)
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OpenIdResponse {
    pub code: String,
    pub id_token: String,
    pub session_state: String,
}

async fn handle_redirect(
    id_token: OpenIdResponse,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    println!("Received IdToken: {id_token:#?}");
    let code = id_token.code.clone();

    let mut credential = open_id_credential(code.as_ref(), CLIENT_ID, CLIENT_SECRET).unwrap();
    let mut result = credential.get_token_async().await;

    dbg!(&result);

    if let Ok(response) = result {
        if response.status().is_success() {
            let mut access_token: MsalTokenResponse = response.json().await.unwrap();
            access_token.enable_pii_logging(true);

            println!("\n{:#?}\n", access_token);
        } else {
            // See if Microsoft Graph returned an error in the Response body
            let result: reqwest::Result<serde_json::Value> = response.json().await;
            println!("{result:#?}");
        }
    }

    // Generic login page response.
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
    let routes = warp::get()
        .and(warp::path("redirect"))
        .and(warp::body::json())
        .and_then(handle_redirect);

    std::env::set_var("RUST_LOG", "trace");
    std::env::set_var("GRAPH_TEST_ENV", "true");

    let url = open_id_authorization_url(CLIENT_ID, CLIENT_SECRET).unwrap();
    webbrowser::open(url.as_ref());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

/*
fn oauth_open_id() -> OAuthSerializer {
    let mut oauth = OAuthSerializer::new();
    oauth
        .client_id(CLIENT_ID)
        .client_secret(CLIENT_SECRET)
        .authorization_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize")
        .redirect_uri("http://localhost:8000/redirect")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .refresh_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .response_type("id_token code")
        .response_mode("form_post")
        .add_scope("openid")
        .add_scope("Files.Read")
        .add_scope("Files.ReadWrite")
        .add_scope("Files.Read.All")
        .add_scope("Files.ReadWrite.All")
        .add_scope("offline_access")
        .nonce("7362CAEA-9CA5")
        .prompt("login")
        .state("12345");
    oauth
}
 */
