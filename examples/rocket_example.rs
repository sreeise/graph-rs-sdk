#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
#[macro_use]
extern crate rocket;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
extern crate reqwest;

use rocket::http::RawStr;
use rocket_codegen::routes;
use rust_onedrive::drive::driveitem::DriveItem;
use rust_onedrive::drive::ItemResult;
use rust_onedrive::drive::{Drive, EP};
use rust_onedrive::jsonfile::JsonFile;
use rust_onedrive::oauth::OAuth;
use std::thread;
use std::time::Duration;

/*
This example shows using Rocket to authenticate with Microsoft OneDrive,
and then requesting drive resources from the Graph API.

If you have not set up an application to call the Graph API for OneDrive
API then you will want to read through the following setup information as well
as the related information for the Microsoft Graph/OneDrive API

The examples below have been built to work for the Microsoft Graph API V1 and V2
for personal and business Microsoft accounts. Using Azure AD may work but
this has not been tested. This example goes through the OAuth code flow
for Microsoft accounts. This type of authentication would normally
run on a server where you can listen for the url being redirected to
after authentication. The is the same redirect url that will be specified
below when creating the OAuth instance in method oauth_web_client().

Setup:

You will first need to head to the Microsoft Application Portal and create and
application. Once the application is created you will need to specify the
scopes you need and change them accordingly in the oauth_web_client() method
when adding scopes using OAuth::add_scope("scope").

For reference the Microsoft Graph Authorization V2 required parameters along with
the methods to use needed to be set are shown above the oauth_web_client() method.

Once an application is registered it will given an application id which is the client id in an OAuth2 request.
For this example, a client secret will need to be generated. The client secret is the same as the password
under Application Secrets int the registration portal. If you do not have a client secret then click on
'Generate New Password'.  Next click on 'Add Platform' and create a new web platform.
Add a redirect url to the platform. In the example below, the redirect url is http://localhost:8000/redirect
but anything can be used.

Overview:

Rocket will listen for the redirect url when the user has signed in: fn redirect() below.
When this happens, the access code that is given in the redirect will be used to automatically
call the access token endpoint and receive an access token and/or refresh token.

Disclaimer/Important Info:

This example is meant for testing and is not meant to be production ready or complete.
*/
fn main() {
    // Get the OAuth instance specified in the method oauth_web_client() below.
    let mut oauth = oauth_web_client();
    // The url used is the same url given in method: OAuth::authorize_url()
    // Or you can optionally use oauth.browser_sign_in_url("https://login.live.com/oauth20_authorize.srf?");
    oauth.browser_sign_in().unwrap();

    rocket::ignite()
        .mount("/", routes![redirect, recent])
        .launch();
}

// Use this in the main method if the browser is redirecting
// before Rocket has fully started. The redirect url
// contains the access code to request an access token. The access code
// is appended to the end of the redirect URL.
// Rocket is used to retrieve this code from the URL.
// In cases where the user is redirected automatically
// such as when the user is already signed in, Rocket may
// not have started up completely.
fn with_browser_thread() {
    // Spawn the browser in a different thread or it may
    // block Rocket from starting.
    let code_handle = thread::spawn(|| {
        // Block the new thread and give enough time for rocket to completely start.
        thread::sleep(Duration::from_secs(2));
        // Get the oauth client and request a browser sign in
        // The url used is the same url given in method: OAuth::authorize_url()
        // Or you can optionally use oauth.browser_sign_in_url("https://login.live.com/oauth20_authorize.srf?");
        let mut oauth = oauth_web_client();
        oauth.browser_sign_in().unwrap();
    });

    rocket::ignite()
        .mount("/", routes![redirect, recent])
        .launch();
    code_handle.join().unwrap();
}

// Methods for authenticating with the Graph API

