use crate::interactive::{WebViewAuthorizationEvent, WebViewOptions};
use graph_error::WebViewResult;
use std::fmt::Debug;

pub trait WithInteractiveAuth<T> {
    type CredentialBuilder: Clone + Debug;

    fn with_interactive_auth(
        &self,
        auth_type: T,
        options: WebViewOptions,
    ) -> WebViewResult<WebViewAuthorizationEvent<Self::CredentialBuilder>>;
}
