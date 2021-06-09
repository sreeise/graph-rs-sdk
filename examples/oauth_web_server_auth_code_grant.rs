#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate reqwest;

use warp::{
    http::{Response, StatusCode},
    Filter,
};

use from_as::*;
use graph_rs_sdk::oauth::OAuth;
use std::time::Duration;
use futures::TryStreamExt;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AccessCode {
    code: String,
}

#[tokio::main]
async fn main() {
    let query = warp::query::<AccessCode>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<AccessCode>,), std::convert::Infallible>((None,)) });

    let routes = warp::get()
        .and(warp::path("redirect"))
        .and(query)
        .map(|code_option: Option<AccessCode>| match code_option {
            Some(code) => {
                // Print out the code for debugging purposes.
                println!("{:#?}", code);

                // Set the access code and request an access token.
                // Callers should handle the Result from requesting an access token
                // in case of an error here.
                set_and_req_access_code(code);

                // Generic login page response.
                Response::builder().body(String::from("Successfully Logged In! You can close your browser."))
            },
            None => Response::builder().body(String::from("There was an issue getting the access code."))
        });

    let mut oauth = oauth_web_client();
    let mut request = oauth.build().authorization_code_grant();
    request.browser_authorization().open().unwrap();

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn oauth_web_client() -> OAuth {
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
        .authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .refresh_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .response_type("code")
        .logout_url("https://login.microsoftonline.com/common/oauth2/v2.0/logout")
        // If this is not set, the redirect_url given above will be used for the logout redirect.
        // See logout.rs for an example.
        .post_logout_redirect_uri("http://localhost:8000/redirect");
    oauth
}

pub fn set_and_req_access_code(access_code: AccessCode) {
    let mut oauth = oauth_web_client();
    // The response type is automatically set to token and the grant type is automatically
    // set to authorization_code if either of these were not previously set.
    // This is done here as an example.
    oauth.access_code(access_code.code.as_str());
    let mut request = oauth.build().authorization_code_grant();

    let access_token = request.access_token().send().unwrap();
    oauth.access_token(access_token);

    // If all went well here we can print out the OAuth config with the Access Token.
    println!("{:#?}", &oauth);

    // Save our configuration to a file so we can retrieve it from other requests.
    oauth
        .as_file("./examples/example_files/web_oauth.json")
        .unwrap();
}
