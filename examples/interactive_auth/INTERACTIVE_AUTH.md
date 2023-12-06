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
async fn authenticate(tenant_id: &str, client_id: &str, client_secret: &str, redirect_uri: &str) -> anyhow::Result<AuthorizationCodeCredentialBuilder> {
    let (authorization_response, credential_builder) = AuthorizationCodeCredential::authorization_url_builder(client_id)
        .with_tenant(tenant_id)
        .with_scope(vec!["user.read"])
        .with_offline_access() // Adds offline_access as a scope which is needed to get a refresh token.
        .with_redirect_uri(redirect_uri)?
        .with_interactive_auth(Default::default())?
        .into_result()?;

    println!("{authorization_response:#?}");

    Ok(credential_builder)
}
```

- Certificate

Requires `features = ["interactive-auth", "openssl"]`

`CredentialBuilder` returned is `AuthorizationCodeCertificateCredentialBuilder`

```rust
async fn authenticate(x509: &X509Certificate, tenant_id: &str, client_id: &str, client_secret: &str, redirect_uri: &str) -> anyhow::Result<AuthorizationCodeCertificateCredentialBuilder> {
    let (authorization_response, credential_builder) = AuthorizationCodeCredential::authorization_url_builder(client_id)
        .with_tenant(tenant_id)
        .with_scope(vec!["user.read"])
        .with_offline_access() // Adds offline_access as a scope which is needed to get a refresh token.
        .with_redirect_uri(redirect_uri)?
        .with_certificate_interactive_auth(Default::default(), x509)?
        .into_result()?;

    println!("{authorization_response:#?}");

    Ok(credential_builder)
}
```

- Assertion

Requires `features = ["interactive-auth"]`

`CredentialBuilder` returned is `AuthorizationCodeAssertionCredentialBuilder`

```rust
async fn authenticate(x509: &X509Certificate, tenant_id: &str, client_id: &str, client_secret: &str, redirect_uri: &str) -> anyhow::Result<AuthorizationCodeAssertionCredentialBuilder> {
    let (authorization_response, credential_builder) = AuthorizationCodeCredential::authorization_url_builder(client_id)
        .with_tenant(tenant_id)
        .with_scope(vec!["user.read"])
        .with_offline_access() // Adds offline_access as a scope which is needed to get a refresh token.
        .with_redirect_uri(redirect_uri)?
        .with_assertion_interactive_auth(Default::default())?
        .into_result()?;

    println!("{authorization_response:#?}");

    Ok(credential_builder)
}
```

### Convenience Methods

The `into_result` method transforms the `AuthorizationEvent` that is normally returned from
`with_interactive_authentication` into `(AuthorizationResponse, CredentialBuilder)`.

By default `with_interactive_authentication` returns `AuthorizationEvent<CredentialBuilder>` which can provide
the caller with useful information about the events happening with the webview such as if the user closed the window.

For those that don't necessarily care about those events use `into_result` to transform the `AuthorizationEvent`
into the credential builder that can be built and passed to the `GraphClient`.

See [Reacting To Events](#reacting-to-events) to learn more.

```rust
async fn authenticate(tenant_id: &str, client_id: &str, client_secret: &str, redirect_uri: &str) -> anyhow::Result<()> {
    let (authorization_response, credential_builder) = AuthorizationCodeCredential::authorization_url_builder(client_id)
        .with_tenant(tenant_id)
        .with_scope(vec!["user.read"])
        .with_offline_access() // Adds offline_access as a scope which is needed to get a refresh token.
        .with_redirect_uri(redirect_uri)?
        .with_interactive_authentication(Default::default())?
        .into_result()?;
    Ok(())
}
```

### WebView Options

You can customize several aspects of the webview and how the webview is used to perform interactive auth
using `WebViewOptions`.

```rust
use graph_rs_sdk::oauth::{web::Theme, web::WebViewOptions, AuthorizationCodeCredential};
use std::ops::Add;
use std::time::{Duration, Instant};

fn get_webview_options() -> WebViewOptions {
    WebViewOptions::builder()
        // Give the window a title. The default is "Sign In"
        .with_window_title("Sign In")
        // OS specific theme. Windows Only.
        // See wry crate for more info.
        .with_theme(Theme::Dark)
        // Add a timeout that will close the window and return an error
        // when that timeout is reached. For instance, if your app is waiting on the
        // user to log in and the user has not logged in after 20 minutes you may
        // want to assume the user is idle in some way and close out of the webview window.
        .with_timeout(Instant::now().add(Duration::from_secs(1200)))
        // The webview can store the cookies that were set after sign in so that on the next
        // sign in the user is automatically logged in through SSO. Or you can clear the browsing
        // data, cookies in this case, after sign in when the webview window closes.
        .with_clear_browsing_data(false)
        // Provide a list of ports to use for interactive authentication.
        // This assumes that you have http://localhost or http://localhost:port
        // for each port registered in your ADF application registration.
        .with_ports(&[8000])
}

