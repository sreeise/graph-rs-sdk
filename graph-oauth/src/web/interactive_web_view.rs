use url::{Host, Url};
use wry::application::window::Theme;
use wry::webview::WebviewExtWindows;
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

pub struct InteractiveWebView;

impl InteractiveWebView {
    fn is_validate_host(uri_to_validate: &Url, validate_against: &Vec<Url>) -> bool {
        let hosts: Vec<Host<&str>> = validate_against.iter().flat_map(|uri| uri.host()).collect();

        if let Some(attempted_host) = uri_to_validate.host() {
            hosts.contains(&attempted_host)
        } else {
            false
        }
    }

    pub fn interactive_authentication(uri: &Url, redirect_uri: &Url) -> anyhow::Result<()> {
        let event_loop: EventLoop<()> = EventLoop::new();
        let valid_uri_vec = vec![uri.clone(), redirect_uri.clone()];

        let window = WindowBuilder::new()
            .with_title("Sign In")
            .with_closable(true)
            .with_content_protection(true)
            .with_minimizable(true)
            .with_maximizable(true)
            .with_resizable(true)
            .with_theme(Some(Theme::Dark))
            .build(&event_loop)?;

        let webview = WebViewBuilder::new(window)?
            .with_url(uri.as_ref())?
            .with_file_drop_handler(|_, _| {
                return true;
            })
            .with_navigation_handler(move |uri| {
                if let Ok(url) = Url::parse(uri.as_str()) {
                    InteractiveWebView::is_validate_host(&url, &valid_uri_vec)
                } else {
                    false
                }
            })
            .build()?;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => (),
            }
        });
    }
}
