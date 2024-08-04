use graph_rs_sdk::identity::{
    interactive::Theme, interactive::WebViewOptions, interactive::WithInteractiveAuth,
    AuthorizationCodeCredential, IntoCredentialBuilder, Secret,
};
use graph_rs_sdk::GraphClient;
use std::collections::HashSet;
use std::ops::Add;
use std::time::{Duration, Instant};
use url::Url;

#[cfg(windows)]
fn get_webview_options() -> WebViewOptions {
    WebViewOptions::builder()
        // Give the window a title. The default is "Sign In"
        .window_title("Sign In")
        // OS specific theme. Windows only.
        // See Tao crate for more info.
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

#[cfg(unix)]
fn get_webview_options() -> WebViewOptions {
    WebViewOptions::builder()
        // Give the window a title. The default is "Sign In"
        .window_title("Sign In")
        // Add a timeout that will close the window and return an error
        // when that timeout is reached. For instance, if your app is waiting on the
        // user to log in and the user has not logged in after 20 minutes you may
        // want to assume the user is idle in some way and close out of the webview window.
        .timeout(Instant::now().add(Duration::from_secs(1200)))
        // The webview can store the cookies that were set after sign in so that on the next
        // sign in the user is automatically logged in through SSO. Or you can clear the browsing
        // data, cookies in this case, after sign in when the webview window closes.
        .clear_browsing_data_on_close(false)
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
