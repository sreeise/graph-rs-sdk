#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
#[macro_use]
extern crate rocket;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
extern crate reqwest;

use from_as::*;
use graph_rs::oauth::OAuth;
use rocket::http::RawStr;
use rocket_codegen::routes;
use std::thread;
use std::time::Duration;

// Client Credentials Grant
// If you have already given admin consent to a user you can skip
// browser authorization step and go strait to requesting an access token.

fn main() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_secs(2));
        let mut oauth = oauth_web_client();
        let mut request = oauth.build().client_credentials();
        request.browser_authorization().open().unwrap();
    });

    rocket::ignite().mount("/", routes![redirect]).launch();
    handle.join().unwrap();
}

fn oauth_web_client() -> OAuth {
    let mut oauth = OAuth::new();
    oauth
        .client_id("<YOUR_CLIENT_ID>")
        .client_secret("<YOUR_CLIENT_SECRET>")
        .add_scope("https://graph.microsoft.com/.default")
        .redirect_uri("http://localhost:8000/redirect")
        .authorize_url("https://login.microsoftonline.com/common/adminconsent")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token");
    oauth
}

#[get("/redirect?<admin_consent>&<tenant>")]
fn redirect(admin_consent: &RawStr, tenant: &RawStr) -> String {
    println!("Admin consent: {:#?}", admin_consent);
    println!("Tenant: {:#?}", tenant);
    set_and_req_access_code();
    // Generic login page response.
    String::from("Successfully Logged In! You can close your browser.")
}

pub fn set_and_req_access_code() {
    let mut oauth = oauth_web_client();

    let mut request = oauth.build().client_credentials();
    let access_token = request.access_token().send().unwrap();
    oauth.access_token(access_token);

    println!("{:#?}", &oauth);

    oauth
        .as_file("./examples/example_files/client_credentials.json")
        .unwrap();
}
