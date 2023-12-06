use crate::identity::tracing_targets::INTERACTIVE_AUTH;
use crate::web::{HostOptions, WebViewOptions};
use std::fmt::{Debug, Display, Formatter};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};
use url::Url;
use wry::application::event::{Event, StartCause, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
use wry::application::window::{Window, WindowBuilder};
use wry::webview::WebView;

#[cfg(target_family = "unix")]
use wry::application::platform::unix::EventLoopBuilderExtUnix;

#[cfg(target_family = "windows")]
use wry::application::platform::windows::EventLoopBuilderExtWindows;

#[derive(Clone, Debug)]
pub enum WindowCloseReason {
    CloseRequested,
    TimedOut {
        start: Instant,
        requested_resume: Instant,
    },
    WindowDestroyed,
}

impl Display for WindowCloseReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowCloseReason::CloseRequested => write!(f, "CloseRequested"),
            WindowCloseReason::TimedOut { .. } => write!(f, "TimedOut"),
            WindowCloseReason::WindowDestroyed => write!(f, "WindowDestroyed"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum InteractiveAuthEvent {
    InvalidRedirectUri(String),
    ReachedRedirectUri(Url),
    WindowClosed(WindowCloseReason),
}

#[derive(Debug, Clone)]
pub enum UserEvents {
    CloseWindow,
    InternalCloseWindow,
    ReachedRedirectUri(Url),
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
                Event::NewEvents(StartCause::Init) => {
                    tracing::debug!(target: INTERACTIVE_AUTH, "webview runtime started")
                }
                Event::NewEvents(StartCause::ResumeTimeReached {
                    start,
                    requested_resume,
                    ..
                }) => {
                    sender
                        .send(InteractiveAuthEvent::WindowClosed(
                            WindowCloseReason::TimedOut {
                                start,
                                requested_resume,
                            },
                        ))
                        .unwrap_or_default();
                    tracing::debug!(target: INTERACTIVE_AUTH, "timeout reached - closing window");

                    if options.clear_browsing_data {
                        let _ = webview.clear_all_browsing_data();
                    }

                    // Wait time to avoid deadlock where window closes before receiver gets the event
                    std::thread::sleep(Duration::from_millis(500));
                    *control_flow = ControlFlow::Exit
                }
                Event::LoopDestroyed
                | Event::WindowEvent {
                    event: WindowEvent::Destroyed,
                    ..
                } => {
                    tracing::debug!(target: INTERACTIVE_AUTH, "window destroyed");
                    sender
                        .send(InteractiveAuthEvent::WindowClosed(
                            WindowCloseReason::WindowDestroyed,
                        ))
                        .unwrap_or_default();

                    // Wait time to avoid deadlock where window closes before receiver gets the event
                    std::thread::sleep(Duration::from_millis(500));
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(UserEvents::CloseWindow)
                | Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    tracing::debug!(target: INTERACTIVE_AUTH, "window close requested by user");
                    sender
                        .send(InteractiveAuthEvent::WindowClosed(
                            WindowCloseReason::CloseRequested,
                        ))
                        .unwrap_or_default();

                    if options.clear_browsing_data {
                        let _ = webview.clear_all_browsing_data();
                    }

                    // Wait time to avoid deadlock where window closes before receiver gets the event
                    std::thread::sleep(Duration::from_millis(500));
                    *control_flow = ControlFlow::Exit
                }
                Event::UserEvent(UserEvents::ReachedRedirectUri(uri)) => {
                    tracing::debug!(target: INTERACTIVE_AUTH, "matched on redirect uri: {uri}");
                    sender
                        .send(InteractiveAuthEvent::ReachedRedirectUri(uri))
                        .unwrap_or_default();
                }
                Event::UserEvent(UserEvents::InternalCloseWindow) => {
                    tracing::debug!(target: INTERACTIVE_AUTH, "closing window");
                    if options.clear_browsing_data {
                        tracing::debug!(target: INTERACTIVE_AUTH, "clearing browsing data");
                        let _ = webview.clear_all_browsing_data();
                    }

                    // Wait time to avoid deadlock where window closes before
                    // the channel has received the redirect uri. InternalCloseWindow
                    // is called after ReachedRedirectUri.
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

    fn event_loop() -> EventLoop<UserEvents> {
        EventLoopBuilder::with_user_event()
            .with_any_thread(true)
            .build()
    }
}
