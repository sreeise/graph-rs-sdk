//! Various errors used to implement the Graph API and OAuth clients for the graph-rs project
//! See the project on [GitHub](https://github.com/sreeise/graph-rs-sdk).
#[macro_use]
extern crate serde;

mod authorization_failure;
pub mod download;
mod error;
mod graph_failure;
mod internal;
pub mod io_error;
mod webview_error;

pub use authorization_failure::*;
pub use error::*;
pub use graph_failure::*;
pub use internal::*;
pub use webview_error::*;

pub type GraphResult<T> = Result<T, GraphFailure>;
pub type IdentityResult<T> = Result<T, AuthorizationFailure>;
pub type AuthExecutionResult<T> = Result<T, AuthExecutionError>;
pub type AuthTaskExecutionResult<T, R> = Result<T, AuthTaskExecutionError<R>>;
pub type WebViewResult<T> = Result<T, WebViewError>;
pub type DeviceCodeWebViewResult<T> = Result<T, WebViewDeviceCodeError>;
