use crate::{AuthExecutionError, AuthorizationFailure, ErrorMessage};
use std::sync::mpsc::RecvError;

#[derive(Debug, thiserror::Error)]
pub enum WebViewExecutionError {
    #[error("WindowClosed: {0:#?}")]
    WindowClosed(String),
    // Issues with the redirect uri such as specifying localhost
    // but not providing a port in the WebViewOptions.
    #[error("InvalidRedirectUri: {0:#?}")]
    InvalidRedirectUri(String),
    /// The host or domain provided or set for login is invalid.
    /// This could be an internal error and most likely will never happen.
    #[error("InvalidStartUri: {reason:#?}")]
    InvalidStartUri { reason: String },
    /// The webview was successfully redirected but the url did not
    /// contain a query or fragment. The query or fragment of the url
    /// is where the auth code would be returned to the app.
    #[error("No query or fragment returned on redirect uri: {0:#?}")]
    RedirectUriMissingQueryOrFragment(String),
    /// Serde serialization error when attempting to serialize
    /// the query or fragment of the url to a AuthorizationQueryResponse
    #[error("{0:#?}")]
    SerdeError(#[from] serde::de::value::Error),

    #[error("{0:#?}")]
    RecvError(#[from] RecvError),
    /// Error from building out the parameters necessary for authorization
    /// this most likely came from an invalid parameter or missing parameter
    /// passed to the client used for building the url.
    #[error("{0:#?}")]
    AuthorizationError(#[from] AuthorizationFailure),
    /// Error that happens calling the http request.
    #[error("{0:#?}")]
    AuthExecutionError(#[from] AuthExecutionError),
}

#[derive(Debug, thiserror::Error)]
pub enum WebViewDeviceCodeExecutionError {
    // Issues with the redirect uri such as specifying localhost
    // but not providing a port in the WebViewOptions.
    #[error("InvalidRedirectUri: {0:#?}")]
    InvalidRedirectUri(String),
    /// The user closed the webview window without logging in.
    #[error("WindowClosedRequested")]
    WindowClosedRequested,
    /// The user navigated to a url that was not the login url
    /// or a redirect url specified. Requires that WebViewOptions
    /// has the enforcement of invalid navigation enabled.
    #[error("WindowClosedOnInvalidNavigation")]
    WindowClosedOnInvalidNavigation,
    /// The webview exited because of a timeout defined in the WebViewOptions.
    #[error("WindowClosedOnTimeoutReached")]
    WindowClosedOnTimeoutReached,
    /// The host or domain provided or set for login is invalid.
    /// This could be an internal error and most likely will never happen.
    #[error("InvalidStartUri: {reason:#?}")]
    InvalidStartUri { reason: String },
    /// The webview was successfully redirected but the url did not
    /// contain a query or fragment. The query or fragment of the url
    /// is where the auth code would be returned to the app.
    #[error("No query or fragment returned on redirect uri: {0:#?}")]
    RedirectUriMissingQueryOrFragment(String),
    /// Serde serialization error when attempting to serialize
    /// the query or fragment of the url to a AuthorizationQueryResponse
    #[error("{0:#?}")]
    SerdeError(#[from] serde::de::value::Error),
    /// Error from building out the parameters necessary for authorization
    /// this most likely came from an invalid parameter or missing parameter
    /// passed to the client used for building the url.
    #[error("{0:#?}")]
    AuthorizationError(#[from] AuthorizationFailure),
    /// Error that happens calling the http request.
    #[error("{0:#?}")]
    AuthExecutionError(#[from] AuthExecutionError),
    #[error("{0:#?}")]
    DeviceCodeAuthFailed(http::Response<Result<serde_json::Value, ErrorMessage>>),
}
