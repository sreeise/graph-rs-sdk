use rust_onedrive::flow::v1::{AccountType, AuthFlow};
use rust_onedrive::process::jsonio::JsonFile;

/*
The following example shows authenticating an application to use the OneDrive REST API
for a personal Microsoft Account.

First go to the Microsoft Application Portal and create a native OneDrive application:

    https://apps.dev.microsoft.com

Grant the application the following scopes:

    Personal Account: Files.Read, Files.ReadWrite, Files.Read.All, Files.ReadWrite.All, offline_access

More info can be found here: https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/drive_get?view=odsp-graph-online
*/

fn main() {
    let mut auth_flow = native_client();

    // Run this first, then get the code from the browser URL. Afterward,
    // comment out the browser_flow() method below and run set_code_request_token(your_access_token).
    auth_flow.browser_flow().unwrap();

    /*
    browser_flow() opens users default browser to the authentication page
    for Microsoft Accounts. AuthFlow knows what to open by
    the AccountType specified in native_client().

    Note that browser_flow() runs in different thread so the main thread
    is not blocked.

    Upon sign in, the page will redirect to the redirect given in
    get_auth_flow(). This redirect is specific to the URI set
    in the microsoft application portal. The redirect URI will
    return with a code. This code needs to be set in AuthFlow
    to make an access token request. Get the code from the
    browser url bar and
    */
}

fn get_auth_flow() -> AuthFlow {
    // native_client() sets using the default scopes to false
    // and therefore requires adding scopes manually. This can
    // be changed by the method: use_default_scope(true)

    // Native clients also automatically prevent using a client_id in
    // the request. An authentication request will get rejected
    // if a native client is the caller. Only web clients use client_id.
    let mut auth_flow = AuthFlow::native_client();
    // There are other possible URI's for the redirect URI. This is set in
    // the Microsoft application portal
    auth_flow
        .set_client_id("<CLIENT ID>")
        .set_redirect_uri("https://login.microsoftonline.com/common/oauth2/nativeclient");
    auth_flow
}

fn native_client() -> AuthFlow {
    let mut auth_flow = get_auth_flow();
    auth_flow
        .add_scope("Files.Read")
        .add_scope("Files.ReadWrite")
        .add_scope("Files.Read.All")
        .add_scope("Files.ReadWrite.All")
        .add_scope("wl.offline_access"); // wl.offline_access will cause the request to return
                                         // a refresh token as well.

    auth_flow.use_default_auth_url(AccountType::Account);
    auth_flow
}

fn set_code_request_token(access_code: &str) {
    let mut auth_flow = native_client();

    // Set the access code that will be used to request an
    // access token and/or refresh token.
    auth_flow.set_access_code(access_code);

    // Makes the POST request for an access token/refresh token.
    // This is stored in the struct AccessToken.
    auth_flow.request_access_token();
    // Stores AuthFlow as json using serde_json.
    JsonFile::json_file("examples/native_client_flow.json", &auth_flow).unwrap();
    println!("{:#?}", &auth_flow);

    /*
    To get AuthFlow back from the json file run:

    let mut auth_flow: AuthFlow = JsonFile::from_file("example/auth_flow.json").unwrap();
    */
}
