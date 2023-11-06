use std::time::Duration;
use url::Url;

use crate::oauth::InteractiveDeviceCodeEvent;
use crate::web::{InteractiveAuthEvent, WebViewOptions, WindowCloseReason};
use graph_error::{WebViewExecutionError, WebViewResult};
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
    ) -> WebViewResult<WebViewValidHosts> {
        if start_uri.host().is_none() || redirect_uris.iter().any(|uri| uri.host().is_none()) {
            return Err(WebViewExecutionError::InvalidStartUri {
                reason: "Authorization url and redirect uri must have valid uri hosts".to_owned(),
            });
        }

        let is_local_host = redirect_uris
            .iter()
            .any(|uri| uri.as_str().eq("http://localhost"));

        if is_local_host && ports.is_empty() {
            return Err(WebViewExecutionError::InvalidStartUri {
                reason: "Redirect uri is http://localhost but not ports were specified".to_string(),
            });
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
        tracing::trace!(target: "interactive_webview", "Constructing WebView Window and EventLoop");
        let validator = WebViewValidHosts::new(uri.clone(), redirect_uris, options.ports)?;
        let event_loop: EventLoop<UserEvents> = EventLoopBuilder::with_user_event()
            .with_any_thread(true)
            .build();
        let proxy = event_loop.create_proxy();
        let sender2 = sender.clone();

        let window = WindowBuilder::new()
            .with_title(options.window_title)
            .with_closable(true)
            .with_content_protection(true)
            .with_minimizable(true)
            .with_maximizable(true)
            .with_focused(true)
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
                            .unwrap_or_default();
                        // Wait time to avoid deadlock where window closes before
                        // the channel has received the redirect uri.

                        let _ = proxy.send_event(UserEvents::ReachedRedirectUri(url));
                        return true;
                    }

                    if !is_valid_host {
                        let _ = proxy.send_event(UserEvents::InvalidNavigationAttempt(Some(url)));
                    }

                    is_valid_host
                } else {
                    tracing::debug!(target: "interactive_webview", "Unable to navigate WebView - Option<Url> was None");
                    let _ = proxy.send_event(UserEvents::CloseWindow);
                    false
                }
            })
            .build()?;

        event_loop.run(move |event, _, control_flow| {
            if let Some(timeout) = options.timeout.as_ref() {
                *control_flow = ControlFlow::WaitUntil(timeout.clone());
            } else {
                *control_flow = ControlFlow::Wait;
            }

            match event {
                Event::NewEvents(StartCause::Init) => tracing::debug!(target: "interactive_webview", "Webview runtime started"),
                Event::NewEvents(StartCause::ResumeTimeReached { start, requested_resume, .. }) => {
                    sender.send(InteractiveAuthEvent::WindowClosed(WindowCloseReason::TimedOut {
                        start, requested_resume
                    })).unwrap_or_default();
                    tracing::debug!(target: "interactive_webview", "Timeout reached - closing window");

                    if options.clear_browsing_data {
                        let _ = webview.clear_all_browsing_data();
                    }

                    // Wait time to avoid deadlock where window closes before receiver gets the event
                    std::thread::sleep(Duration::from_millis(500));
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(UserEvents::CloseWindow) | Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    sender.send(InteractiveAuthEvent::WindowClosed(WindowCloseReason::CloseRequested)).unwrap_or_default();
                    tracing::trace!(target: "interactive_webview", "Window closing before reaching redirect uri");

                    if options.clear_browsing_data {
                        let _ = webview.clear_all_browsing_data();
                    }

                    // Wait time to avoid deadlock where window closes before receiver gets the event
                    std::thread::sleep(Duration::from_millis(500));
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(UserEvents::ReachedRedirectUri(uri)) => {
                    tracing::trace!(target: "interactive_webview", "Matched on redirect uri: {uri:#?} - Closing window");

                    if options.clear_browsing_data {
                        let _ = webview.clear_all_browsing_data();
                    }

                    // Wait time to avoid deadlock where window closes before
                    // the channel has received the redirect uri.
                    std::thread::sleep(Duration::from_millis(500));
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(UserEvents::InvalidNavigationAttempt(uri_option)) => {
                    tracing::error!(target: "interactive_webview", "WebView attempted to navigate to invalid host with uri: {uri_option:#?}");
                    if options.close_window_on_invalid_uri_navigation {
                        tracing::error!(target: "interactive_webview", "Closing window due to attempted navigation to invalid host with uri: {uri_option:#?}");
                        sender.send(InteractiveAuthEvent::WindowClosed(WindowCloseReason::InvalidWindowNavigation)).unwrap_or_default();

                        if options.clear_browsing_data {
                            let _ = webview.clear_all_browsing_data();
                        }

                        // Wait time to avoid deadlock where window closes before receiver gets the event
                        std::thread::sleep(Duration::from_secs(1));

                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => (),
            }
        });
    }

    #[tracing::instrument]
    pub fn device_code_interactive_authentication(
        uri: Url,
        options: WebViewOptions,
        sender: std::sync::mpsc::Sender<InteractiveDeviceCodeEvent>,
    ) -> anyhow::Result<()> {
        tracing::trace!(target: "interactive_webview", "Constructing WebView Window and EventLoop");
        //let validator = WebViewValidHosts::new(uri.clone(), redirect_uris, options.ports)?;
        let event_loop: EventLoop<UserEvents> = EventLoopBuilder::with_user_event()
            .with_any_thread(true)
            .build();
        let proxy = event_loop.create_proxy();
        let sender2 = sender.clone();

        let window = WindowBuilder::new()
            .with_title(options.window_title)
            .with_closable(true)
            .with_content_protection(true)
            .with_minimizable(true)
            .with_maximizable(true)
            .with_focused(true)
            .with_resizable(true)
            .with_theme(options.theme)
            .build(&event_loop)?;

        let webview = WebViewBuilder::new(window)?
            .with_url(uri.as_ref())?
            // Disables file drop
            .with_file_drop_handler(|_, _| true)
            .with_navigation_handler(move |uri| {
                if let Ok(url) = Url::parse(uri.as_str()) {
                    sender2
                        .send(InteractiveDeviceCodeEvent::InteractiveAuthEvent(
                            InteractiveAuthEvent::ReachedRedirectUri(url.clone()),
                        ))
                        .unwrap_or_default();
                }
                true
            })
            .build()?;

        event_loop.run(move |event, _, control_flow| {
            if let Some(timeout) = options.timeout.as_ref() {
                *control_flow = ControlFlow::WaitUntil(timeout.clone());
            } else {
                *control_flow = ControlFlow::Wait;
            }

            match event {
                Event::NewEvents(StartCause::Init) => tracing::debug!(target: "interactive_webview", "Webview runtime started"),
                Event::NewEvents(StartCause::ResumeTimeReached { start, requested_resume, .. }) => {
                    sender.send(InteractiveDeviceCodeEvent::InteractiveAuthEvent(InteractiveAuthEvent::WindowClosed(WindowCloseReason::TimedOut {
                        start, requested_resume
                    }))).unwrap_or_default();
                    tracing::debug!(target: "interactive_webview", "Timeout reached - closing window");

                    if options.clear_browsing_data {
                        let _ = webview.clear_all_browsing_data();
                    }

                    // Wait time to avoid deadlock where window closes before receiver gets the event
                    std::thread::sleep(Duration::from_millis(500));
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(UserEvents::CloseWindow) | Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    sender.send(InteractiveDeviceCodeEvent::InteractiveAuthEvent(InteractiveAuthEvent::WindowClosed(WindowCloseReason::CloseRequested))).unwrap_or_default();
                    tracing::trace!(target: "interactive_webview", "Window closing before reaching redirect uri");

                    if options.clear_browsing_data {
                        let _ = webview.clear_all_browsing_data();
                    }

                    // Wait time to avoid deadlock where window closes before receiver gets the event
                    std::thread::sleep(Duration::from_millis(500));
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(UserEvents::ReachedRedirectUri(uri)) => {
                    tracing::trace!(target: "interactive_webview", "Matched on redirect uri: {uri:#?} - Closing window");

                    if options.clear_browsing_data {
                        let _ = webview.clear_all_browsing_data();
                    }

                    // Wait time to avoid deadlock where window closes before
                    // the channel has received the redirect uri.
                    std::thread::sleep(Duration::from_millis(500));
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(UserEvents::InvalidNavigationAttempt(uri_option)) => {
                    tracing::error!(target: "interactive_webview", "WebView attempted to navigate to invalid host with uri: {uri_option:#?}");
                    if options.close_window_on_invalid_uri_navigation {
                        tracing::error!(target: "interactive_webview", "Closing window due to attempted navigation to invalid host with uri: {uri_option:#?}");
                        sender.send(InteractiveDeviceCodeEvent::InteractiveAuthEvent(InteractiveAuthEvent::WindowClosed(WindowCloseReason::InvalidWindowNavigation))).unwrap_or_default();

                        if options.clear_browsing_data {
                            let _ = webview.clear_all_browsing_data();
                        }

                        // Wait time to avoid deadlock where window closes before receiver gets the event
                        std::thread::sleep(Duration::from_secs(1));

                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => (),
            }
        });
    }
}
