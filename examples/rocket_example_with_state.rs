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

/*
This example shows using Rocket to authenticate with Microsoft OneDrive that
includes authorization with a state parameter in the request query.

This example uses the code flow: https://docs.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online

If you have not set up an application to call the Graph API for OneDrive
API then you will want to first read through the information in rocket_example.rs
before moving forward. A client id and client secret are needed to
call the overdrive API in order to authenticate users. This is done
through the Microsoft application portal or through Azure. Creating an
application will create an application ID which is the client id. Then,
under application secrets, a new password will need to be generated. This
password is the client secret. The rocket_example.rs file has more information
on how to set up an application.
*/

// Create an OAuth struct with the needed credentials.
fn oauth_web_client() -> OAuth {
    let mut oauth = OAuth::new();
    oauth
        .client_id("<YOUR_CLIENT_ID>")
        .client_secret("<YOUR_CLIENT_SECRET>")
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

fn main() {
    // The client_id and client_secret must be changed in the oauth_web_client()
    // method before running this example.

    // Spawn the browser to sign in within a different thread that waits until
    // rocket has started. Otherwise, the redirect from sign in may happen
    // before rocket has started.
    let handle = thread::spawn(|| {
        // Block the new thread and give enough time for rocket to completely start.
        thread::sleep(Duration::from_secs(2));
        // Get the oauth client and request a browser sign in
        // The url used is the same url given in method: OAuth::authorize_url()
        let mut oauth = oauth_web_client();
        let mut request = oauth.build().code_flow();
        request.browser_authorization().open().unwrap();
    });

    rocket::ignite().mount("/", routes![redirect]).launch();
    handle.join().unwrap();
}

#[get("/redirect?<code>&<state>")]
fn redirect(code: &RawStr, state: &RawStr) -> String {
    // Print out the code and state for debugging purposes.
    println!("{:#?}", code);
    println!("{:#?}", state);

    // Assert that the state is the same as the one given in the original request.
    assert_eq!("13534298", state.to_string());

    // Set the access code and request an access token.
    // Callers should handle the Result from requesting an access token
    // in case of an error here.
    set_and_req_access_code(code, state);
    // Generic login page response.
    String::from("Successfully Logged In! You can close your browser.")
}

pub fn set_and_req_access_code(access_code: &str, state: &str) {
    let mut oauth = oauth_web_client();
    oauth.response_type("token");
    oauth.state(state);
    oauth.access_code(access_code);

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
