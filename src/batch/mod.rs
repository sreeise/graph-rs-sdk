use crate::api_default_imports::*;
use crate::header::{HeaderName, CONTENT_TYPE};
use graph_http::RequestComponents;
use reqwest::header::{HeaderMap, HeaderValue};

resource_api_client!(BatchApiClient);

impl BatchApiClient {
    pub fn batch<B: serde::Serialize>(&self, batch: &B) -> ResponseHandler {
        let body_result = serde_json::to_string(batch).map_err(GraphFailure::from);
        let url_result = self.build_url("$batch", &serde_json::json!({}));

        let rc_result = RequestComponents::try_from((
            self.resource_config.resource_identity,
            Method::POST,
            url_result,
            body_result,
        ));

        if let Ok(rc) = rc_result {
            let mut header_map = HeaderMap::new();
            header_map
                .entry(CONTENT_TYPE)
                .or_insert(HeaderValue::from_static("application/json"));
            return ResponseHandler::new(self.client.clone(), rc, None).headers(header_map);
        }

        let rc = RequestComponents::new(
            self.resource_config.resource_identity,
            self.resource_config.url.clone(),
            Method::POST,
            None,
        );
        ResponseHandler::new(self.client.clone(), rc, rc_result.err())
    }
}
