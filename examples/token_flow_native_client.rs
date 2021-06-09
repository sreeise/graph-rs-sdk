use graph_rs_sdk::oauth::OAuth;

// The following example shows authenticating an application to use the OneDrive REST API
// for a native client. Native clients typically use the implicit OAuth flow. This requires
// using the browser to log in. To get an access token, set the response type to 'token'
// which will return an access token in the URL. The implicit flow does not make POST requests
// for access tokens like other flows do.
//
// There are two versions of the implicit flow. The first, called token flow is used
// for Microsoft V1.0 OneDrive authentication. The second is Microsoft's implementation
// of the OAuth V2.0 implicit flow.
//
// Implicit flows are typically performed when requesting access tokens directly from
// the user agent such as from a browser using JavaScript.
//
// For more information on the implicit flows see:
// 1. Token flow for v1.0: https://docs.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online
// 2. Implicit grant flow for v2.0: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-implicit-grant-flow
//
// To better understand OAuth V2.0 and the implicit flow see: https://tools.ietf.org/html/rfc6749#section-1.3.2

fn main() {
    // Opens the default browser to the Microsoft login page.
    // After logging in the page will redirect and the Url
    // will have the access token in either the query or
    // the fragment of the Uri.
    let mut oauth = native_client();
    let mut request = oauth.build().implicit_grant();
    let _ = request.browser_authorization().open().unwrap();
}

fn native_client() -> OAuth {
    let mut oauth = OAuth::new();
    oauth
        .client_id("<YOUR_CLIENT_ID>")
        .redirect_uri("http://localhost:8000/redirect")
        .add_scope("Files.Read")
        .add_scope("Files.ReadWrite")
        .add_scope("Files.Read.All")
        .add_scope("Files.ReadWrite.All")
        .response_type("token")
        .response_mode("query")
        .prompt("login")
        .authorize_url("https://login.live.com/oauth20_authorize.srf?")
        .access_token_url("https://login.live.com/oauth20_token.srf");
    oauth
}
