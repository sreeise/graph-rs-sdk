use std::collections::HashSet;
use url::Url;

use crate::web::HostOptions;
use graph_error::{WebViewError, WebViewResult};

pub(crate) struct WebViewHostValidator {
    start_uri: Url,
    redirect_uris: Vec<Url>,
    ports: HashSet<usize>,
    is_local_host: bool,
}

impl WebViewHostValidator {
    pub fn new(
        start_uri: Url,
        redirect_uris: Vec<Url>,
        ports: HashSet<usize>,
    ) -> WebViewResult<WebViewHostValidator> {
        if start_uri.host().is_none() || redirect_uris.iter().any(|uri| uri.host().is_none()) {
            return Err(WebViewError::InvalidUri(
                "Authorization url and redirect uri must have valid uri hosts".into(),
            ));
        }

        let is_local_host = redirect_uris
            .iter()
            .any(|uri| uri.as_str().eq("http://localhost"));

        if is_local_host && ports.is_empty() {
            return Err(WebViewError::InvalidUri(
                "Redirect uri is http://localhost but not ports were specified".into(),
            ));
        }

        Ok(WebViewHostValidator {
            start_uri,
            redirect_uris,
            ports,
            is_local_host,
        })
    }

    pub fn is_valid_uri(&self, url: &Url) -> bool {
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

    pub fn is_redirect_host(&self, url: &Url) -> bool {
        if let Some(host) = url.host() {
            self.redirect_uris
                .iter()
                .any(|uri| uri.host().eq(&Some(host.clone())))
        } else {
            false
        }
    }
}

impl TryFrom<HostOptions> for WebViewHostValidator {
    type Error = WebViewError;

    fn try_from(value: HostOptions) -> Result<Self, Self::Error> {
        WebViewHostValidator::new(value.start_uri, value.redirect_uris, value.ports)
    }
}
