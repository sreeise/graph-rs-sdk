# OAuth client implementing the OAuth 2.0 and OpenID Connect protocols for Microsoft identity platform

Purpose built as OAuth client for Microsoft Graph and the [graph-rs-sdk](https://crates.io/crates/graph-rs-sdk) project.
This project can however be used outside [graph-rs-sdk](https://crates.io/crates/graph-rs-sdk) as an OAuth client
for Microsoft Identity Platform or by using [graph-rs-sdk](https://crates.io/crates/graph-rs-sdk).

For async:

```toml
graph-oauth = "1.0.1"
tokio = { version = "1.25.0", features = ["full"] }
```

For blocking:

```toml
graph-oauth = "1.0.1"
```

See the project on [GitHub](https://github.com/sreeise/graph-rs-sdk).

### Supported Authorization Flows

#### Microsoft OneDrive and SharePoint

- [Token Flow](https://learn.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/graph-oauth?view=odsp-graph-online#token-flow)
- [Code Flow](https://learn.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/graph-oauth?view=odsp-graph-online#code-flow)

#### Microsoft Identity Platform

- [Authorization Code Grant](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow)
- [Authorization Code Grant PKCE](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow)
- [Open ID Connect](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc)
- [Implicit Grant](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-implicit-grant-flow)
- [Device Code Flow](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code)
- [Client Credentials](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow)
- [Resource Owner Password Credentials](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth-ropc)

For more extensive examples and explanations see the
[OAuth Examples](https://github.com/sreeise/graph-rs-sdk/tree/master/examples/oauth) in the examples/oauth
directory on [GitHub](https://github.com/sreeise/graph-rs-sdk).

```rust
use graph_oauth::oauth::{AccessToken, OAuth};

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
    let response = request.access_token().send().await?;
    println!("{response:#?}");

    if response.status().is_success() {
        let mut access_token: AccessToken = response.json().await?;

        // Option<&JsonWebToken>
        let jwt = access_token.jwt();
        println!("{jwt:#?}");

        oauth.access_token(access_token);

        // If all went well here we can print out the OAuth config with the Access Token.
        println!("{:#?}", &oauth);
    } else {
        // See if Microsoft Graph returned an error in the Response body
        let result: reqwest::Result<serde_json::Value> = response.json().await;

        match result {
            Ok(body) => println!("{body:#?}"),
            Err(err) => println!("Error on deserialization:\n{err:#?}"),
        }
    }
}
```
