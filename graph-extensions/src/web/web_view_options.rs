use std::time::Duration;

#[derive(Clone)]
pub struct WebViewOptions {
    pub panic_on_invalid_uri_navigation_attempt: bool,
    pub theme: Option<wry::application::window::Theme>,
    /// Provide a list of ports to use for interactive authentication.
    /// This assumes that you have http://localhost or http://localhost:port
    /// for each port registered in your ADF application registration.
    pub ports: Vec<usize>,
    pub timeout: Duration,
}

impl Default for WebViewOptions {
    fn default() -> Self {
        WebViewOptions {
            panic_on_invalid_uri_navigation_attempt: true,
            theme: None,
            ports: vec![],
            // 10 Minutes default timeout
            timeout: Duration::from_secs(10 * 60),
        }
    }
}
