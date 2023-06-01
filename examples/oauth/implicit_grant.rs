use std::collections::BTreeSet;
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
use graph_rs_sdk::oauth::{
    CredentialBuilder, ImplicitCredential, Prompt, ResponseMode, ResponseType,
};

fn oauth_implicit_flow() {
    let authorizer = ImplicitCredential::builder()
        .with_client_id("<YOUR_CLIENT_ID>")
        .with_redirect_uri("http://localhost:8000/redirect")
        .with_prompt(Prompt::Login)
        .with_response_type(ResponseType::Token)
        .with_response_mode(ResponseMode::Fragment)
        .with_redirect_uri("https::/localhost:8080/myapp")
        .with_scope(["User.Read"])
        .with_nonce("678910")
        .build();

    let url = authorizer.url().unwrap();

    // webbrowser crate in dev dependencies will open to default browser in the system.

    // Opens the default browser to the Microsoft login page.
    // After logging in the page will redirect and the Url
    // will have the access token in either the query or
    // the fragment of the Uri.
    webbrowser::open(url.as_str()).unwrap();
}

fn multi_response_types() {
    let _ = ImplicitCredential::builder()
        .with_response_type(vec![ResponseType::Token, ResponseType::IdToken])
        .build();

    // Or

    let _ = ImplicitCredential::builder()
        .with_response_type(ResponseType::StringSet(BTreeSet::from_iter(vec![
            "token".to_string(),
            "id_token".to_string(),
        ])))
        .build();
}
