#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
#[macro_use]
extern crate rocket;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
extern crate reqwest;
#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::oauth::OAuth;
use rocket::http::RawStr;
use rocket_codegen::routes;
use std::{thread, time::Duration};

/*
This example shows how to use a code_challenge and code_verifier
to perform the authorization code grant flow with proof key for
code exchange (PKCE).

For more info see: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow
And the PKCE RFC: https://tools.ietf.org/html/rfc7636
*/

// Store and initialize OAuth within another struct so that we can
// use it in lazy_static since OAuth requires being mutable to
// change its fields.
// Although probably not suitable for production use for this example
// we will just clone the internal oauth each time we need it.
// We use lazy static to ensure the code verifier and code challenge
// stays the same between requests.
struct OAuthClient {
    client: OAuth,
}

impl OAuthClient {
    pub fn new(client_id: &str, client_secret: &str) -> OAuthClient {
        let mut oauth = OAuth::new();
        oauth
            .client_id(client_id)
            .client_secret(client_secret)
            .add_scope("user.read")
            .add_scope("user.readwrite")
            .redirect_uri("http://localhost:8000/redirect")
            .authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize")
            .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
            .refresh_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
            .response_type("code");

        // Generate the code challenge and code verifier.
        oauth.generate_sha256_challenge_and_verifier().unwrap();

        OAuthClient { client: oauth }
    }

    pub fn oauth(&self) -> OAuth {
        self.client.clone()
    }
}

static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";

lazy_static! {
    static ref OAUTH_CLIENT: OAuthClient = OAuthClient::new(CLIENT_ID, CLIENT_SECRET);
}

fn main() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_secs(2));
        let mut oauth = OAUTH_CLIENT.oauth();
        let mut request = oauth.build().authorization_code_grant();
        request.browser_authorization().open().unwrap();
    });

    rocket::ignite().mount("/", routes![redirect]).launch();
    handle.join().unwrap();
}

#[get("/redirect?<code>")]
fn redirect(code: &RawStr) -> String {
    // Print out the code for debugging purposes.
    println!("{:#?}", code);
    // Set the access code and request an access token.
    // Callers should handle the Result from requesting an access token
    // in case of an error here.
    set_and_req_access_code(code);
    // Generic login page response. Note
    String::from("Successfully Logged In! You can close your browser.")
}

pub fn set_and_req_access_code(access_code: &str) {
    let mut oauth = OAUTH_CLIENT.oauth();

    oauth.access_code(access_code);
    let mut request = oauth.build().authorization_code_grant();

    let access_token = request.access_token().send().unwrap();
    oauth.access_token(access_token);
    println!("{:#?}", oauth);
}
