use graph_oauth::oauth::AccessToken;
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
/// # Setup:
/// This example shows using the OneDrive and SharePoint code flow: https://learn.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online
/// Includes authorization with a state parameter in the request query. The state parameter is optional.
///
/// You will first need to head to the Microsoft Application Portal and create and
/// application. Once the application is created you will need to specify the
/// scopes you need and change them accordingly in the oauth_web_client() method
/// when adding scopes using OAuth::add_scope("scope").
///
/// For reference the Microsoft Graph Authorization V2 required parameters along with
/// the methods to use needed to be set are shown in the oauth_web_client() method.
///
/// Once an application is registered in Azure you will be given an application id which is the client id in an OAuth2 request.
/// For this example, a client secret will need to be generated. The client secret is the same as the password
/// under Application Secrets int the registration portal. If you do not have a client secret then click on
/// 'Generate New Password'.  Next click on 'Add Platform' and create a new web platform.
/// Add a redirect url to the platform. In the example below, the redirect url is http://localhost:8000/redirect
/// but anything can be used.
///
/// # Sign In Flow:
///
/// After signing in, you will be redirected, and the access code that is given in the redirect
/// will be used to automatically call the access token endpoint and receive an access token
/// and/or refresh token.
///
/// Disclaimer/Important Info:
///
/// This example is meant for testing and is not meant to be production ready or complete.
use graph_rs_sdk::oauth::OAuth;
use warp::Filter;

// The client_id and client_secret must be changed before running this example.
static CLIENT_ID: &str = "<YOUR_CLIENT_ID>";
static CLIENT_SECRET: &str = "<YOUR_CLIENT_SECRET>";

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AccessCode {
    code: String,
    state: String,
}

// Create OAuth client and set credentials.
fn oauth_web_client() -> OAuth {
    let mut oauth = OAuth::new();
    oauth
        .client_id(CLIENT_ID)
        .client_secret(CLIENT_SECRET)
        .add_scope("Files.Read")
        .add_scope("Files.ReadWrite")
        .add_scope("Files.Read.All")
        .add_scope("Files.ReadWrite.All")
        .add_scope("wl.offline_access")
        .redirect_uri("http://localhost:8000/redirect")
        .authorize_url("https://login.live.com/oauth20_authorize.srf?")
        .access_token_url("https://login.live.com/oauth20_token.srf")
        .refresh_token_url("https://login.live.com/oauth20_token.srf")
        .response_mode("query")
        .state("13534298") // Optional
        .logout_url("https://login.live.com/oauth20_logout.srf?") // Optional
        // The redirect_url given above will be used for the logout redirect if none is provided.
        .post_logout_redirect_uri("http://localhost:8000/redirect"); // Optional
    oauth
}

pub async fn set_and_req_access_code(access_code: AccessCode) {
    let mut oauth = oauth_web_client();
    oauth.response_type("token");
    oauth.state(access_code.state.as_str());
    oauth.access_code(access_code.code.as_str());

    // Request the access token.
    let mut client = oauth.build_async().code_flow();

    let response = client.access_token().send().await.unwrap();
    println!("{response:#?}");

    if response.status().is_success() {
        let access_token: AccessToken = response.json().await.unwrap();

        println!("{access_token:#?}");
        oauth.access_token(access_token);
    } else {
        // See if Microsoft Graph returned an error in the Response body
        let result: reqwest::Result<serde_json::Value> = response.json().await;
        println!("{result:#?}");
    }

    // If all went well here we can print out the OAuth config with the Access Token.
    println!("{:#?}", &oauth);
}

async fn handle_redirect(
    code_option: Option<AccessCode>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    match code_option {
        Some(access_code) => {
            // Print out the code for debugging purposes.
            println!("{access_code:#?}");

            // Assert that the state is the same as the one given in the original request.
            assert_eq!("13534298", access_code.state.as_str());

            // Set the access code and request an access token.
            // Callers should handle the Result from requesting an access token
            // in case of an error here.
            set_and_req_access_code(access_code).await;

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

    let mut oauth = oauth_web_client();
    let mut request = oauth.build_async().code_flow();
    request.browser_authorization().open().unwrap();

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