/*
    This method creates an OAuth instance and inserts the parameters needed for authorization.
    Note the credentials given to OAuth should be the ones you set for your application
    in the Microsoft Registration Portal.

    For more information visit the related URLs below:

  Microsoft Accounts
    # Authorization for Accounts: https://docs.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online
        1. Authorization url: oauth.authorize_url("https://example.com/authorize?");
        1. Client Id: oauth.client_id("client id");
        2. Scope: oauth.add_scope("Read").add_scope("Read.Write");
        3. Response Type: oauth.response_type("code"); => This is set implicitly and does not need to be manually set.
        4. Redirect URL: oauth.redirect_url("http://localhost:8000/redirect") The redirect URL used to redirect to after authentication.

// Note: You do not need to set the access code in this example. This is done when Rocket intercepts the request.
// The code is appended onto the end of the redirect url and used to call OneDrive API for
// an access Token. The OAuth config holds Access Token is then stored in authorize_configs/web_oauth.json.

   # Access Token Request for Accounts:
        1. Client Id: oauth.client_id("client id");
        2. Client Secret: oauth.client_secret("client_secret");
        3. Redirect URI: oauth.redirect_url("url");
        4. Code: oauth.access_code("Code given in url from authorization request");
        5. Grant Type: This is set implicitly and does not need to be set manually,
            however, the caller can specify a custom grant type by calling:
                oauth.grant_type("token");

  Microsoft Graph:
    For the V2 endpoint you will want to change the methods below to:
        authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?");
        access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token");
        refresh_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token");

The scopes given below will allow you to access most of the needed items for
the Graph OneDrive API.
*/
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
        .redirect_url("http://localhost:8000/redirect")
        .authorize_url("https://login.live.com/oauth20_authorize.srf?")
        .access_token_url("https://login.live.com/oauth20_token.srf")
        .refresh_token_url("https://login.live.com/oauth20_token.srf")
        .response_mode("query");
    oauth
}

#[get("/redirect?<code>")]
fn redirect(code: &RawStr) -> String {
    // Print out the code for debugging purposes.
    println!("{:#?}", code);
    // Set the access code and request an access token.
    // Callers should handle the Result from requesting an access token
    // in case of an error here.
    set_and_req_access_code(code).unwrap();
    // Generic login page response. Note
    String::from("Successfully Logged In! You can close your browser.")
}

pub fn set_and_req_access_code(access_code: &str) -> ItemResult<()> {
    let mut oauth = oauth_web_client();
    oauth.response_type("token");
    oauth.access_code(access_code);
    oauth.request_access_token().unwrap();

    // If all went well here we can print out the OAuth config with the Access Token.
    println!("{:#?}", &oauth);

    // Save our configuration to a file so we can retrieve it from other requests.
    JsonFile::json_file("./examples/example_files/web_oauth.json", &oauth)
}
// Methods for calling the Graph API.

// This method gets gets recent drive items from the API.
// All OneDrive requests result in the same struct:
//      Result<DriveItem, RequestError>
//
// where DriveItem holds all the recent items you requested if the request is successful.
// If there is an error, then a RequestError will be returned. RequestError will also store
// an error from the Graph API if error originated from there. Errors for the Graph API
// can be found here: https://docs.microsoft.com/en-us/onedrive/developer/rest-api/concepts/errors?view=odsp-graph-online
//
//
// This will store a users recent drive items in example_files/drive_recent.json after printing
// the drive item to the console or the error if the request is unsuccessful.
//
// Curl: curl http://localhost:8000/drive/recent
// This will store the drive item in examples/example_files/drive_recent.json.
// You can use method recent_from_file() to get and print the stored drive item.
// CAREFUL: This may contain sensitive information!
#[get("/drive/recent", format = "application/json")]
fn recent() {
    let oauth: OAuth = JsonFile::from_file("./examples/example_files/web_oauth.json").unwrap();
    let mut drive = Drive::from(oauth);
    let result = drive.drive_recent();
    match result {
        Ok(t) => {
            println!("{:#?}", &t);
            JsonFile::json_file("./examples/example_files/drive_recent.json", &t).unwrap();
        },
        Err(e) => println!("{:#?}", e),
    };
}

fn recent_from_file() {
    let item: DriveItem =
        JsonFile::from_file("./examples/example_files/drive_recent.json").unwrap();
    println!("{:#?}", &item);
}
