use anyhow::Context;
use std::time::Duration;
use url::Url;

use crate::web::{InteractiveAuthEvent, WebViewOptions, WindowCloseReason};
use wry::application::event_loop::EventLoopBuilder;
use wry::application::platform::windows::EventLoopBuilderExtWindows;
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
    redirect_uris: Vec<Url>,
    ports: Vec<usize>,
    is_local_host: bool,
}

impl WebViewValidHosts {
    fn new(
        start_uri: Url,
        redirect_uris: Vec<Url>,
        ports: Vec<usize>,
    ) -> anyhow::Result<WebViewValidHosts> {
        if start_uri.host().is_none() || redirect_uris.iter().any(|uri| uri.host().is_none()) {
            return Err(anyhow::Error::msg(
                "authorization url and redirect uri must have valid uri host",
            ));
        }

        let is_local_host = redirect_uris
            .iter()
            .any(|uri| uri.as_str().eq("http://localhost"));

        if is_local_host && ports.is_empty() {
            return Err(anyhow::anyhow!(
                "Redirect uri is http://localhost but not ports were specified".to_string()
            ));
        }

        Ok(WebViewValidHosts {
            start_uri,
            redirect_uris,
            ports,
            is_local_host,
        })
    }

    fn is_valid_uri(&self, url: &Url) -> bool {
        if let Some(host) = url.host() {
            if self.is_local_host && !self.ports.is_empty() {
                let hosts: Vec<url::Host> = self
                    .redirect_uris
                    .iter()
                    .map(|port| url::Host::parse(&format!("http://localhost:{}", port)).unwrap())
                    .collect();

                for redirect_uri in self.redirect_uris.iter() {
                    if let Some(redirect_uri_host) = redirect_uri.host() {
                        if hosts.iter().any(|host| host.eq(&redirect_uri_host)) {
                            return true;
                        }
                    }
                }
            }

            self.start_uri.host().eq(&Some(host.clone()))
                || self
                    .redirect_uris
                    .iter()
                    .any(|uri| uri.host().eq(&Some(host.clone())))
        } else {
            false
        }
    }

    fn is_redirect_host(&self, url: &Url) -> bool {
        if let Some(host) = url.host() {
            self.redirect_uris
                .iter()
                .any(|uri| uri.host().eq(&Some(host.clone())))
        } else {
            false
        }
    }
}

pub struct InteractiveWebView;

impl InteractiveWebView {
    #[tracing::instrument]
    pub fn interactive_authentication(
        uri: Url,
        redirect_uris: Vec<Url>,
        options: WebViewOptions,
        sender: std::sync::mpsc::Sender<InteractiveAuthEvent>,
    ) -> anyhow::Result<()> {
        let validator = WebViewValidHosts::new(uri.clone(), redirect_uris, options.ports)?;
        let event_loop: EventLoop<UserEvents> = EventLoopBuilder::with_user_event()
            .with_any_thread(true)
            .build();
        let proxy = event_loop.create_proxy();
        let sender2 = sender.clone();

        let window = WindowBuilder::new()
            .with_title("Sign In")
            .with_closable(true)
            .with_content_protection(true)
            .with_minimizable(true)
            .with_maximizable(true)
            .with_resizable(true)
            .with_theme(options.theme)
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
                        sender2
                            .send(InteractiveAuthEvent::ReachedRedirectUri(url.clone()))
                            .context("mpsc error")
                            .unwrap();
                        // Wait time to avoid deadlock where window closes before
                        // the channel has received the redirect uri.
                        std::thread::sleep(Duration::from_secs(1));
                        let _ = proxy.send_event(UserEvents::ReachedRedirectUri(url));
                        return true;
                    }

                    if !is_valid_host {
                        let _ = proxy.send_event(UserEvents::InvalidNavigationAttempt(Some(url)));
                    }

                    is_valid_host
                } else {
                    tracing::info!("Unable to navigate WebView - Option<Url> was None");
                    let _ = proxy.send_event(UserEvents::CloseWindow);
                    false
                }
            })
            .build()?;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => tracing::info!("Webview runtime started"),
                Event::UserEvent(UserEvents::CloseWindow) | Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    tracing::info!("Window closing without reaching redirect uri");
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(UserEvents::ReachedRedirectUri(uri)) => {
                    tracing::info!("Matched on redirect uri: {uri:#?}");
                    tracing::info!("Closing window");
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(UserEvents::InvalidNavigationAttempt(uri_option)) => {
                    tracing::error!("WebView attempted to navigate to invalid host with uri: {uri_option:#?}");
                    if options.close_window_on_invalid_uri_navigation {
                        tracing::error!("Closing window due to attempted navigation to invalid host with uri: {uri_option:#?}");
                        sender.send(InteractiveAuthEvent::ClosingWindow(WindowCloseReason::InvalidWindowNavigation)).unwrap();
                        // Wait time to avoid deadlock where window closes before
                        // the channel has received last event.
                        std::thread::sleep(Duration::from_secs(1));
                        let _ = webview.clear_all_browsing_data();
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => (),
            }
        });
    }
}
