use rust_onedrive::oauth::{Grant, OAuth, OAuthCredential};
use transform_request::prelude::*;

/*
The following example shows authenticating an application to use the OneDrive REST API
for a personal Microsoft Account.

First go to the Microsoft Application Portal and create a native OneDrive application:

    https://apps.dev.microsoft.com

Grant the application the following scopes:

    Personal Account: Files.Read, Files.ReadWrite, Files.Read.All, Files.ReadWrite.All, offline_access

More info can be found here: https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/drive_get?view=odsp-graph-online
*/

#[allow(dead_code)]
fn main() {
    // For token and code flows, native clients use the same credentials
    // as web clients except for a client id. First open the browser
    // and get the user to sign in.
    let mut oauth = native_client();
    oauth.request_authorization().unwrap();
    // Run this first, then get the code from the browser URL. Afterward,
    // comment out the browser_flow() method below and run set_code_request_token(your_access_token).
    //  oauth.browser_sign_in().unwrap();
}

#[allow(dead_code)]
fn native_client() -> OAuth {
    let mut oauth = OAuth::code_flow();
    // wl.offline_access will cause the request to return
    // a refresh token as well.
    // The authorization url will be https://login.live.com/oauth20_desktop.srf
    // for mobile and desktop applications. Additionally, mobile and desktop clients
    // do not use a client secret.
    oauth
        .client_id("<CLIENT ID>")
        .redirect_uri("https://login.microsoftonline.com/common/oauth2/nativeclient")
        .add_scope("Files.Read")
        .add_scope("Files.ReadWrite")
        .add_scope("Files.Read.All")
        .add_scope("Files.ReadWrite.All")
        .add_scope("wl.offline_access")
        .authorize_url("https://login.live.com/oauth20_authorize.srf?")
        .access_token_url("https://login.live.com/oauth20_token.srf");
    oauth
}

#[allow(dead_code)]
fn set_code_request_token(access_code: &str) {
    let mut oauth = native_client();

    // Set the access code that will be used to request an
    // access token and/or refresh token.
    oauth.access_code(access_code);

    // Makes the POST request for an access token/refresh token.
    // This is stored in the struct AccessToken.
    match oauth.request_access_token() {
        Ok(_) => println!("Sucess!"),
        Err(e) => println!("Error: {:#?}", e),
    }

    // If there is an issue with the request, OAuth stores
    // the error for the last request in the field req_error
    // which holds an Option<DriveError>. DriveError holds
    // the status code, error type such as BadRequest, and the
    // error info/reason.
    if oauth.get(OAuthCredential::AccessToken).is_some() {
        // Stores OAuth as json using serde_json.
        oauth
            .to_file("./examples/example_files/native_client_flow.json")
            .unwrap();
        println!("{:#?}", &oauth);
    }
    /*
    To get OAuth back from the json file run:

    let mut oauth: OAuth = OAuth::from_file("example/oauth.json").unwrap();
    */
}
