# Interactive Authentication

Interactive Authentication uses a webview to perform sign in and handle the redirect
uri making it easy for you to integrate the sdk into your application.

Interactive Authentication uses a webview provided by the Wry crate https://github.com/tauri-apps/wry
See the wry documentation for platform specific installation. Linux and macOS require
installation of platform specific dependencies. These are not included by default.

The examples below executes the Authorization Code OAuth flow and handles
sign in/redirect using WebView as well as execution of the token requests.

The WebView window will load on the sign in page for Microsoft Graph.
You can Log in with a user and upon redirect the WebView Window will close automatically.

The `CredentialBuilder` that is returned stores the authorization code returned on the
redirect url after logging in. You can use the `CredentialBuilder` to build a
`ConfidentialClient<AuthorizationCodeCredential>` which can be passed to the `GraphClient`

The `ConfidentialClient<AuthorizationCodeCredential>` handles authorization to get an access token
on the first request made using the Graph client. The token is stored in an in memory cache
and subsequent calls will use this token. If a refresh token is included, which you can get
by requesting the offline_access scope, then the confidential client will take care of refreshing
the token.

The Auth Code Grant can be performed using a client secret, a certificate, or an assertion.

- Client Secret: 

Requires `features = ["interactive-auth"]`

`CredentialBuilder` returned is `AuthorizationCodeCredentialBuilder`

```rust
async fn authenticate(tenant_id: &str, client_id: &str, client_secret: &str, redirect_uri: url::Url) -> anyhow::Result<AuthorizationCodeCredentialBuilder> {
    let (authorization_response, credential_builder) = AuthorizationCodeCredential::authorization_url_builder(client_id)
        .with_tenant(tenant_id)
        .with_scope(vec!["user.read"])
        .with_offline_access() // Adds offline_access as a scope which is needed to get a refresh token.
        .with_redirect_uri(redirect_uri)
        .with_interactive_auth(Secret(client_secret.to_string()), Default::default())
        .into_credential_builder()?;

    println!("{authorization_response:#?}");

    Ok(credential_builder)
}
```

- Certificate

Requires `features = ["interactive-auth", "openssl"]`

`CredentialBuilder` returned is `AuthorizationCodeCertificateCredentialBuilder`

```rust
async fn authenticate(x509: &X509Certificate, tenant_id: &str, client_id: &str, client_secret: &str, redirect_uri: url::Url) -> anyhow::Result<AuthorizationCodeCertificateCredentialBuilder> {
    let (authorization_response, credential_builder) = AuthorizationCodeCredential::authorization_url_builder(client_id)
        .with_tenant(tenant_id)
        .with_scope(vec!["user.read"])
        .with_offline_access() // Adds offline_access as a scope which is needed to get a refresh token.
        .with_redirect_uri(redirect_uri)
        .with_certificate_interactive_auth(x509, Default::default())
        .into_credential_builder()?;

    println!("{authorization_response:#?}");

    Ok(credential_builder)
}
```

### Convenience Methods

The `into_credential_builder` method maps the `WebViewAuthorizationEvent` and `Result<WebViewAuthorizationEvent>`
that is normally returned from `with_interactive_auth` into `(AuthorizationResponse, CredentialBuilder)`
and `Result<(AuthorizationResponse, CredentialBuilder)>` respectively.

By default `with_interactive_auth` returns `AuthorizationEvent<CredentialBuilder>` which can provide
the caller with useful information about the events happening with the webview such as if the user closed the window.

For those that don't necessarily care about those events use `into_result` to transform the `AuthorizationEvent`
into the credential builder that can be built and passed to the `GraphClient`.

See [Reacting To Events](#reacting-to-events) to learn more.

```rust
async fn authenticate(tenant_id: &str, client_id: &str, client_secret: &str, redirect_uri: url::Url) -> anyhow::Result<()> {
    let (authorization_response, credential_builder) = AuthorizationCodeCredential::authorization_url_builder(client_id)
        .with_tenant(tenant_id)
        .with_scope(vec!["user.read"])
        .with_offline_access() // Adds offline_access as a scope which is needed to get a refresh token.
        .with_redirect_uri(redirect_uri)
        .with_interactive_auth(Secret(client_secret.to_string()), Default::default())
        .into_credential_builder()?;
    Ok(())
}
```

### WebView Options

You can customize several aspects of the webview and how the webview is used to perform interactive auth
using `WebViewOptions`.

```rust
use graph_rs_sdk::identity::{
    interactive::Theme, interactive::WebViewOptions, interactive::WithInteractiveAuth,
    AuthorizationCodeCredential, IntoCredentialBuilder, Secret,
};
use graph_rs_sdk::GraphClient;
use std::collections::HashSet;
use std::ops::Add;
use std::time::{Duration, Instant};
use url::Url;

fn get_webview_options() -> WebViewOptions {
    WebViewOptions::builder()
        // Give the window a title. The default is "Sign In"
        .window_title("Sign In")
        // OS specific theme. Windows only.
        // See wry crate for more info.
        .theme(Theme::Dark)
        // Add a timeout that will close the window and return an error
        // when that timeout is reached. For instance, if your app is waiting on the
        // user to log in and the user has not logged in after 20 minutes you may
        // want to assume the user is idle in some way and close out of the webview window.
        .timeout(Instant::now().add(Duration::from_secs(1200)))
        // The webview can store the cookies that were set after sign in so that on the next
        // sign in the user is automatically logged in through SSO. Or you can clear the browsing
        // data, cookies in this case, after sign in when the webview window closes.
        // Default is false.
        // When using webview and the user is automatically logged in the webview
        // will only show temporarily and then close itself.
        .clear_browsing_data_on_close(true)
        // Provide a list of ports to use for interactive authentication.
        // This assumes that you have http://localhost or http://localhost:port
        // for each port registered in your ADF application registration.
        .ports(HashSet::from([8000]))
}

async fn customize_webview(
    tenant_id: &str,
    client_id: &str,
    client_secret: &str,
    scope: Vec<&str>,
    redirect_uri: &str,
) -> anyhow::Result<GraphClient> {
    let (authorization_response, mut credential_builder) =
        AuthorizationCodeCredential::authorization_url_builder(client_id)
            .with_tenant(tenant_id)
            .with_scope(scope)
            .with_redirect_uri(Url::parse(redirect_uri)?)
            .with_interactive_auth(Secret(client_secret.to_string()), get_webview_options())
            .into_credential_builder()?;

    let confidential_client = credential_builder.build();

    Ok(GraphClient::from(&confidential_client))
}

```
