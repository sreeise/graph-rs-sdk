use std::ops::Add;
use std::time::{Duration, Instant};

pub use wry::application::window::Theme;

#[derive(Clone, Debug)]
pub struct WebViewOptions {
    /// Give the window a title. The default is "Sign In"
    pub window_title: String,
    /// Close the webview window whenever there is a navigation by the webview or user
    /// to a url that is not one of the redirect urls or the login url.
    /// For instance, if this is considered a security issue and the user should
    /// not be able to navigate to another url.
    /// Either way, the url bar does not show regardless.
    pub close_window_on_invalid_uri_navigation: bool,
    /// OS specific theme. Does not work on all operating systems.
    /// See wry crate for more info.
    pub theme: Option<Theme>,
    /// Provide a list of ports to use for interactive authentication.
    /// This assumes that you have http://localhost or http://localhost:port
    /// for each port registered in your ADF application registration.
    pub ports: Vec<usize>,
    /// Add a timeout that will close the window and return an error
    /// when that timeout is reached. For instance, if your app is waiting on the
    /// user to log in and the user has not logged in after 20 minutes you may
    /// want to assume the user is idle in some way and close out of the webview window.
    pub timeout: Option<Instant>,
    /// The webview can store the cookies that were set after sign in so that on the next
    /// sign in the user is automatically logged in through SSO. Or you can clear the browsing
    /// data, cookies in this case, after sign in when the webview window closes.
    pub clear_browsing_data: bool,
}

impl WebViewOptions {
    pub fn builder() -> WebViewOptions {
        WebViewOptions::default()
    }

    /// Give the window a title. The default is "Sign In"
    pub fn with_window_title(mut self, window_title: impl ToString) -> Self {
        self.window_title = window_title.to_string();
        self
    }

    /// Close the webview window whenever there is a navigation by the webview or user
    /// to a url that is not one of the redirect urls or the login url.
    /// For instance, if this is considered a security issue and the user should
    /// not be able to navigate to another url.
    /// Either way, the url bar does not show regardless.
    pub fn with_close_window_on_invalid_navigation(mut self, close_window: bool) -> Self {
        self.close_window_on_invalid_uri_navigation = close_window;
        self
    }

    /// OS specific theme. Does not work on all operating systems.
    /// See wry crate for more info.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn with_ports(mut self, ports: &[usize]) -> Self {
        self.ports = ports.to_vec();
        self
    }

    /// Add a timeout that will close the window and return an error
    /// when that timeout is reached. For instance, if your app is waiting on the
    /// user to log in and the user has not logged in after 20 minutes you may
    /// want to assume the user is idle in some way and close out of the webview window.
    pub fn with_timeout(mut self, instant: Instant) -> Self {
        self.timeout = Some(instant);
        self
    }

    /// The webview can store the cookies that were set after sign in so that on the next
    /// sign in the user is automatically logged in through SSO. Or you can clear the browsing
    /// data, cookies in this case, after sign in when the webview window closes.
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
            timeout: None,
            clear_browsing_data: false,
        }
    }
}
