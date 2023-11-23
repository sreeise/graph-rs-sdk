use crate::web::{HostOptions, UserEvents, WebViewOptions};
use graph_error::WebViewResult;
use std::fmt::{Debug, Display, Formatter};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};
use url::Url;
use wry::application::event::{Event, StartCause, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
use wry::application::window::{Window, WindowBuilder};
use wry::webview::WebView;

#[cfg(target_family = "windows")]
use wry::application::platform::windows::EventLoopBuilderExtWindows;

pub trait InteractiveAuthenticator {
    fn interactive_authentication(
        &self,
        interactive_web_view_options: Option<WebViewOptions>,
    ) -> WebViewResult<std::sync::mpsc::Receiver<InteractiveAuthEvent>>;
}

pub trait InteractiveAuth
where
    Self: Debug,
{
    fn webview(
        host_options: HostOptions,
        window: Window,
        proxy: EventLoopProxy<UserEvents>,
    ) -> anyhow::Result<WebView>;

    fn interactive_auth(
        start_url: Url,
        redirect_uris: Vec<Url>,
        options: WebViewOptions,
        sender: Sender<InteractiveAuthEvent>,
    ) -> anyhow::Result<()> {
        let event_loop: EventLoop<UserEvents> = Self::event_loop();
        let proxy = event_loop.create_proxy();
        let window = Self::window_builder(&options).build(&event_loop).unwrap();
        let host_options = HostOptions::new(start_url, redirect_uris, options.ports.clone());
        let webview = Self::webview(host_options, window, proxy)?;

        event_loop.run(move |event, _, control_flow| {
            if let Some(timeout) = options.timeout.as_ref() {
                *control_flow = ControlFlow::WaitUntil(*timeout);
            } else {
                *control_flow = ControlFlow::Wait;
            }

            match event {
                Event::NewEvents(StartCause::Init) => tracing::trace!(target: "graph_rs_sdk::interactive_auth", "Webview runtime started"),
                Event::NewEvents(StartCause::ResumeTimeReached { start, requested_resume, .. }) => {
                    sender.send(InteractiveAuthEvent::WindowClosed(WindowCloseReason::TimedOut {
                        start, requested_resume
                    })).unwrap_or_default();
                    tracing::debug!(target: "graph_rs_sdk::interactive_auth", "Timeout reached - closing window");

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
                    tracing::trace!(target: "graph_rs_sdk::interactive_auth", "Window close requested by user");

                    if options.clear_browsing_data {
                        let _ = webview.clear_all_browsing_data();
                    }

                    // Wait time to avoid deadlock where window closes before receiver gets the event
                    std::thread::sleep(Duration::from_millis(500));
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(UserEvents::ReachedRedirectUri(uri)) => {
                    tracing::trace!(target: "graph_rs_sdk::interactive_auth", "Matched on redirect uri: {uri}");
                    sender.send(InteractiveAuthEvent::ReachedRedirectUri(uri))
                        .unwrap_or_default();
                }
                Event::UserEvent(UserEvents::InternalCloseWindow) => {
                    tracing::trace!(target: "graph_rs_sdk::interactive_auth", "Closing window");
                    if options.clear_browsing_data {
                        let _ = webview.clear_all_browsing_data();
                    }

                    // Wait time to avoid deadlock where window closes before
                    // the channel has received the redirect uri.
                    std::thread::sleep(Duration::from_millis(500));
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            }
        });
    }

    #[cfg(target_family = "windows")]
    fn window_builder(options: &WebViewOptions) -> WindowBuilder {
        WindowBuilder::new()
            .with_title(options.window_title.clone())
            .with_closable(true)
            .with_content_protection(true)
            .with_minimizable(true)
            .with_maximizable(true)
            .with_focused(true)
            .with_resizable(true)
            .with_theme(options.theme)
    }

    #[cfg(target_family = "unix")]
    fn window_builder(options: &WebViewOptions) -> WindowBuilder {
        WindowBuilder::new()
            .with_title(options.window_title.clone())
            .with_closable(true)
            .with_content_protection(true)
            .with_minimizable(true)
            .with_maximizable(true)
            .with_focused(true)
            .with_resizable(true)
    }

    #[cfg(target_family = "windows")]
    fn event_loop() -> EventLoop<UserEvents> {
        EventLoopBuilder::with_user_event()
            .with_any_thread(true)
            .build()
    }

    #[cfg(target_family = "unix")]
    fn event_loop() -> EventLoop<UserEvents> {
        EventLoopBuilder::with_user_event().build()
    }
}

#[derive(Clone, Debug)]
pub enum WindowCloseReason {
    CloseRequested,
    TimedOut {
        start: Instant,
        requested_resume: Instant,
    },
}

impl Display for WindowCloseReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowCloseReason::CloseRequested => write!(f, "CloseRequested"),
            WindowCloseReason::TimedOut { .. } => write!(f, "TimedOut"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum InteractiveAuthEvent {
    InvalidRedirectUri(String),
    ReachedRedirectUri(Url),
    WindowClosed(WindowCloseReason),
}
