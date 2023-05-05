#[derive(Clone)]
pub struct InteractiveWebViewOptions {
    pub panic_on_invalid_uri_navigation_attempt: bool,
    pub theme: Option<wry::application::window::Theme>,
    /// Provide a list of ports to use for interactive authentication.
    /// This assumes that you have http://localhost or http://localhost:port
    /// for each port registered in your ADF application registration.
    pub ports: Vec<usize>,
}

impl Default for InteractiveWebViewOptions {
    fn default() -> Self {
        InteractiveWebViewOptions {
            panic_on_invalid_uri_navigation_attempt: true,
            theme: None,
            ports: vec![],
        }
    }
}
