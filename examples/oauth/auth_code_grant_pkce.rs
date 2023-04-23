use graph_rs_sdk::error::AuthorizationResult;
use graph_rs_sdk::oauth::{
    AccessToken, AuthCodeAuthorizationUrl, AuthorizationCodeCredential,
    ConfidentialClientApplication, ProofKeyForCodeExchange, TokenRequest,
};
use lazy_static::lazy_static;
use warp::{get, Filter};

static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";

// You can also pass your own values for PKCE instead of automatic generation by
// calling ProofKeyCodeExchange::new(code_verifier, code_challenge, code_challenge_method)
lazy_static! {
    static ref PKCE: ProofKeyForCodeExchange = ProofKeyForCodeExchange::generate().unwrap();
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AccessCode {
    code: String,
}

// This example shows how to use a code_challenge and code_verifier
// to perform the authorization code grant flow with proof key for
// code exchange (PKCE).
//
// For more info see: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow
// And the PKCE RFC: https://tools.ietf.org/html/rfc7636

// Open the default system web browser to the sign in url for authorization.
// This method uses AuthorizationCodeAuthorizationUrl to build the sign in
// url and query needed to get an authorization code and opens the default system
// web browser to this Url.
fn authorization_sign_in() {
    let auth_code_url_builder = AuthCodeAuthorizationUrl::builder()
        .with_client_id(CLIENT_ID)
        .with_scope(vec!["user.read"])
        .with_redirect_uri("http://localhost:8000/redirect")
        .with_proof_key_for_code_exchange(&PKCE)
        .build();

    let url = auth_code_url_builder.url().unwrap();
    webbrowser::open(url.as_str()).unwrap();
}

/// Build the Authorization Code Grant Credential.
fn get_confidential_client_application(authorization_code: &str) -> ConfidentialClientApplication {
    let credential = AuthorizationCodeCredential::builder()
        .with_authorization_code(authorization_code)
        .with_client_id(CLIENT_ID)
        .with_client_secret(CLIENT_SECRET)
        .with_redirect_uri("http://localhost:8000/redirect")
        .with_proof_key_for_code_exchange(&PKCE)
        .build();

    ConfidentialClientApplication::from(credential)
}

// When the authorization code comes in on the redirect from sign in, call the get_credential
// method passing in the authorization code. The AuthorizationCodeCredential can be passed
// to a confidential client application in order to exchange the authorization code
// for an access token.
async fn handle_redirect(
    code_option: Option<AccessCode>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    match code_option {
        Some(access_code) => {
            // Print out the code for debugging purposes.
            println!("{:#?}", access_code.code);

            let mut confidential_client =
                get_confidential_client_application(access_code.code.as_str());

            // Returns reqwest::Response
            let response = confidential_client.get_token_async().await.unwrap();
            println!("{response:#?}");

            if response.status().is_success() {
                let access_token: AccessToken = response.json().await.unwrap();

                // If all went well here we can print out the OAuth config with the Access Token.
                println!("AccessToken: {:#?}", access_token.bearer_token());
            } else {
                // See if Microsoft Graph returned an error in the Response body
                let result: reqwest::Result<serde_json::Value> = response.json().await;
                println!("{result:#?}");
                return Ok(Box::new("Error Logging In! You can close your browser."));
            }

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

    authorization_sign_in();

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
