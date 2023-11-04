use graph_rs_sdk::oauth::{web::Theme, web::WebViewOptions, AuthorizationCodeCredential};
use std::ops::Add;
use std::time::{Duration, Instant};

fn get_webview_options() -> WebViewOptions {
    WebViewOptions::builder()
        .with_window_title("Sign In")
        .with_theme(Theme::Dark)
        .with_close_window_on_invalid_navigation(true)
        .with_timeout(Instant::now().add(Duration::from_secs(120)))
}

async fn customize_webview(tenant_id: &str, client_id: &str, scope: Vec<&str>, redirect_uri: &str) {
    let mut credential_builder = AuthorizationCodeCredential::authorization_url_builder(client_id)
        .with_tenant(tenant_id)
        .with_scope(scope)
        .with_offline_access() // Adds offline_access as a scope which is needed to get a refresh token.
        .with_redirect_uri(redirect_uri)
        .with_interactive_authentication(Some(get_webview_options()))
        .unwrap();
}
