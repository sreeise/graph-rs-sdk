use url::Url;

use crate::web::InteractiveWebViewOptions;
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

#[derive(Debug, Clone)]
pub enum UserEvents {
    CloseWindow,
    ReachedRedirectUri(Url),
    InvalidNavigationAttempt(Option<Url>),
}

struct WebViewValidHosts {
    start_uri: Url,
    redirect_uri: Url,
}

impl WebViewValidHosts {
    fn new(start_uri: Url, redirect_uri: Url) -> anyhow::Result<WebViewValidHosts> {
        if start_uri.host().is_none() || redirect_uri.host().is_none() {
            return Err(anyhow::Error::msg(
                "authorization url and redirect uri must have valid uri host",
            ));
        }

        Ok(WebViewValidHosts {
            start_uri,
            redirect_uri,
        })
    }

    fn is_valid_uri(&self, url: &Url) -> bool {
        if let Some(host) = url.host() {
            self.start_uri.host().eq(&Some(host.clone()))
                || self.redirect_uri.host().eq(&Some(host))
        } else {
            false
        }
    }

    fn is_redirect_host(&self, url: &Url) -> bool {
        if let Some(host) = url.host() {
            self.redirect_uri.host().eq(&Some(host))
        } else {
            false
        }
    }
}

pub struct InteractiveWebView;

impl InteractiveWebView {
    pub fn interactive_authentication(
        uri: &Url,
        redirect_uri: &Url,
        options: InteractiveWebViewOptions,
    ) -> anyhow::Result<()> {
        let event_loop: EventLoop<UserEvents> = EventLoop::<UserEvents>::with_user_event();
        let proxy = event_loop.create_proxy();

        let validator = WebViewValidHosts::new(uri.clone(), redirect_uri.clone())?;

        let window = WindowBuilder::new()
            .with_title("Sign In")
            .with_closable(true)
            .with_content_protection(true)
            .with_minimizable(true)
            .with_maximizable(true)
            .with_resizable(true)
            .with_theme(options.theme.clone())
            .build(&event_loop)?;

        let webview = WebViewBuilder::new(window)?
            .with_url(uri.as_ref())?
            // Disables file drop
            .with_file_drop_handler(|_, _| true)
            .with_navigation_handler(move |uri| {
                if let Ok(url) = Url::parse(uri.as_str()) {
                    let is_valid_host = validator.is_valid_uri(&url);
                    let is_redirect = validator.is_redirect_host(&url);

                    if is_redirect {
                        let _ = proxy.send_event(UserEvents::ReachedRedirectUri(url));
                        return true;
                    }

                    if !is_valid_host {
                        let _ = proxy.send_event(UserEvents::CloseWindow);
                    }

                    is_valid_host
                } else {
                    let _ = proxy.send_event(UserEvents::CloseWindow);
                    false
                }
            })
            .build()?;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
                Event::UserEvent(UserEvents::CloseWindow) | Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    let _ = webview.clear_all_browsing_data();
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(UserEvents::ReachedRedirectUri(uri)) => {
                    dbg!(&uri);
                    let _ = webview.clear_all_browsing_data();
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(UserEvents::InvalidNavigationAttempt(url_option)) => {
                    error!("WebView or possible attacker attempted to navigate to invalid host - closing window for security reasons. Possible url attempted: {url_option:#?}");
                    let _ = webview.clear_all_browsing_data();
                    *control_flow = ControlFlow::Exit;

                    if options.panic_on_invalid_uri_navigation_attempt {
                        panic!("WebView or possible attacker attempted to navigate to invalid host. Possible url attempted: {url_option:#?}")
                    }
                }
                _ => (),
            }
        });
    }
}
