use graph_rs_sdk::oauth::{AccessToken, OAuth};
/// # Example
/// ```
/// use graph_rs_sdk::*:
///
/// #[tokio::main]
/// async fn main() {
///   start_server_main().await;
/// }
/// ```
use graph_rs_sdk::*;
use warp::Filter;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AccessCode {
    code: String,
}

fn oauth_client() -> OAuth {
    let mut oauth = OAuth::new();
    oauth
        .client_id("<YOUR_CLIENT_ID>")
        .client_secret("<YOUR_CLIENT_SECRET>")
        .add_scope("files.read")
        .add_scope("files.readwrite")
        .add_scope("files.read.all")
        .add_scope("files.readwrite.all")
        .add_scope("offline_access")
        .redirect_uri("http://localhost:8000/redirect")
        .authorization_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .refresh_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .response_type("code");
    oauth
}

pub async fn set_and_req_access_code(access_code: AccessCode) -> GraphResult<()> {
    let mut oauth = oauth_client();
    // The response type is automatically set to token and the grant type is automatically
    // set to authorization_code if either of these were not previously set.
    // This is done here as an example.
    oauth.authorization_code(access_code.code.as_str());
    let mut request = oauth.build_async().authorization_code_grant();

    // Returns reqwest::Response
    let response = request.access_token().send().await?;
    println!("{response:#?}");

    if response.status().is_success() {
        let mut access_token: AccessToken = response.json().await?;

        // Option<&JsonWebToken>
        let jwt = access_token.jwt();
        println!("{jwt:#?}");

        // Store in OAuth to make requests for refresh tokens.
        oauth.access_token(access_token);

        // If all went well here we can print out the OAuth config with the Access Token.
        println!("{:#?}", &oauth);
    } else {
        // See if Microsoft Graph returned an error in the Response body
        let result: reqwest::Result<serde_json::Value> = response.json().await;

        match result {
            Ok(body) => println!("{body:#?}"),
            Err(err) => println!("Error on deserialization:\n{err:#?}"),
        }
    }

    Ok(())
}

async fn handle_redirect(
    code_option: Option<AccessCode>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    match code_option {
        Some(access_code) => {
            // Print out the code for debugging purposes.
            println!("{access_code:#?}");

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

    let mut oauth = oauth_client();
    let mut request = oauth.build_async().authorization_code_grant();
    request.browser_authorization().open().unwrap();

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
