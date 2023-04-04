/// # Example
/// ```
/// use graph_rs_sdk::*:
///
/// #[tokio::main]
/// async fn main() {
///   start_server_main().await;
/// }
/// ```
///
/// # Overview:
///
/// [Microsoft Client Credentials](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow)
/// You can use the OAuth 2.0 client credentials grant specified in RFC 6749,
/// sometimes called two-legged OAuth, to access web-hosted resources by using the
/// identity of an application. This type of grant is commonly used for server-to-server
/// interactions that must run in the background, without immediate interaction with a user.
/// These types of applications are often referred to as daemons or service accounts.
///
/// This OAuth flow example requires signing in as an administrator for Azure, known as admin consent,
/// to approve your application to call Microsoft Graph Apis on behalf of a user. Admin consent
/// only has to be done once for a user. After admin consent is given, the oauth client can be
/// used to continue getting new access tokens programmatically.
use graph_rs_sdk::oauth::OAuth;
use warp::Filter;

// The client_id and client_secret must be changed before running this example.
static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ClientCredentialsResponse {
    admin_consent: bool,
    tenant: String,
}

fn get_oauth_client() -> OAuth {
    let mut oauth = OAuth::new();
    oauth
        .client_id(CLIENT_ID)
        .client_secret(CLIENT_SECRET)
        .add_scope("https://graph.microsoft.com/.default")
        .redirect_uri("http://localhost:8000/redirect")
        .authorize_url("https://login.microsoftonline.com/common/adminconsent")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token");
    oauth
}

async fn request_access_token() {
    let mut oauth = get_oauth_client();
    let mut request = oauth.build_async().client_credentials();
    let access_token = request.access_token().send().await.unwrap();

    println!("{access_token:#?}");
    oauth.access_token(access_token);
}

async fn handle_redirect(
    client_credential_option: Option<ClientCredentialsResponse>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    match client_credential_option {
        Some(client_credential_response) => {
            // Print out for debugging purposes.
            println!("{client_credential_response:#?}");

            // Request an access token.
            request_access_token().await;

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
    let query = warp::query::<ClientCredentialsResponse>()
        .map(Some)
        .or_else(|_| async {
            Ok::<(Option<ClientCredentialsResponse>,), std::convert::Infallible>((None,))
        });

    let routes = warp::get()
        .and(warp::path("redirect"))
        .and(query)
        .and_then(handle_redirect);

    // Get the oauth client and request a browser sign in
    let mut oauth = get_oauth_client();
    let mut request = oauth.build_async().client_credentials();
    request.browser_authorization().open().unwrap();

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
