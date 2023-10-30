use crate::web::WebViewOptions;
use graph_error::IdentityResult;
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
}

#[derive(Clone, Debug)]
pub enum InteractiveAuthEvent {
    InvalidRedirectUri(String),
    TimedOut(std::time::Duration),
    ReachedRedirectUri(Url),
    ClosingWindow(WindowCloseReason),
}
