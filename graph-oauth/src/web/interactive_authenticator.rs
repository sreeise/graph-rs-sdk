use crate::identity::AuthorizationUrl;
use crate::web::{InteractiveWebView, InteractiveWebViewOptions};

pub trait InteractiveAuthenticator: AuthorizationUrl {
    fn interactive_authentication(
        &self,
        interactive_web_view_options: Option<InteractiveWebViewOptions>,
    ) -> anyhow::Result<()> {
        let url = self.authorization_url()?;
        let redirect_url = self.redirect_uri()?;
        InteractiveWebView::interactive_authentication(
            &url,
            &redirect_url,
            interactive_web_view_options.unwrap_or_default(),
        )?;
        Ok(())
    }
}
