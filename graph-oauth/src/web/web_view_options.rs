use std::ops::Add;
use std::time::{Duration, Instant};

pub use wry::application::window::Theme;

#[derive(Clone, Debug)]
pub struct WebViewOptions {
    pub window_title: String,
    // Close window if navigation to a uri that does not match one of the
    // given redirect uri's.
    pub close_window_on_invalid_uri_navigation: bool,
    pub theme: Option<Theme>,
    /// Provide a list of ports to use for interactive authentication.
    /// This assumes that you have http://localhost or http://localhost:port
    /// for each port registered in your ADF application registration.
    pub ports: Vec<usize>,
    pub timeout: Instant,
    pub clear_browsing_data: bool,
}

impl WebViewOptions {
    pub fn builder() -> WebViewOptions {
        WebViewOptions::default()
    }

    pub fn with_window_title(mut self, window_title: impl ToString) -> Self {
        self.window_title = window_title.to_string();
        self
    }

    pub fn with_close_window_on_invalid_navigation(mut self, close_window: bool) -> Self {
        self.close_window_on_invalid_uri_navigation = close_window;
        self
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn with_ports(mut self, ports: &[usize]) -> Self {
        self.ports = ports.to_vec();
        self
    }

    pub fn with_timeout(mut self, instant: Instant) -> Self {
        self.timeout = instant;
        self
    }

    pub fn with_clear_browsing_data(mut self, clear_browsing_data: bool) -> Self {
        self.clear_browsing_data = clear_browsing_data;
        self
    }
}

impl Default for WebViewOptions {
    fn default() -> Self {
        WebViewOptions {
            window_title: "Sign In".to_string(),
            close_window_on_invalid_uri_navigation: true,
            theme: None,
            ports: vec![],
            // 10 Minutes default timeout
            timeout: Instant::now().add(Duration::from_secs(10 * 60)),
            clear_browsing_data: false,
        }
    }
}
