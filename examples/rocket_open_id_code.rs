#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
#[macro_use]
extern crate rocket;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
extern crate reqwest;
use rocket::Data;
use rocket_codegen::routes;
use rust_onedrive::oauth::{Grant, IdToken, OAuth};
use rust_onedrive::transform::Transform;
use std::io::Read;
use std::thread;
use std::time::Duration;
use transform_request::ToFile;

// Note: The open id connect grant is a work in progress.

// Create an OAuth struct with the needed credentials.
fn oauth_open_id() -> OAuth {
    let mut oauth = OAuth::open_id_connect();
    oauth
        .client_id("<YOUR_CLIENT_ID>")
        .client_secret("<YOUR_CLIENT_SECRET>")
        .authorize_url("https://login.microsoftonline.com/<YOUR_TENANT or other Microsoft values such as common>/oauth2/v2.0/authorize")
        .redirect_url("http://localhost:8000/redirect")
        .access_token_url("https://login.microsoftonline.com/<YOUR_TENANT or other Microsoft values such as common>/oauth2/v2.0/token")
        .refresh_token_url("https://login.microsoftonline.com/<YOUR_TENANT or other Microsoft values such as common>/oauth2/v2.0/token")
        .response_type("id_token code")
        .response_mode("form_post")
        .add_scope("openid")
        .add_scope("Files.Read")
        .add_scope("Files.ReadWrite")
        .nonce("7362CAEA-9CA5")
        .prompt("login")
        .state("12345");
    oauth
}

fn main() {
    // Spawn the browser to sign in within a different thread that waits until
    // rocket has started. Otherwise, the redirect from sign in may happen
    // before rocket has started.
    let handle = thread::spawn(|| {
        // Block the new thread and give enough time for rocket to completely start.
        thread::sleep(Duration::from_secs(2));
        // Use the OpenId trait from OAuth to request an access code.
        // The full name syntax is used here so it does not clash with methods
        // in the other grant types.
        let mut oauth = oauth_open_id();
        oauth.request_authorization().unwrap();
    });

    rocket::ignite().mount("/", routes![redirect]).launch();
    handle.join().unwrap();
}

#[post("/redirect", data = "<id_token>")]
fn redirect(id_token: Data) {
    // Read in the response body to a String
    let mut s = String::new();
    id_token.open().read_to_string(&mut s).unwrap();
    // Print the string for debugging in case the attempt to deserialize the response
    // in the transform method below does not work..
    println!("Token response:\n{:#?}\n", s);

    // Transform the String to an IdToken
    // and pass the IdToken to OAuth.
    let token: IdToken = IdToken::transform(s).unwrap();
    println!("IdToken:\n{:#?}\n", token);
    let mut oauth = oauth_open_id();
    oauth.id_token(token);
    access_token(&mut oauth);
}

pub fn access_token(oauth: &mut OAuth) {
    oauth.request_access_token().unwrap();
    // If all went well here we can print out the OAuth config with the Access Token.
    println!("OAuth:\n{:#?}\n", &oauth);
    oauth
        .to_file("./examples/example_files/web_oauth.json")
        .unwrap();
}
