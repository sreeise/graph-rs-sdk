use crate::web::WebViewOptions;

pub trait InteractiveAuthenticator {
    fn interactive_authentication(
        &self,
        interactive_web_view_options: Option<WebViewOptions>,
    ) -> anyhow::Result<Option<String>>;
}

/*
let url = self.authorization_url()?;
        let redirect_url = self.redirect_uri()?;
        let web_view_options = interactive_web_view_options.unwrap_or_default();
        let timeout = web_view_options.timeout.clone();
        let (sender, receiver) = std::sync::mpsc::channel();

        let handle = std::thread::spawn(move || {
           match receiver.recv_timeout(timeout) {
               Ok(url) => return Ok(url),
               Err(e) => Err(e)
           }
        });

        InteractiveWebView::interactive_authentication(
            &url,
            &redirect_url,
            web_view_options,
            sender
        )?;
        handle.join().unwrap().map_err(anyhow::Error::from)
 */
