use std::collections::HashSet;
use std::time::Instant;
use url::Url;

pub use wry::application::window::Theme;

#[derive(Clone, Debug)]
pub struct HostOptions {
    pub(crate) start_uri: Url,
    pub(crate) redirect_uris: Vec<Url>,
    pub(crate) ports: HashSet<usize>,
}

impl HostOptions {
    pub fn new(start_uri: Url, redirect_uris: Vec<Url>, ports: HashSet<usize>) -> HostOptions {
        HostOptions {
            start_uri,
            redirect_uris,
            ports,
        }
    }
}

impl Default for HostOptions {
    fn default() -> Self {
        HostOptions {
            start_uri: Url::parse("http://localhost").expect("Internal Error"),
            redirect_uris: vec![],
            ports: vec![3000].into_iter().collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WebViewOptions {
    /// Give the window a title. The default is "Sign In"
    pub window_title: String,
    /// OS specific theme. Only available on Windows.
    /// See wry crate for more info.
    ///
    /// Theme is not set by default.
    #[cfg(windows)]
    pub theme: Option<Theme>,
    /// Provide a list of ports to use for interactive authentication.
    /// This assumes that you have http://localhost or http://localhost:port
    /// for each port registered in your ADF application registration.
    pub ports: HashSet<usize>,
    /// Add a timeout that will close the window and return an error
    /// when that timeout is reached. For instance, if your app is waiting on the
    /// user to log in and the user has not logged in after 20 minutes you may
    /// want to assume the user is idle in some way and close out of the webview window.
    ///
    /// Default is no timeout.
    pub timeout: Option<Instant>,
    /// The webview can store the cookies that were set after sign in so that on the next
    /// sign in the user is automatically logged in through SSO. Or you can clear the browsing
    /// data, cookies in this case, after sign in when the webview window closes.
    ///
    /// Default is true
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

    /// OS specific theme. Only available on Windows.
    /// See wry crate for more info.
    #[cfg(windows)]
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn with_ports(mut self, ports: HashSet<usize>) -> Self {
        self.ports = ports;
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

#[cfg(windows)]
impl Default for WebViewOptions {
    fn default() -> Self {
        WebViewOptions {
            window_title: "Sign In".to_string(),
            theme: None,
            ports: Default::default(),
            timeout: None,
            clear_browsing_data: true,
        }
    }
}

#[cfg(unix)]
impl Default for WebViewOptions {
    fn default() -> Self {
        WebViewOptions {
            window_title: "Sign In".to_string(),
            ports: Default::default(),
            timeout: None,
            clear_browsing_data: true,
        }
    }
}
