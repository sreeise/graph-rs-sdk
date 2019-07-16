#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
#[macro_use]
extern crate rocket;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
extern crate reqwest;

use from_to_file::*;
use rocket::http::RawStr;
use rocket_codegen::routes;
use rust_onedrive::oauth::{Grant, OAuth};
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_secs(2));
        let mut oauth = oauth_web_client();
        let _ = oauth.request_authorization().unwrap();
    });

    rocket::ignite().mount("/", routes![redirect]).launch();
    handle.join().unwrap();
}

fn oauth_web_client() -> OAuth {
    let mut oauth = OAuth::authorization_code_grant();
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
    let mut oauth = oauth_web_client();
    // The response type is automatically set to token and the grant type is automatically
    // set to authorization_code if either of these were not previously set.
    // This is done here as an example.
    oauth.access_code(access_code);
    oauth.request_access_token().unwrap();

    // If all went well here we can print out the OAuth config with the Access Token.
    println!("{:#?}", &oauth);

    // Save our configuration to a file so we can retrieve it from other requests.
    oauth
        .to_json_file("./examples/example_files/web_oauth.json")
        .unwrap();
}
