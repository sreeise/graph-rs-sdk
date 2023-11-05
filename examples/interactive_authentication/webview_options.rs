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
