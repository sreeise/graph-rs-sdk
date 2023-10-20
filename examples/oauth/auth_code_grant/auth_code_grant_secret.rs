use graph_rs_sdk::error::ErrorMessage;
use graph_rs_sdk::oauth::{
    AuthCodeAuthorizationUrlParameters, AuthorizationCodeCredential, ConfidentialClientApplication,
    Token, TokenCredentialExecutor, TokenRequest,
};
use graph_rs_sdk::*;
use warp::Filter;

// Update these values with your own or provide them directly in the
// methods below.
static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";
static REDIRECT_URI: &str = "http://localhost:8000/redirect";
static SCOPE: &str = "User.Read";

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AccessCode {
    code: String,
}

pub fn authorization_sign_in() {
    let url = AuthorizationCodeCredential::authorization_url_builder(CLIENT_ID)
        .with_redirect_uri(REDIRECT_URI)
        .with_scope(vec![SCOPE])
        .url()
        .unwrap();

    // web browser crate in dev dependencies will open to default browser in the system.
    webbrowser::open(url.as_str()).unwrap();
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

    authorization_sign_in();

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

/// # Use the access code to build Confidential Client Application
///
/// ```rust
/// use graph_rs_sdk::oauth::ConfidentialClientApplication;
///
/// // Set the access code and request an access token.
/// // Callers should handle the Result from requesting an access token
/// // in case of an error here.
/// let client_app = ConfidentialClientApplication::builder("client-id")
///     .with_authorization_code("code")
///     .with_client_secret("client-secret")
///     .with_scope(vec!["User.Read"])
///     .build();
/// ```
async fn handle_redirect(
    code_option: Option<AccessCode>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    match code_option {
        Some(access_code) => {
            // Print out the code for debugging purposes.
            println!("{access_code:#?}");

            let authorization_code = access_code.code;

            // Set the access code and request an access token.
            // Callers should handle the Result from requesting an access token
            // in case of an error here.
            let mut confidential_client = ConfidentialClientApplication::builder(CLIENT_ID)
                .with_authorization_code(authorization_code)
                .with_client_secret(CLIENT_SECRET)
                .with_scope(vec![SCOPE])
                .with_redirect_uri(REDIRECT_URI)
                .unwrap()
                .build();

            let response = confidential_client.execute_async().await.unwrap();
            println!("{response:#?}");

            if response.status().is_success() {
                let mut access_token: Token = response.json().await.unwrap();

                // Enables the printing of the bearer, refresh, and id token.
                access_token.enable_pii_logging(true);
                println!("{:#?}", access_token);

                // This will print the actual access token to the console.
            } else {
                // See if Microsoft Graph returned an error in the Response body
                let result: reqwest::Result<ErrorMessage> = response.json().await;

                match result {
                    Ok(error_message) => println!("{error_message:#?}"),
                    Err(err) => println!("Error on deserialization:\n{err:#?}"),
                }
            }

            // Generic login page response.
            Ok(Box::new(
                "Successfully Logged In! You can close your browser.",
            ))
        }
        None => Err(warp::reject()),
    }
}
