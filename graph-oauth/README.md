# graph-oauth

OAuth client for Microsoft Graph and the graph-rs project.

See the project on [GitHub](https://github.com/sreeise/graph-rs).

### Authorization Flows and Microsoft Graph

    1. Token Flow - v1.0
    2. Code Flow - v1.0
    3. Authorization Code Grant - v1.0 and beta
    4. Open ID - v1.0 and beta
    5. Implicit - v1.0 and beta
    6. Client Credentials - v1.0 and beta
    7. Resource Owner Password Credentials - v1.0 and beta

For more examples see the example's directory in the graph-rs project
on [GitHub](https://github.com/sreeise/graph-rs).

For more information on Microsoft graph and OAuth 2.0 authorization flows see:
https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-app-types

```rust
use graph_oauth::oauth::OAuth;

fn main() {
    let mut oauth = OAuth::new();
    oauth
        .client_id("<YOUR_CLIENT_ID>")
        .client_secret("<YOUR_CLIENT_SECRET>")
        .add_scope("files.read")
        .add_scope("files.readwrite")
        .add_scope("files.read.all")
        .add_scope("files.readwrite.all")
        .add_scope("offline_access")
        .redirect_uri("http://localhost:8000/redirect")
        .authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .refresh_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .response_type("code")
        .logout_url("https://login.microsoftonline.com/common/oauth2/v2.0/logout")
        .post_logout_redirect_uri("http://localhost:8000/redirect");

    let mut request = oauth.build().authorization_code_grant();
    
    // Opens the default browser.
    let _ = request.browser_authorization().open();
    
    // The access code will be appended to the url on redirect. Pass
    // this code to the OAuth instance:
    oauth.access_code("<ACCESS CODE>");

    // Perform an authorization code grant request for an access token:
    let mut request = oauth.build().authorization_code_grant();
    let access_token = request.access_token().send().unwrap();
    println!("{:#?}", access_token);
}
```
