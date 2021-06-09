#[macro_use]
extern crate serde;
extern crate reqwest;
extern crate serde_json;

use warp::{http::Response, Filter};

use from_as::*;
use graph_rs_sdk::oauth::OAuth;

// The client_id and client_secret must be changed before running this example.
static CLIENT_ID: &str = "<YOUR_CLIENT_ID>";
static CLIENT_SECRET: &str = "<YOUR_CLIENT_SECRET>";

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AccessCode {
    code: String,
    state: String,
}

// This example shows using Rocket to authenticate with Microsoft OneDrive that
// includes authorization with a state parameter in the request query.
//
// This example uses the code flow: https://docs.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online
//
// If you have not set up an application to call the Graph API for OneDrive
// API then you will want to first read through the information in rocket_example.rs
// before moving forward. A client id and client secret are needed to
// call the overdrive API in order to authenticate users. This is done
// through the Microsoft application portal or through Azure. Creating an
// application will create an application ID which is the client id. Then,
// under application secrets, a new password will need to be generated. This
// password is the client secret. The rocket_example.rs file has more information
// on how to set up an application.

// Create an OAuth struct with the needed credentials.
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
        .state("13534298")
        .logout_url("https://login.live.com/oauth20_logout.srf?")
        // If this is not set, the redirect_url given above will be used for the logout redirect.
        // See logout.rs for an example.
        .post_logout_redirect_uri("http://localhost:8000/redirect");
    oauth
}

#[tokio::main]
async fn main() {
    let query = warp::query::<AccessCode>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<AccessCode>,), std::convert::Infallible>((None,)) });

    let routes = warp::get().and(warp::path("redirect")).and(query).map(
        |code_option: Option<AccessCode>| match code_option {
            Some(access_code) => {
                // Print out the code for debugging purposes.
                println!("{:#?}", access_code);

                // Assert that the state is the same as the one given in the original request.
                assert_eq!("13534298", access_code.state.as_str());

                // Set the access code and request an access token.
                // Callers should handle the Result from requesting an access token
                // in case of an error here.
                set_and_req_access_code(access_code);

                // Generic login page response.
                Response::builder().body(String::from(
                    "Successfully Logged In! You can close your browser.",
                ))
            },
            None => Response::builder()
                .body(String::from("There was an issue getting the access code.")),
        },
    );

    let mut oauth = oauth_web_client();
    let mut request = oauth.build().code_flow();
    request.browser_authorization().open().unwrap();

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

pub fn set_and_req_access_code(access_code: AccessCode) {
    let mut oauth = oauth_web_client();
    oauth.response_type("token");
    oauth.state(access_code.code.as_str());
    oauth.access_code(access_code.code.as_str());

    // Request the access token.
    let mut request = oauth.build().code_flow();
    let access_token = request.access_token().send().unwrap();
    oauth.access_token(access_token);

    // If all went well here we can print out the OAuth config with the Access Token.
    println!("{:#?}", &oauth);

    // Save our configuration to a file so we can retrieve it for other requests.
    oauth
        .as_file("./examples/example_files/web_oauth.json")
        .unwrap();
}
