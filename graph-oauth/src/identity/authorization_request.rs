use http::header::CONTENT_TYPE;
use http::{HeaderMap, HeaderValue};
use std::collections::HashMap;
use url::Url;

pub struct AuthorizationRequest {
    pub(crate) uri: Url,
    pub(crate) form_urlencoded: HashMap<String, String>,
    pub(crate) basic_auth: Option<(String, String)>,
    pub(crate) headers: HeaderMap,
}

impl AuthorizationRequest {
    pub fn new(
        uri: Url,
        form_urlencoded: HashMap<String, String>,
        basic_auth: Option<(String, String)>,
    ) -> AuthorizationRequest {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        AuthorizationRequest {
            uri,
            form_urlencoded,
            basic_auth,
            headers,
        }
    }

    pub fn with_extra_headers(&mut self, extra_headers: &HeaderMap) {
        for (header_name, header_value) in extra_headers.iter() {
            self.headers.insert(header_name, header_value.clone());
        }
    }

    pub fn with_extra_query_parameters(&mut self, extra_query_params: &HashMap<String, String>) {
        for (key, value) in extra_query_params.iter() {
            self.uri
                .query_pairs_mut()
                .append_pair(key.as_ref(), value.as_ref());
        }
    }
}
