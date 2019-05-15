use rust_onedrive::oauth::{Grant, OAuth};
use transform_request::prelude::*;

fn web_client() -> OAuth {
    let mut oauth = OAuth::code_flow();
    oauth
        .client_id("<CLIENT_ID>")
        .client_secret("<CLIENT_SECRET>")
        // Or whatever you set the redirect to for a web client.
        .redirect_uri("http://localhost:8000/redirect")
        .authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token?");

    oauth
}

fn main() {
    let mut oauth = web_client();
    // request_authorization() opens users default browser to the authentication page
    // Run this first, then get the code from the browser URL. Afterward,
    // comment out the the request_authorization() method below and run
    // set_code_request_token(your_access_token).
    oauth.request_authorization().unwrap();
}

#[allow(dead_code)]
fn set_code_request_token(access_code: &str) {
    let mut oauth = web_client();

    // Set the access code that will be used to request an
    // access token and/or refresh token.
    oauth.access_code(access_code);

    // Makes the POST request for an access token/refresh token.
    // This is stored in the struct AccessToken.
    match oauth.request_access_token() {
        Ok(_) => {
            println!("The request was successful\n");
            println!("{:#?}", &oauth);

            /*
            Stores OAuth as json using serde_json.

            To get OAuth back from the json file run:
            let mut oauth: OAuth = OAuth::from_file("example/oauth.json").unwrap();
            */
            oauth.to_file("example/web_client_flow.json").unwrap();
        },
        Err(e) => println!("There was an error: {:#?}", e),
    }
}
