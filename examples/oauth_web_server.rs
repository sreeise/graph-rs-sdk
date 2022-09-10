#[macro_use]
extern crate serde;
extern crate reqwest;
extern crate serde_json;

use warp::{http::Response, Filter};

use from_as::*;
use graph_rs_sdk::oauth::OAuth;

// The client_id and client_secret must be changed before running this example.
static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AccessCode {
    code: String,
}

// This example shows using Rocket to authenticate with Microsoft OneDrive,
// and then requesting drive resources from the Graph API.
//
// This example uses the code flow: https://docs.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online
//
// If you have not set up an application to call the Graph API for OneDrive
// API then you will want to read through the following setup information as well
// as the related information for the Microsoft Graph/OneDrive API
//
// The examples below have been built to work for the Microsoft Graph API V1 and V2
// for personal and business Microsoft accounts. Using Azure AD may work but
// this has not been tested. This example goes through the OAuth code flow
// for Microsoft accounts.
//
// Setup:
//
// You will first need to head to the Microsoft Application Portal and create and
// application. Once the application is created you will need to specify the
// scopes you need and change them accordingly in the oauth_web_client() method
// when adding scopes using OAuth::add_scope("scope").
//
// For reference the Microsoft Graph Authorization V2 required parameters along with
// the methods to use needed to be set are shown above the oauth_web_client() method.
//
// Once an application is registered it will given an application id which is the client id in an OAuth2 request.
// For this example, a client secret will need to be generated. The client secret is the same as the password
// under Application Secrets int the registration portal. If you do not have a client secret then click on
// 'Generate New Password'.  Next click on 'Add Platform' and create a new web platform.
// Add a redirect url to the platform. In the example below, the redirect url is http://localhost:8000/redirect
// but anything can be used.
//
// Overview:
//
// After signing in, you will be redirected, and the access code that is given in the redirect
// will be used to automatically call the access token endpoint and receive an access token
// and/or refresh token.
//
// Disclaimer/Important Info:
//
// This example is meant for testing and is not meant to be production ready or complete.
#[tokio::main]
async fn main() {
    let query = warp::query::<AccessCode>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<AccessCode>,), std::convert::Infallible>((None,)) });

    let routes = warp::get().and(warp::path("redirect")).and(query).map(
        |code_option: Option<AccessCode>| match code_option {
            Some(code) => {
                // Print out the code for debugging purposes.
                println!("{:#?}", code);

                // Set the access code and request an access token.
                // Callers should handle the Result from requesting an access token
                // in case of an error here.
                set_and_req_access_code(code);

                // Generic login page response.
                Response::builder().body(String::from(
                    "Successfully Logged In! You can close your browser.",
                ))
            }
            None => Response::builder()
                .body(String::from("There was an issue getting the access code.")),
        },
    );

    // Get the oauth client and request a browser sign in
    let mut oauth = oauth_web_client();
    let mut request = oauth.build().code_flow();
    request.browser_authorization().open().unwrap();

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

// Methods for authenticating with the Graph API

// This method creates an OAuth instance and inserts the parameters needed for authorization.
// Note the credentials given to OAuth should be the ones you set for your application
// in the Microsoft Registration Portal.
//
// For more information visit the related URLs below:
//
// Microsoft Accounts
// # Authorization for Accounts: https://docs.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online
// 1. Authorization url: oauth.authorize_url("https://example.com/authorize?");
// 1. Client Id: oauth.client_id("client id");
// 2. Scope: oauth.add_scope("Read").add_scope("Read.Write");
// 3. Response Type: oauth.response_type("code"); => This is set implicitly and does not need to be manually set.
// 4. Redirect URL: oauth.redirect_url("http://localhost:8000/redirect") The redirect URL used to redirect to after authentication.
//
// Note: You do not need to set the access code in this example. This is done when Rocket intercepts the request.
// The code is appended onto the end of the redirect url and used to call OneDrive API for
// an access Token. The OAuth config holds Access Token is then stored in authorize_configs/web_oauth.json.
//
// # Access Token Request for Accounts:
// 1. Client Id: oauth.client_id("client id");
// 2. Client Secret: oauth.client_secret("client_secret");
// 3. Redirect URI: oauth.redirect_url("url");
// 4. Code: oauth.access_code("Code given in url from authorization request");
// 5. Grant Type: This is set implicitly and does not need to be set manually,
// however, the caller can specify a custom grant type by calling:
// oauth.grant_type("token");
//
// Microsoft Graph:
// For the V2 endpoint you will want to change the methods below to:
// authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?");
// access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token");
// refresh_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token");
//
// Also change the scope of wl.offline_access to just offline_access
//
// The scopes given below will allow you to access most of the needed items for
// the Graph OneDrive API.
fn oauth_web_client() -> OAuth {
    let mut oauth = OAuth::new();
    oauth
        .client_id(CLIENT_ID)
        .client_secret(CLIENT_SECRET)
        .add_scope("Files.Read")
        .add_scope("Files.ReadWrite")
        .add_scope("Files.Read.All")
        .add_scope("Files.ReadWrite.All")
        .add_scope("wl.offline_access")
        .redirect_uri("http://localhost:8000/redirect")
        .authorize_url("https://login.live.com/oauth20_authorize.srf?")
        .access_token_url("https://login.live.com/oauth20_token.srf")
        .refresh_token_url("https://login.live.com/oauth20_token.srf")
        .response_type("code")
        .logout_url("https://login.live.com/oauth20_logout.srf?")
        .post_logout_redirect_uri("http://localhost:8000/redirect");
    oauth
}

pub fn set_and_req_access_code(access_code: AccessCode) {
    let mut oauth = oauth_web_client();
    // The response type is automatically set to token and the grant type is automatically
    // set to authorization_code if either of these were not previously set.
    // This is done here as an example.
    oauth.access_code(access_code.code.as_str());

    let mut request = oauth.build().code_flow();
    let access_token = request.access_token().send().unwrap();
    oauth.access_token(access_token);

    // If all went well here we can print out the OAuth config with the Access Token.
    println!("{:#?}", &oauth);
}
