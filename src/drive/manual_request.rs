use crate::core::ResourceIdentity;
use crate::drive::DrivesRequest;
use crate::error::{GraphFailure, GraphRsError};
use graph_http::types::Content;
use graph_http::IntoResponse;
use graph_http::{
    AsyncDownload, AsyncHttpClient, BlockingDownload, BlockingHttpClient, GraphResponse,
    RequestAttribute, RequestClient, RequestType, UploadSessionClient,
};
use reqwest::header::{HeaderValue, CONTENT_LENGTH};
use reqwest::Method;
use serde_json::json;
use std::path::Path;

fn template(s: &str, last: &str) -> String {
    if s.starts_with(':') {
        format!("{{{{drive_root_path}}}}{{{{id}}}}/{}", last)
    } else {
        format!("{{{{drive_item}}}}/{{{{id}}}}/{}", last)
    }
}

// Methods that require explicit implementation.
impl<'a, Client> DrivesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub(crate) fn transfer_identity(&self) {
        let ident = self.client.ident();
        if ident.eq(&ResourceIdentity::Users) ||
            ident.eq(&ResourceIdentity::Me) ||
            ident.eq(&ResourceIdentity::Sites) ||
            ident.eq(&ResourceIdentity::Groups)
        {
            self.client
                .request
                .extend_path(&[ResourceIdentity::Drive.as_ref()]);
        }
    }

    pub fn check_out_item<S: AsRef<str>>(
        &'a self,
        id: S,
    ) -> IntoResponse<'a, GraphResponse<Content>, Client> {
        render_path!(
            self.client,
            template(id.as_ref(), "checkout").as_str(),
            &json!({ "id": id.as_ref() })
        );
        let client = self.client.request();
        client.set_method(Method::POST);
        client.header(CONTENT_LENGTH, HeaderValue::from(0));
        IntoResponse::new(&self.client.request)
    }

    pub fn preview<S: AsRef<str>, B: serde::Serialize>(
        &'a self,
        id: S,
        body: Option<&B>,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
        if let Some(body) = body {
            let body = serde_json::to_string(body);
            if let Ok(body) = body {
                let client = self.client.request();
                client.set_method(Method::POST);
                client.set_body(body);
            } else if let Err(e) = body {
                return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
            }
        } else {
            let client = self.client.request();
            client.set_method(Method::POST);
            client.header(CONTENT_LENGTH, HeaderValue::from(0));
        }
        render_path!(
            self.client,
            template(id.as_ref(), "preview").as_str(),
            &json!({ "id": id.as_ref() })
        );
        IntoResponse::new(&self.client.request)
    }

    pub fn upload_new<S: AsRef<str>, P: AsRef<Path>>(
        &'a self,
        id: S,
        file: P,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
        if id.as_ref().starts_with(':') {
            if let Err(err) = self
                .client
                .request()
                .set_body_with_file(file.as_ref().to_path_buf())
            {
                return IntoResponse::new_error(self.client.request(), err);
            }

            self.client.request().set_method(Method::PUT);
            render_path!(
                self.client,
                template(id.as_ref(), "content").as_str(),
                &json!({"id": id.as_ref() })
            );
        } else {
            let name = file.as_ref().file_name();
            if name.is_none() {
                return IntoResponse::new_error(
                    self.client.request(),
                    GraphFailure::invalid("file_name"),
                );
            }
            let name = name.unwrap().to_str();
            if name.is_none() {
                return IntoResponse::new_error(
                    self.client.request(),
                    GraphFailure::internal(GraphRsError::FileNameInvalidUTF8),
                );
            }
            render_path!(
                self.client,
                "{{drive_item}}/{{id}}:/{{file_name}}:/content",
                &json!({
                    "id": id.as_ref(),
                    "file_name": name.unwrap(),
                })
            );

            if let Err(e) = self
                .client
                .request()
                .set_body_with_file(file.as_ref().to_path_buf())
            {
                return IntoResponse::new_error(self.client.request(), e);
            }
            self.client.request().set_method(Method::PUT);
        }
        IntoResponse::new(&self.client.request)
    }

    pub fn create_upload_session<
        S: AsRef<str>,
        P: AsRef<Path> + Send + Sync,
        B: serde::Serialize,
    >(
        &'a self,
        id: S,
        file: P,
        body: &B,
    ) -> IntoResponse<'a, UploadSessionClient<Client>, Client> {
        let body = serde_json::to_string(body);
        if let Ok(body) = body {
            let client = self.client.request();
            client.set_method(Method::POST);
            client.set_upload_session(file.as_ref().to_path_buf());
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
        }
        render_path!(
            self.client,
            template(id.as_ref(), "createUploadSession").as_str(),
            &json!({ "id": id.as_ref() })
        );
        IntoResponse::new(&self.client.request)
    }
}

impl<'a> DrivesRequest<'a, BlockingHttpClient> {
    pub fn download<S: AsRef<str>, P: AsRef<Path>>(
        &'a self,
        id: S,
        directory: P,
    ) -> BlockingDownload {
        render_path!(
            self.client,
            template(id.as_ref(), "content").as_str(),
            &json!({ "id": id.as_ref() })
        );
        self.client
            .request()
            .set_request(vec![
                RequestAttribute::Method(Method::GET),
                RequestAttribute::Download(directory.as_ref().to_path_buf()),
                RequestAttribute::RequestType(RequestType::Redirect),
            ])
            .unwrap();
        self.client.request().download()
    }
}

impl<'a> DrivesRequest<'a, AsyncHttpClient> {
    pub fn download<S: AsRef<str>, P: AsRef<Path>>(&'a self, id: S, directory: P) -> AsyncDownload {
        render_path!(
            self.client,
            template(id.as_ref(), "content").as_str(),
            &json!({ "id": id.as_ref() })
        );
        self.client
            .request()
            .set_request(vec![
                RequestAttribute::Method(Method::GET),
                RequestAttribute::Download(directory.as_ref().to_path_buf()),
                RequestAttribute::RequestType(RequestType::Redirect),
            ])
            .unwrap();
        futures::executor::block_on(self.client.request().download())
    }
}
