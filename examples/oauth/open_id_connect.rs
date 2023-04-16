use graph_rs_sdk::oauth::{AccessToken, IdToken, OAuth};
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
static CLIENT_ID: &str = "<YOUR_CLIENT_ID>";
static CLIENT_SECRET: &str = "<YOUR_CLIENT_SECRET>";

fn oauth_open_id() -> OAuth {
    let mut oauth = OAuth::new();
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

async fn handle_redirect(id_token: IdToken) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    println!("Received IdToken: {id_token:#?}");

    let mut oauth = oauth_open_id();

    // Pass the id token to the oauth client.
    oauth.id_token(id_token);

    // Build the request to get an access token using open id connect.
    let mut request = oauth.build_async().open_id_connect();

    // Request an access token.
    let response = request.access_token().send().await.unwrap();
    println!("{response:#?}");

    if response.status().is_success() {
        let access_token: AccessToken = response.json().await.unwrap();

        // You can optionally pass the access token to the oauth client in order
        // to use a refresh token to get more access tokens. The refresh token
        // is stored in AccessToken.
        oauth.access_token(access_token);

        // If all went well here we can print out the OAuth config with the Access Token.
        println!("OAuth:\n{:#?}\n", &oauth);
    } else {
        // See if Microsoft Graph returned an error in the Response body
        let result: reqwest::Result<serde_json::Value> = response.json().await;
        println!("{result:#?}");
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

    // Get the oauth client and request a browser sign in.
    let mut oauth = oauth_open_id();
    let mut request = oauth.build_async().open_id_connect();
    request.browser_authorization().open().unwrap();

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
