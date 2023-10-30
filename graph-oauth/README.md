# OAuth 2.0 and OpenID Connect Client For The Microsoft Identity Platform

Support for:

- Automatic Token Refresh
- Interactive Authentication | features = [`interactive-auth`]
- Device Code Polling
- Authorization Using Certificates | features = [`openssl`]

Purpose built as OAuth client for Microsoft Graph and the [graph-rs-sdk](https://crates.io/crates/graph-rs-sdk) project.
This project can however be used outside [graph-rs-sdk](https://crates.io/crates/graph-rs-sdk) as an OAuth client
for Microsoft Identity Platform or by using [graph-rs-sdk](https://crates.io/crates/graph-rs-sdk).

For async:

```toml
graph-oauth = "1.0.2"
tokio = { version = "1.25.0", features = ["full"] }
```

For blocking:

```toml
graph-oauth = "1.0.2"
```

### Feature Flags

- `native-tls`: Use the `native-tls` TLS backend (OpenSSL on *nix, SChannel on Windows, Secure Transport on macOS).
- `rustls-tls`: Use the `rustls-tls` TLS backend (cross-platform backend, only supports TLS 1.2 and 1.3).
- `interactive-auth`: Interactive Authentication using the [wry](https://github.com/tauri-apps/wry) crate to run web view on
  platforms that support it such as on a desktop.
- `openssl`: Use X509 Certificates from the openssl crate in the OAuth2 and OpenId Connect flows. 

Default features: `default=["native-tls"]`

These features enable the native-tls and rustls-tls features in the reqwest crate. 
For more info see the [reqwest](https://crates.io/crates/reqwest) crate.



## OAuth - Getting Access Tokens

The crate is undergoing major development in order to support all or most scenarios in the
Microsoft Identity Platform where its possible to do so. The master branch on GitHub may have some
unstable features. Any version that is not a pre-release version of the crate is considered stable.

Use application builders to store your auth configuration and have the client
handle the access token requests for you.

There are two main types for building your chosen OAuth or OpenId Connect Flow.

- `PublicClientApplication`
- `ConfidentialClientApplication`

Once you have built a `ConfidentialClientApplication` or a `PublicClientApplication`
you can pass these to the graph client.

Automatic token refresh is also done by passing the `ConfidentialClientApplication` or the
`PublicClientApplication` to the `Graph` client.

For more extensive examples see the
[OAuth Examples](https://github.com/sreeise/graph-rs-sdk/tree/master/examples/oauth) in the examples/oauth
directory on [GitHub](https://github.com/sreeise/graph-rs-sdk).


```rust,ignore
let confidental_client: ConfidentialClientApplication<ClientSecretCredential> = ...

let graph_client = Graph::from(confidential_client);
```

### Identity Platform Support

The following flows from the Microsoft Identity Platform are supported:

- [Authorization Code Grant](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow)
- [Authorization Code Grant PKCE](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow)
- [Authorization Code Grant Certificate](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-auth-code-flow#request-an-access-token-with-a-certificate-credential)
- [Open ID Connect](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc)
- [Device Code Flow](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code)
- [Client Credentials](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow)
- [Resource Owner Password Credentials](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth-ropc)

You can use the url builders for those flows that require an authorization code using a redirect after sign in you can use

### Examples

### Authorization Code Grant

The authorization code grant is considered a confidential client (except in the hybrid flow)
and we can get an access token by using the authorization code returned in the query of the URL
on redirect after sign in is performed by the user.

Once you have the authorization code you can pass this to the client and the client
will perform the request to get an access token on the first graph api call that you make.

```rust
use graph_rs_sdk::{
    Graph,
    oauth::ConfidentialClientApplication,
};

#[tokio::main]
async fn main() {
    let authorization_code = "<AUTH_CODE>";
    let client_id = "<CLIENT_ID>";
    let client_secret = "<CLIENT_SECRET>";
    let scope = vec!["<SCOPE>", "<SCOPE>"];
    let redirect_uri = "http://localhost:8080";

    let mut confidential_client = ConfidentialClientApplication::builder(client_id)
        .with_authorization_code(authorization_code) // returns builder type for AuthorizationCodeCredential
        .with_client_secret(client_secret)
        .with_scope(scope)
        .with_redirect_uri(redirect_uri)
        .unwrap()
        .build();

    let graph_client = Graph::from(confidential_client);

    let _response = graph_client
        .users()
        .list_user()
        .send() // Also makes first access token request at this point
        .await;
}
```

### Client Credentials Grant.

The OAuth 2.0 client credentials grant flow permits a web service (confidential client) to use its own credentials,
instead of impersonating a user, to authenticate when calling another web service. The grant specified in RFC 6749,
sometimes called two-legged OAuth, can be used to access web-hosted resources by using the identity of an application.
This type is commonly used for server-to-server interactions that must run in the background, without immediate
interaction with a user, and is often referred to as daemons or service accounts.

Client credentials flow requires a one time administrator acceptance
of the permissions for your apps scopes. To see an example of building the URL to sign in and accept permissions
as an administrator see [Admin Consent Example](https://github.com/sreeise/graph-rs-sdk/tree/master/examples/oauth/client_credentials/client_credentials_admin_consent.rs)

```rust
use graph_rs_sdk::{
  oauth::ConfidentialClientApplication, Graph
};

static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";
static TENANT_ID: &str = "<TENANT_ID>";

pub async fn get_graph_client() -> Graph {
  let mut confidential_client_application = ConfidentialClientApplication::builder(CLIENT_ID)
          .with_client_secret(CLIENT_SECRET)
          .with_tenant(TENANT_ID)
          .build();

  Graph::from(confidential_client_application)
}
```


### Automatic Token Refresh

Using automatic token refresh requires getting a refresh token as part of the token response.
To get a refresh token you must include the `offline_access` scope.

Automatic token refresh is done by passing the `ConfidentialClientApplication` or the
`PublicClientApplication` to the `Graph` client.

If you are using the `client credentials` grant you do not need the `offline_access` scope.
Tokens will still be automatically refreshed as this flow does not require using a refresh token to get
a new access token.

```rust
async fn authenticate() {
  let scope = vec!["offline_access"];
  let mut credential_builder = ConfidentialClientApplication::builder(CLIENT_ID)
          .auth_code_url_builder()
          .interactive_authentication(None) // Open web view for interactive authentication sign in
          .unwrap();
  // ... add any other parameters you need

  let confidential_client = credential_builder.with_client_secret(CLIENT_SECRET)
          .build();
  
  let client = Graph::from(&confidential_client);
}
```


### Interactive Authentication

Requires Feature `interactive_auth`

```toml
[dependencies]
graph-rs-sdk = { version = "...", features = ["interactive_auth"] }
```

Interactive Authentication uses the [wry](https://github.com/tauri-apps/wry) crate to run web view on
platforms that support it such as on a desktop.

```rust
use graph_rs_sdk::oauth::{
  web::Theme, web::WebViewOptions, AuthorizationCodeCredential,
  ConfidentialClientApplication
};
use graph_rs_sdk::Graph;

fn run_interactive_auth() -> ConfidentialClientApplication<AuthorizationCodeCredential> {
  let mut confidential_client_builder = ConfidentialClientApplication::builder(CLIENT_ID)
          .auth_code_url_builder()
          .with_tenant(TENANT_ID)
          .with_scope(vec!["user.read"])
          .with_offline_access() // Adds offline_access as a scope which is needed to get a refresh token.
          .with_redirect_uri(REDIRECT_URI)
          .interactive_authentication(None)
          .unwrap();

  confidential_client_builder.with_client_secret(CLIENT_SECRET).build()
}

async fn authenticate() {
    // Create a tracing subscriber to log debug/trace events coming from
    // authorization http calls and the Graph client.
    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(true)
        .with_max_level(tracing::Level::TRACE)
        .init();

    let mut confidential_client = run_interactive_auth();

    let client = Graph::from(&confidential_client);

    let response = client.user(USER_ID)
        .get_user()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
    let body: serde_json::Value = response.json().await.unwrap();
    println!("{body:#?}");
}
```
