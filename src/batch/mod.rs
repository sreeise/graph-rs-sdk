use crate::api_default_imports::*;
use graph_http::RequestComponents;

resource_api_client!(BatchApiClient);

impl BatchApiClient {
    pub fn batch<B: serde::Serialize>(&self, batch: &B) -> ResponseHandler {
        let body_result = serde_json::to_string(batch).map_err(GraphFailure::from);

        let url_result = self.build_url("$batch", &serde_json::json!({}));

        if body_result.is_err() {
            return ResponseHandler::new(
                self.client.clone(),
                RequestComponents::new(
                    self.resource_config.resource_identity,
                    self.resource_config.url.clone(),
                    Method::POST,
                    None,
                ),
                body_result.err(),
            );
        }

        if url_result.is_err() {
            return ResponseHandler::new(
                self.client.clone(),
                RequestComponents::new(
                    self.resource_config.resource_identity,
                    self.resource_config.url.clone(),
                    Method::POST,
                    None,
                ),
                url_result.err(),
            );
        }

        let rq = RequestComponents::from((
            self.resource_config.resource_identity,
            Method::POST,
            url_result,
            body_result,
        ));
        return ResponseHandler::new(self.client.clone(), rq, None);
    }
}