async fn customize_webview(tenant_id: &str, client_id: &str, scope: Vec<&str>, redirect_uri: &str) -> anyhow::Result<()> {
    let (authorization_response, credential_builder) = AuthorizationCodeCredential::authorization_url_builder(client_id)
        .with_tenant(tenant_id)
        .with_scope(scope)
        .with_redirect_uri(redirect_uri)?
        .with_interactive_authentication(get_webview_options())?
        .into_result()?;
    
    Ok(())
}
```

### Reacting To Events

By default `with_interactive_authentication` returns `AuthorizationEvent<CredentialBuilder>` which can provide
the caller with useful information about the events happening with the webview such as if the user closed the window.

For those that don't necessarily care about those events use `into_result` to transform the `AuthorizationEvent`
into the credential builder that can be built and passed to the `GraphClient`.

```rust
fn authenticate(tenant_id: &str, client_id: &str, client_secret: &str, scope: Vec<&str>, redirect_uri: &str) -> anyhow::Result<GraphClient> {
    let authorization_event =
        OpenIdCredential::authorization_url_builder(client_id)
            .with_tenant(tenant_id)
            .with_scope(vec!["user.read", "offline_access", "email", "profile"])
            .with_response_mode(ResponseMode::Fragment)
            .with_response_type(vec![ResponseType::Code, ResponseType::IdToken])
            .with_redirect_uri(redirect_uri)?
            .with_interactive_auth_for_secret(Default::default())?;

    match authorization_event {
        AuthorizationEvent::Authorized { authorization_response, mut credential_builder } => {
            println!("{authorization_response:#?}");

            let mut confidential_client = credential_builder
                .with_client_secret(client_secret)
                .build();

            Ok(GraphClient::from(&confidential_client))
        }
        AuthorizationEvent::Unauthorized(authorization_response) => {
            println!("{authorization_response:#?}");
            Err(anyhow!(format!("error: {:#?}, error_description: {:#?}, error_uri: {:#?}", authorization_response.error, authorization_response.error_description, authorization_response.error_uri)))
        }
        AuthorizationEvent::Impeded(AuthorizationImpeded::WindowClosed(reason)) => {
            println!("Authorization Impeded With Reason: {reason:#?}");
            Err(anyhow!(reason))
        }
        AuthorizationEvent::Impeded(AuthorizationImpeded::InvalidUri(reason)) => {
            println!("Authorization Impeded With Reason: {reason:#?}");
            Err(anyhow!(reason))
        }
    }
}
```

The `Unauthorized` and `Impeded` variants of  `AuthorizationImpeded` useful for error handling inside an application.

1. `AuthorizationImpeded::WindowClosed(reason: String)`
   Where reason is one of:
   - CloseRequested: The user closed the window before finishing login.
   - TimedOut: A timeout was reached that you can set in `WebViewOptions`. Suppose your waiting on the user to sign
     in but the window has been idle (no user interaction) for 20 minutes. You can specify a timeout such as 20 minutes
     to close the window and  perform some other work.
   - WindowDestroyed: Either the window was destroyed or the webview event loop was destroyed causing the window
     to also be destroyed. The cause is unknown. The login did not finish.
2. `AuthorizationImpeded::InvalidUri(reason)`
    - Where reason is a message for why the URI is invalid and the URI itself.


The third variant `Authorized` means that the query or fragment of the URL was successfully parsed
from a redirect after sign in. This variant provides the parsed `AuthorizationResponse` from the
query or fragment and the `CredentialBuilder` that you can build and pass to the `GraphClient`:

```rust
fn authenticate(tenant_id: &str, client_id: &str, client_secret: &str, scope: Vec<&str>, redirect_uri: &str) -> anyhow::Result<GraphClient> {
    let authorization_event =
        OpenIdCredential::authorization_url_builder(client_id)
            .with_tenant(tenant_id)
            .with_scope(vec!["user.read", "offline_access", "email", "profile"])
            .with_response_mode(ResponseMode::Fragment)
            .with_response_type(vec![ResponseType::Code, ResponseType::IdToken])
            .with_redirect_uri(redirect_uri)?
            .with_interactive_auth_for_secret(Default::default())?;

    match authorization_event {
        AuthorizationEvent::Authorized { authorization_response, mut credential_builder } => {
            println!("{authorization_response:#?}");

            let mut confidential_client = credential_builder
                .with_client_secret(client_secret)
                .build();

            Ok(GraphClient::from(&confidential_client))
        }
        _ => Err(anyhow!("failed"))
    }
}
```

Using `into_result` transforms the `Unauthorized` and `Impeded` variants of `AuthorizationEvent`
into `WebViewError` which is then returned in the result `Result<(AuthorizationResponse, CredentialBuilder), WebViewError>`

```rust
async fn authenticate(tenant_id: &str, client_id: &str, scope: Vec<&str>, redirect_uri: &str) -> anyhow::Result<()> {
    let (authorization_response, credential_builder) = AuthorizationCodeCredential::authorization_url_builder(client_id)
        .with_tenant(tenant_id)
        .with_scope(scope)
        .with_redirect_uri(redirect_uri)?
        .with_interactive_auth_for_secret(Default::default())?
        .into_result()?;
    
    Ok(())
}
```
