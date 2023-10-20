use std::time::Duration;

#[derive(Clone, Debug)]
pub struct WebViewOptions {
    // Close window if navigation to a uri that does not match one of the
    // given redirect uri's.
    pub close_window_on_invalid_uri_navigation: bool,
    pub theme: Option<wry::application::window::Theme>,
    /// Provide a list of ports to use for interactive authentication.
    /// This assumes that you have http://localhost or http://localhost:port
    /// for each port registered in your ADF application registration.
    pub ports: Vec<usize>,
    pub timeout: Duration,
    pub clear_browsing_data: bool,
}

impl Default for WebViewOptions {
    fn default() -> Self {
        WebViewOptions {
            close_window_on_invalid_uri_navigation: true,
            theme: None,
            ports: vec![],
            // 10 Minutes default timeout
            timeout: Duration::from_secs(10 * 60),
            clear_browsing_data: false,
        }
    }
}
