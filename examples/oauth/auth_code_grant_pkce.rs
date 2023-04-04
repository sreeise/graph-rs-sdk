use graph_rs_sdk::oauth::OAuth;
use lazy_static::lazy_static;
/// # Example
/// ```
/// use graph_rs_sdk::*:
///
/// #[tokio::main]
/// async fn main() {
///   start_server_main().await;
/// }
/// ```
use warp::Filter;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AccessCode {
    code: String,
}

static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";

lazy_static! {
    static ref OAUTH_CLIENT: OAuthClient = OAuthClient::new(CLIENT_ID, CLIENT_SECRET);
}

// This example shows how to use a code_challenge and code_verifier
// to perform the authorization code grant flow with proof key for
// code exchange (PKCE).
//
// For more info see: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow
// And the PKCE RFC: https://tools.ietf.org/html/rfc7636

// Store and initialize OAuth within another struct so that we can
// use it in lazy_static since OAuth requires being mutable to
// change its fields.
// Although probably not suitable for production use for this example
// we will just clone the internal oauth each time we need it.
// We use lazy static to ensure the code verifier and code challenge
// stays the same between requests.
struct OAuthClient {
    client: OAuth,
}

impl OAuthClient {
    pub fn new(client_id: &str, client_secret: &str) -> OAuthClient {
        let mut oauth = OAuth::new();
        oauth
            .client_id(client_id)
            .client_secret(client_secret)
            .add_scope("user.read")
            .add_scope("user.readwrite")
            .redirect_uri("http://localhost:8000/redirect")
            .authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize")
            .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
            .refresh_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
            .response_type("code");

        // Generate the code challenge and code verifier.
        oauth.generate_sha256_challenge_and_verifier().unwrap();

        OAuthClient { client: oauth }
    }

    pub fn oauth(&self) -> OAuth {
        self.client.clone()
    }
}

async fn handle_redirect(
    code_option: Option<AccessCode>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    match code_option {
        Some(access_code) => {
            // Print out the code for debugging purposes.
            println!("{:#?}", access_code.code);

            // Set the access code and request an access token.
            // Callers should handle the Result from requesting an access token
            // in case of an error here.
            let mut oauth = OAUTH_CLIENT.oauth();

            oauth.access_code(access_code.code.as_str());
            let mut request = oauth.build_async().authorization_code_grant();

            let access_token = request.access_token().send().await.unwrap();
            oauth.access_token(access_token);
            println!("{oauth:#?}");

            // Generic login page response.
            Ok(Box::new(
                "Successfully Logged In! You can close your browser.",
            ))
        }
        None => Err(warp::reject()),
    }
}

/// # Example
/// ```
/// use graph_rs_sdk::*:
///
/// #[tokio::main]
/// async fn main() {
///   start_server_main().await;
/// }
/// ```
pub async fn start_server_main() {
    let query = warp::query::<AccessCode>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<AccessCode>,), std::convert::Infallible>((None,)) });

    let routes = warp::get()
        .and(warp::path("redirect"))
        .and(query)
        .and_then(handle_redirect);

    let mut oauth = OAUTH_CLIENT.oauth();
    let mut request = oauth.build_async().authorization_code_grant();
    request.browser_authorization().open().unwrap();

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
