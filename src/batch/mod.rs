use crate::api_default_imports::*;
use crate::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

resource_api_client!(BatchApiClient);

impl BatchApiClient {
    pub fn batch<B: serde::Serialize>(&self, batch: &B) -> RequestHandler {
        let body_result = serde_json::to_string(batch).map_err(GraphFailure::from);
        let url_result = self.build_url("$batch", &serde_json::json!({}));

        let rc_result = RequestComponents::try_from((
            self.resource_config.resource_identity,
            Method::POST,
            url_result,
        ));

        match rc_result {
            Ok(rc) => {
                let mut header_map = HeaderMap::new();
                header_map
                    .entry(CONTENT_TYPE)
                    .or_insert(HeaderValue::from_static("application/json"));

                match body_result {
                    Ok(body) => RequestHandler::new(
                        self.client.clone(),
                        rc,
                        None,
                        Some(reqwest::Body::from(body)),
                    )
                    .headers(header_map),
                    Err(err) => RequestHandler::new(self.client.clone(), rc, Some(err), None)
                        .headers(header_map),
                }
            }
            Err(err) => {
                let rc = RequestComponents::new(
                    self.resource_config.resource_identity,
                    self.resource_config.url.clone(),
                    Method::POST,
                );
                RequestHandler::new(self.client.clone(), rc, Some(err), None)
            }
        }
    }
}
