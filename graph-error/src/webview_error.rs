use crate::{AuthExecutionError, AuthorizationFailure, ErrorMessage};

#[derive(Debug, thiserror::Error)]
pub enum WebViewError {
    /// Webview Window closed for one of the following reasons:
    /// 1. The user closed the webview window without logging in.
    /// 2. The webview exited because of a timeout defined in the WebViewOptions.
    #[error("WindowClosed: {0:#?}")]
    WindowClosed(String),

    /// One of the following errors has occurred:
    ///
    /// 1. Issues with the redirect uri such as specifying localhost
    ///    but not providing a port in the WebViewOptions.
    ///
    /// 2. The webview was successfully redirected but the url did not
    ///    contain a query or fragment. The query or fragment of the url
    ///    is where the auth code would be returned to the app.
    ///
    /// 3. The host or domain provided or set for login is invalid.
    ///    This could be an internal error and most likely will never happen.
    #[error("{0:#?}")]
    InvalidUri(String),

    /// The query or fragment of the redirect uri is an error returned
    /// from Microsoft.
    #[error("{error:#?}, {error_description:#?}, {error_uri:#?}")]
    AuthorizationQuery {
        error: String,
        error_description: String,
        error_uri: Option<String>,
    },
    /// Error that happens when building or calling the http request.
    #[error("{0:#?}")]
    AuthExecutionError(#[from] Box<AuthExecutionError>),
}

impl From<AuthorizationFailure> for WebViewError {
    fn from(value: AuthorizationFailure) -> Self {
        WebViewError::AuthExecutionError(Box::new(AuthExecutionError::Authorization(value)))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WebViewDeviceCodeError {
    /// Webview Window closed for one of the following reasons:
    /// 1. The user closed the webview window without logging in.
    /// 2. The webview exited because of a timeout defined in the WebViewOptions.
    #[error("WindowClosed: {0:#?}")]
    WindowClosed(String),
    /// Error that happens calling the http request.
    #[error("{0:#?}")]
    AuthExecutionError(#[from] AuthExecutionError),
    #[error("{0:#?}")]
    DeviceCodePollingError(http::Response<Result<serde_json::Value, ErrorMessage>>),
}
