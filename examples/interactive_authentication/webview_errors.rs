use graph_rs_sdk::{error::WebViewError, oauth::AuthorizationCodeCredential};

async fn interactive_auth(tenant_id: &str, client_id: &str, scope: Vec<&str>, redirect_uri: &str) {
    let mut credential_builder_result =
        AuthorizationCodeCredential::authorization_url_builder(client_id)
            .with_tenant(tenant_id)
            .with_scope(scope)
            .with_redirect_uri(redirect_uri)
            .with_interactive_authentication_for_secret(None);

    if let Ok((authorization_query_response, credential_builder)) = credential_builder_result {
        // ...
    } else if let Err(err) = credential_builder_result {
        match err {
            // Webview Window closed for one of the following reasons:
            // 1. The user closed the webview window without logging in.
            // 2. The webview exited because of a timeout defined in the WebViewOptions.
            //
            // Values will be one of:
            // 1. CloseRequested: User closed the window before completing sign in and redirect.
            // 2. TimedOut: The timeout specified in WebViewOptions was reached. By default there
            // is no timeout.
            WebViewError::WindowClosed(reason) => {}

            // One of the following errors has occurred:
            //
            // 1. Issues with the redirect uri such as specifying localhost
            //    but not providing a port in the WebViewOptions.
            //
            // 2. The webview was successfully redirected but the url did not
            //    contain a query or fragment. The query or fragment of the url
            //    is where the auth code would be returned to the app.
            //
            // 3. The host or domain provided or set for login is invalid.
            //    This could be an internal error and most likely will never happen.
            WebViewError::InvalidUri(reason) => {}

            // The query or fragment of the redirect uri is an error returned
            // from Microsoft.
            WebViewError::AuthorizationQuery {
                error,
                error_description,
                error_uri,
            } => {}

            WebViewError::AuthExecutionError(_) => {}
        }
    }
}
