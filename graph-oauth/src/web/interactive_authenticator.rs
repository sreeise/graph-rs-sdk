use crate::web::WebViewOptions;
use graph_error::IdentityResult;
use std::time::Instant;
use url::Url;

pub trait InteractiveAuthenticator {
    fn interactive_authentication(
        &self,
        interactive_web_view_options: Option<WebViewOptions>,
    ) -> IdentityResult<std::sync::mpsc::Receiver<InteractiveAuthEvent>>;
}

#[derive(Clone, Debug)]
pub enum WindowCloseReason {
    CloseRequested,
    InvalidWindowNavigation,
    TimedOut {
        start: Instant,
        requested_resume: Instant,
    },
}

#[derive(Clone, Debug)]
pub enum InteractiveAuthEvent {
    InvalidRedirectUri(String),
    ReachedRedirectUri(Url),
    ClosingWindow(WindowCloseReason),
}
