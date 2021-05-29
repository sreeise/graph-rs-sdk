use crate::error::{AsRes, GraphRsError};
use crate::pages::PageRequest;
use graph_http::IntoResponse;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use reqwest::Method;
use std::ffi::OsStr;
use std::path::Path;

impl<'a, Client> PageRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn create_pages_from_file<P: AsRef<Path>>(
        &self,
        file: P,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
        render_path!(self.client, "/pages");

        if !file.as_ref().extension().eq(&Some(OsStr::new("html"))) {
            return IntoResponse::new_error(
                &self.client.request,
                GraphRsError::InvalidFileExtension {
                    requires: "html".to_string(),
                    found: file
                        .as_ref()
                        .extension()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                }
                .graph_failure(),
            );
        }

        if let Err(e) = self
            .client
            .request()
            .set_body_with_file(file.as_ref().to_path_buf())
        {
            return IntoResponse::new_error(&self.client.request, e);
        }
        let client = self.client.request();
        client.header(CONTENT_TYPE, HeaderValue::from_static("text/html"));
        client.set_method(Method::POST);
        IntoResponse::new(&self.client.request)
    }
}
