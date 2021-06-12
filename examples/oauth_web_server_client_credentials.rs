#[macro_use]
extern crate serde;
extern crate reqwest;
extern crate serde_json;

use graph_rs_sdk::oauth::OAuth;
use warp::{http::Response, Filter};

// The client_id and client_secret must be changed before running this example.
static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";

// Client Credentials Grant
// If you have already given admin consent to a user you can skip
// browser authorization step and go strait to requesting an access token.

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ClientCredentialsResponse {
    admin_consent: bool,
    tenant: String,
}

#[tokio::main]
async fn main() {
    // If this is not the first time you are using the client credentials grant
    // then you only have to run request_access_token() and you can comment out
    // what is below.
    let query = warp::query::<ClientCredentialsResponse>()
        .map(Some)
        .or_else(|_| async {
            Ok::<(Option<ClientCredentialsResponse>,), std::convert::Infallible>((None,))
        });

    let routes = warp::get().and(warp::path("redirect")).and(query).map(
        |cc: Option<ClientCredentialsResponse>| match cc {
            Some(code) => {
                // Print out for debugging purposes.
                println!("{:#?}", code);

                // Request an access token.
                request_access_token();

                // Generic login page response.
                Response::builder().body(String::from(
                    "Successfully Logged In! You can close your browser.",
                ))
            }
            None => Response::builder()
                .body(String::from("There was an issue getting the access code.")),
        },
    );

    // Get the oauth client and request a browser sign in
    let mut oauth = get_oauth_client();
    let mut request = oauth.build().code_flow();
    request.browser_authorization().open().unwrap();

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
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

fn request_access_token() {
    let mut oauth = get_oauth_client();
    let mut request = oauth.build().client_credentials();
    let access_token = request.access_token().send().unwrap();

    println!("{:#?}", access_token);
    oauth.access_token(access_token);
}
