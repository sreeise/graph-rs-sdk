use graph_rs_sdk::oauth::{
    AccessToken, AuthCodeAuthorizationUrl, AuthorizationCodeCredential,
    ConfidentialClientApplication, TokenRequest,
};
use graph_rs_sdk::*;
use warp::Filter;

static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AccessCode {
    code: String,
}

pub fn authorization_sign_in() {
    let url = AuthorizationCodeCredential::authorization_url_builder()
        .with_client_id(CLIENT_ID)
        .with_redirect_uri("http://localhost:8000/redirect")
        .with_scope(vec!["offline_access", "files.read"])
        .url()
        .unwrap();

    // web browser crate in dev dependencies will open to default browser in the system.
    webbrowser::open(url.as_str()).unwrap();
}

pub fn get_confidential_client(authorization_code: &str) -> ConfidentialClientApplication {
    let auth_code_credential = AuthorizationCodeCredential::builder()
        .with_authorization_code(authorization_code)
        .with_client_id(CLIENT_ID)
        .with_client_secret(CLIENT_SECRET)
        .with_scope(vec!["files.read", "offline_access"])
        .with_redirect_uri("http://localhost:8000/redirect")
        .build();

    ConfidentialClientApplication::from(auth_code_credential)
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
            let mut confidential_client_application =
                get_confidential_client(access_code.code.as_str());

            let response = confidential_client_application
                .get_token_async()
                .await
                .unwrap();
            println!("{response:#?}");

            if response.status().is_success() {
                let mut access_token: AccessToken = response.json().await.unwrap();

                // Option<&JsonWebToken>
                let jwt = access_token.jwt();
                println!("{jwt:#?}");

                println!("{:#?}", access_token);

                // This will print the actual access token to the console.
                println!("Access Token: {:#?}", access_token.bearer_token());
                println!("Refresh Token: {:#?}", access_token.refresh_token());
            } else {
                // See if Microsoft Graph returned an error in the Response body
                let result: reqwest::Result<serde_json::Value> = response.json().await;

                match result {
                    Ok(body) => println!("{body:#?}"),
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
