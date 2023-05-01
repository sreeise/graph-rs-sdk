#[derive(Clone)]
pub struct InteractiveWebViewOptions {
    pub panic_on_invalid_uri_navigation_attempt: bool,
    pub theme: Option<wry::application::window::Theme>,
}

impl Default for InteractiveWebViewOptions {
    fn default() -> Self {
        InteractiveWebViewOptions {
            panic_on_invalid_uri_navigation_attempt: true,
            theme: None,
        }
    }
}
