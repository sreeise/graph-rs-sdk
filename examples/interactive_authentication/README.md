# Interactive Authentication

Interactive Authentication uses a webview to perform sign in and handle the redirect
uri making it easy for you to integrate the sdk into your application.

Interactive Authentication uses a webview provided by the Wry crate https://github.com/tauri-apps/wry
See the wry documentation for platform specific installation. Linux and macOS require
installation of platform specific dependencies. These are not included by default.

This example executes the Authorization Code OAuth flow and handles
sign in/redirect using WebView as well as authorization and token retrieval.

The WebView window will load on the sign in page for Microsoft Graph
Log in with a user and upon redirect the will close the window automatically.
The credential_builder will store the authorization code returned on the
redirect url after logging in and then build a `ConfidentialClient<AuthorizationCodeCredential>`

The `ConfidentialClient<AuthorizationCodeCredential>` handles authorization to get an access token
on the first request made using the Graph client. The token is stored in an in memory cache
and subsequent calls will use this token. If a refresh token is included, which you can get
by requesting the offline_access scope, then the confidential client will take care of refreshing
the token.

### Example

```rust
static CLIENT_ID: &str = "CLIENT_ID";
static CLIENT_SECRET: &str = "CLIENT_SECRET";
static TENANT_ID: &str = "TENANT_ID";

// This should be the user id for the user you are logging in as.
static USER_ID: &str = "USER_ID";

static REDIRECT_URI: &str = "http://localhost:8000/redirect";

async fn authenticate() {
    // Create a tracing subscriber to log debug/trace events coming from
    // authorization http calls and the Graph client.
    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(true)
        .with_max_level(tracing::Level::TRACE)
        .init();

    let mut credential_builder = AuthorizationCodeCredential::authorization_url_builder(CLIENT_ID)
        .with_tenant(TENANT_ID)
        .with_scope(vec!["user.read"])
        .with_offline_access() // Adds offline_access as a scope which is needed to get a refresh token.
        .with_redirect_uri(REDIRECT_URI)
        .with_interactive_authentication(None)
        .unwrap();

    let mut confidential_client = credential_builder.with_client_secret(CLIENT_SECRET).build();

    let client = GraphClient::from(&confidential_client);

    let response = client.user(USER_ID).get_user().send().await.unwrap();

    println!("{response:#?}");
    let body: serde_json::Value = response.json().await.unwrap();
    println!("{body:#?}");
}

```


### WebView Options

You can customize several aspects of the webview including security mechanisms
or setting an OS theme.

```rust
use graph_rs_sdk::oauth::{web::Theme, web::WebViewOptions, AuthorizationCodeCredential};
use std::ops::Add;
use std::time::{Duration, Instant};

fn get_webview_options() -> WebViewOptions {
    WebViewOptions::builder()
        // Give the window a title. The default is "Sign In"
        .with_window_title("Sign In")
        // OS specific theme. Does not work on all operating systems.
        // See wry crate for more info.
        .with_theme(Theme::Dark)
        // Close the webview window whenever there is a navigation by the webview or user
        // to a url that is not one of the redirect urls or the login url. 
        // For instance, if this is considered a security issue and the user should
        // not be able to navigate to another url. 
        // Either way, the url bar does not show regardless.
        .with_close_window_on_invalid_navigation(true)
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

async fn customize_webview(tenant_id: &str, client_id: &str, scope: Vec<&str>, redirect_uri: &str) {
    let mut credential_builder = AuthorizationCodeCredential::authorization_url_builder(client_id)
        .with_tenant(tenant_id)
        .with_scope(scope)
        .with_redirect_uri(redirect_uri)
        .with_interactive_authentication(Some(get_webview_options()))
        .unwrap();
}
```
