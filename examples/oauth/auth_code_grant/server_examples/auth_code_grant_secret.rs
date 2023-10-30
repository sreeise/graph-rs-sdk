use graph_rs_sdk::error::ErrorMessage;
use graph_rs_sdk::oauth::{
    AuthCodeAuthorizationUrlParameters, AuthorizationCodeCredential, ConfidentialClientApplication,
    Token, TokenCredentialExecutor,
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

fn get_graph_client(authorization_code: &str) -> Graph {
    let mut confidential_client = ConfidentialClientApplication::builder(CLIENT_ID)
        .with_auth_code(authorization_code)
        .with_client_secret(CLIENT_SECRET)
        .with_scope(vec![SCOPE])
        .with_redirect_uri(REDIRECT_URI)
        .unwrap()
        .build();
    Graph::from(&confidential_client)
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
            let client = get_graph_client(authorization_code.as_str());
            let result = client.users().list_user().send().await;

            match result {
                Ok(response) => {
                    println!("{response:#?}");

                    let status = response.status();
                    let body: serde_json::Value = response.json().await.unwrap();
                    println!("Status: {status:#?}");
                    println!("Body: {body:#?}");
                }
                Err(err) => {
                    println!("{err:#?}");
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
