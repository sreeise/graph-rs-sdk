use rust_onedrive::flow::v1::AccountType;
use rust_onedrive::flow::v1::AuthFlow;
use rust_onedrive::process::jsonio::JsonFile;

fn main() {
    let mut auth_flow = web_client();

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

fn web_client() -> AuthFlow {
    // Setting true in the new() method here tells AuthFlow
    // to use the default scopes that are set in the application portal.
    // A Microsoft URL that can retrieve these scopes is then used instead and
    // makes it easier to set up AuthFlow. The scopes can still be implemented
    // manually by setting AuthFlow::new to false and using method add_scope()
    let mut auth_flow = AuthFlow::web_client(true);
    auth_flow
        .set_client_id("<CLIENT_ID>")
        // Or whatever you set the redirect to for a web client.
        .set_redirect_uri("http://localhost:8000/redirect")
        .set_client_secret("<CLIENT_SECRET>");
    auth_flow.use_default_auth_url(AccountType::Account);
    auth_flow
}

fn set_code_request_token(access_code: &str) {
    let mut auth_flow = web_client();

    // Set the access code that will be used to request an
    // access token and/or refresh token.
    auth_flow.set_access_code(access_code);

    // Makes the POST request for an access token/refresh token.
    // This is stored in the struct AccessToken.
    auth_flow.request_access_token();

    // If there is an issue with the request, AuthFlow stores
    // the error for the last request in the field req_error
    // which holds an Option<DriveError>. DriveError holds
    // the status code, error type such as BadRequest, and the
    // error info/reason.
    if auth_flow.req_error.is_some() {
        println!("{:#?}", auth_flow.req_error); // Some(DriveError)
    } else {
        // Stores AuthFlow as json using serde_json.
        JsonFile::json_file("example/web_client_flow.json", &auth_flow).unwrap();
    }

    /*
    To get AuthFlow back from the json file run:

    let mut auth_flow: AuthFlow = JsonFile::from_file("example/auth_flow.json").unwrap();
    */
}
