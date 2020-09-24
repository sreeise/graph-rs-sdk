use crate::client::*;
use graph_error::{GraphFailure, GraphRsError};
use graph_http::types::{Collection, Content, DeltaPhantom};
use graph_http::{
    AsyncDownload, AsyncHttpClient, BlockingDownload, BlockingHttpClient, GraphResponse,
    IntoResponse, RequestAttribute, RequestClient, RequestType, UploadSessionClient,
};
use handlebars::*;
use reqwest::header::{HeaderValue, CONTENT_LENGTH};
use reqwest::Method;
use serde_json::json;
use std::path::Path;

register_client!(DriveRequest,);
register_client!(DriveContentTypesRequest,);
register_client!(DriveListRequest,);
register_client!(DriveListItemsRequest,);
register_client!(DriveVersionsRequest,);
register_client!(
    DrivesRequest,
    drive_item => "drive/items", "items", Ident::Drives,
    drive_root => "drive", "", Ident::Drives,
    drive_root_path => "drive/root", "root", Ident::Drives,
);

fn template(s: &str, last: &str) -> String {
    if s.starts_with(':') {
        vec!["{{drive_root_path}}{{id}}/", last].join("")
    } else {
        vec!["{{drive_item}}/{{id}}/", last].join("")
    }
}

fn encode(s: &str) -> String {
    if s.starts_with(':') {
        url::percent_encoding::percent_encode(
            s.as_bytes(),
            url::percent_encoding::DEFAULT_ENCODE_SET,
        )
        .collect::<String>()
    } else {
        s.to_string()
    }
}

// Requests for the /drives/{{drive-id}} path

impl<'a, Client> DrivesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( drive, serde_json::Value => "{{drive_root}}" );
    get!( root, serde_json::Value => "{{drive_root}}/root" );
    get!( recent, Collection<serde_json::Value> => "{{drive_root}}/recent" );
    get!( delta, DeltaPhantom<Collection<serde_json::Value>> => "{{drive_root}}/root/delta" );
    get!( root_children, Collection<serde_json::Value> => "{{drive_root}}/root/children" );
    get!( drive_activity, Collection<serde_json::Value> => "{{drive_root}}/activities" );
    get!( thumbnails, Collection<serde_json::Value> => "{{drive_item}}/thumbnails" );
    get!( shared_with_me, Collection<serde_json::Value> => "{{drive_root}}/sharedWithMe" );
    get!( special_documents, serde_json::Value => "{{drive_root}}/special/documents" );
    get!( special_documents_children, Collection<serde_json::Value> => "{{drive_root}}/special/documents/children" );
    get!( special_photos, serde_json::Value => "{{drive_root}}/special/photos" );
    get!( special_photos_children, Collection<serde_json::Value> => "{{drive_root}}/special/photos/children" );
    get!( special_camera_roll, serde_json::Value => "{{drive_root}}/special/cameraroll" );
    get!( special_camera_roll_children, Collection<serde_json::Value> => "{{drive_root}}/special/cameraroll/children" );
    get!( special_app_root, serde_json::Value => "{{drive_root}}/special/approot" );
    get!( special_app_root_children, Collection<serde_json::Value> => "{{drive_root}}/special/approot/children" );
    get!( special_music, serde_json::Value => "{{drive_root}}/special/music" );
    get!( special_music_children, Collection<serde_json::Value> => "{{drive_root}}/special/music/children" );
    get!( | special_folder, serde_json::Value => "{{drive_root}}/special/{{id}}" );

    pub fn list_children<S: AsRef<str>>(
        &'a self,
        id: S,
    ) -> IntoResponse<'a, Collection<serde_json::Value>, Client> {
        self.client.request().set_method(Method::GET);
        render_path!(
            self.client,
            &template(id.as_ref(), "children"),
            &json!({ "id": encode(id.as_ref()) })
        );
        IntoResponse::new(&self.client.request)
    }

    pub fn item_activity<S: AsRef<str>>(
        &'a self,
        id: S,
    ) -> IntoResponse<'a, Collection<serde_json::Value>, Client> {
        self.client.request().set_method(Method::GET);
        render_path!(
            self.client,
            &template(id.as_ref(), "activities"),
            &json!({ "id": encode(id.as_ref()) })
        );
        IntoResponse::new(&self.client.request)
    }

    pub fn get_item<S: AsRef<str>>(&'a self, id: S) -> IntoResponse<'a, serde_json::Value, Client> {
        self.client.request().set_method(Method::GET);
        render_path!(
            self.client,
            template(id.as_ref(), "").as_str(),
            &json!({ "id": encode(id.as_ref()) })
        );
        IntoResponse::new(&self.client.request)
    }

    pub fn update<S: AsRef<str>, B: serde::Serialize>(
        &'a self,
        id: S,
        body: &B,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
        let body = serde_json::to_string(body);
        if let Ok(body) = body {
            let client = self.client.request();
            client.set_method(Method::PATCH);
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
        }
        render_path!(
            self.client,
            template(id.as_ref(), "").as_str(),
            &json!({"id": encode(id.as_ref()) })
        );
        IntoResponse::new(&self.client.request)
    }

    pub fn delete<S: AsRef<str>>(
        &'a self,
        id: S,
    ) -> IntoResponse<'a, GraphResponse<Content>, Client> {
        self.client.request().set_method(Method::DELETE);
        render_path!(
            self.client,
            template(id.as_ref(), "").as_str(),
            &json!({"id": encode(id.as_ref()) })
        );
        IntoResponse::new(&self.client.request)
    }

    pub fn create_folder<S: AsRef<str>, B: serde::Serialize>(
        &'a self,
        id: S,
        body: &B,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
        let body = serde_json::to_string(body);
        if let Ok(body) = body {
            let client = self.client.request();
            client.set_method(Method::POST);
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
        }

        if id.as_ref().is_empty() {
            render_path!(self.client, "{{drive_root_path}}/children", &json!({}));
        } else {
            render_path!(
                self.client,
                template(id.as_ref(), "children").as_str(),
                &json!({ "id": encode(id.as_ref()) })
            );
        }
        IntoResponse::new(&self.client.request)
    }

    pub fn copy<S: AsRef<str>, B: serde::Serialize>(
        &'a self,
        id: S,
        body: &B,
    ) -> IntoResponse<'a, GraphResponse<Content>, Client> {
        let body = serde_json::to_string(body);
        if let Ok(body) = body {
            let client = self.client.request();
            client.set_method(Method::POST);
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
        }
        render_path!(
            self.client,
            template(id.as_ref(), "copy").as_str(),
            &json!({"id": encode(id.as_ref()) })
        );
        IntoResponse::new(&self.client.request)
    }

    pub fn list_versions<S: AsRef<str>>(
        &self,
        id: S,
    ) -> IntoResponse<'a, Collection<serde_json::Value>, Client> {
        self.client.request().set_method(Method::GET);
        render_path!(
            self.client,
            template(id.as_ref(), "versions").as_str(),
            &json!({ "id": encode(id.as_ref()) })
        );
        IntoResponse::new(&self.client.request)
    }

    pub fn single_thumbnail<S: AsRef<str>>(
        &'a self,
        id: S,
        thumb_id: &str,
        size: &str,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
        self.client.request().set_method(Method::GET);
        render_path!(
            self.client,
            template(id.as_ref(), "thumbnails/{{thumb_id}}/{{size}}").as_str(),
            &json!({
               "id": encode(id.as_ref()),
               "thumb_id": thumb_id,
               "size": size
            })
        );
        IntoResponse::new(&self.client.request)
    }

    pub fn thumbnail_binary<S: AsRef<str>>(
        &'a self,
        id: S,
        thumb_id: &str,
        size: &str,
    ) -> IntoResponse<'a, Vec<u8>, Client> {
        self.client.request().set_method(Method::GET);
        render_path!(
            self.client,
            template(id.as_ref(), "thumbnails/{{thumb_id}}/{{size}}/content").as_str(),
            &json!({
               "id": encode(id.as_ref()),
               "thumb_id": thumb_id,
               "size": size
            })
        );
        IntoResponse::new(&self.client.request)
    }

    pub fn upload_replace<S: AsRef<str>, P: AsRef<Path>>(
        &'a self,
        id: S,
        file: P,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
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
            &json!({"id": encode(id.as_ref()) })
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
                &json!({"id": encode(id.as_ref()) })
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

    pub fn restore_version<S: AsRef<str>>(
        &'a self,
        id: S,
        version_id: S,
    ) -> IntoResponse<'a, GraphResponse<Content>, Client> {
        self.client.request().set_method(Method::POST);
        render_path!(
            self.client,
            template(id.as_ref(), "versions/{{version_id}}/restoreVersion").as_str(),
            &json!({
                "id": encode(id.as_ref()),
                "version_id": version_id.as_ref(),
            })
        );
        IntoResponse::new(&self.client.request)
    }

    pub fn upload_session<S: AsRef<str>, P: AsRef<Path> + Send + Sync, B: serde::Serialize>(
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
            &json!({ "id": encode(id.as_ref()) })
        );
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
            &json!({ "id": encode(id.as_ref()) })
        );
        IntoResponse::new(&self.client.request)
    }

    pub fn content<S: AsRef<str>>(
        &'a self,
        id: S,
    ) -> IntoResponse<'a, GraphResponse<Content>, Client> {
        render_path!(
            self.client,
            template(id.as_ref(), "content").as_str(),
            &json!({ "id": encode(id.as_ref()) })
        );
        self.client.request().set_method(Method::GET);
        IntoResponse::new(&self.client.request)
    }

    pub fn check_out<S: AsRef<str>>(
        &'a self,
        id: S,
    ) -> IntoResponse<'a, GraphResponse<Content>, Client> {
        render_path!(
            self.client,
            template(id.as_ref(), "checkout").as_str(),
            &json!({ "id": encode(id.as_ref()) })
        );
        let client = self.client.request();
        client.set_method(Method::POST);
        client.header(CONTENT_LENGTH, HeaderValue::from(0));
        IntoResponse::new(&self.client.request)
    }

    pub fn check_in<S: AsRef<str>, B: serde::Serialize>(
        &'a self,
        id: S,
        body: &B,
    ) -> IntoResponse<'a, GraphResponse<Content>, Client> {
        render_path!(
            self.client,
            template(id.as_ref(), "checkin").as_str(),
            &json!({ "id": encode(id.as_ref()) })
        );

        let body = serde_json::to_string(body);
        if let Ok(body) = body {
            let client = self.client.request();
            client.set_method(Method::POST);
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
        }
        IntoResponse::new(&self.client.request)
    }

    pub fn move_item<S: AsRef<str>, B: serde::Serialize>(
        &'a self,
        id: S,
        body: &B,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
        let body = serde_json::to_string(body);
        if let Ok(body) = body {
            let client = self.client.request();
            client.set_method(Method::PATCH);
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
        }
        render_path!(
            self.client,
            template(id.as_ref(), "").as_str(),
            &json!({ "id": id.as_ref() })
        );
        IntoResponse::new(&self.client.request)
    }

    pub fn activities_by_interval<S: AsRef<str>>(
        &'a self,
        id: S,
        start: &str,
        end: Option<&str>,
        interval: &str,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
        self.client.request().set_method(Method::GET);
        if let Some(end) = end {
            let interval = format!(
                "getActivitiesByInterval(startDateTime='{}',endDateTime='{}',interval='{}')",
                start, end, interval
            );
            render_path!(
                self.client,
                &template(id.as_ref(), &interval),
                &serde_json::json!({
                   "id": id.as_ref(),
                })
            );
        } else {
            let interval = format!(
                "getActivitiesByInterval(startDateTime='{}',interval='{}')",
                start, interval
            );
            render_path!(
                self.client,
                &template(id.as_ref(), &interval),
                &serde_json::json!({
                    "id": id.as_ref(),
                })
            );
        }
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
            &json!({ "id": encode(id.as_ref()) })
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
            &json!({ "id": encode(id.as_ref()) })
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

// Requests for the /drive path.

impl<'a, Client> DriveRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn list(&self) -> DriveListRequest<'a, Client> {
        DriveListRequest::new(&self.client)
    }

    get!({
        doc: "# Get following from drive",
        name: get_following,
        response: serde_json::Value,
        path: "/drive/following/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property following in drive",
        name: update_following,
        response: GraphResponse<Content>,
        path: "/drive/following/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property following for drive",
        name: delete_following,
        response: GraphResponse<Content>,
        path: "/drive/following/{{id}}",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action restoreVersion",
        name: restore_version,
        response: GraphResponse<Content>,
        path: "/drive/list/items/{{id}}/versions/{{id2}}/microsoft.graph.restoreVersion",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get items from drive",
        name: get_items,
        response: serde_json::Value,
        path: "/drive/items/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property items in drive",
        name: update_items,
        response: GraphResponse<Content>,
        path: "/drive/items/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property items for drive",
        name: delete_items,
        response: GraphResponse<Content>,
        path: "/drive/items/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get list from drive",
        name: get_list,
        response: serde_json::Value,
        path: "/drive/list",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property list in drive",
        name: update_list,
        response: GraphResponse<Content>,
        path: "/drive/list",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property list for drive",
        name: delete_list,
        response: GraphResponse<Content>,
        path: "/drive/list",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get entities from drives",
        name: list_drive,
        response: Collection<serde_json::Value>,
        path: "/drives",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Add new entity to drives",
        name: create_drive,
        response: serde_json::Value,
        path: "/drives",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get items from drive",
        name: list_items,
        response: Collection<serde_json::Value>,
        path: "/drive/items",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to items for drive",
        name: create_items,
        response: serde_json::Value,
        path: "/drive/items",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get special from drive",
        name: list_special,
        response: Collection<serde_json::Value>,
        path: "/drive/special",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to special for drive",
        name: create_special,
        response: serde_json::Value,
        path: "/drive/special",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function getActivitiesByInterval",
        name: items,
        response: Collection<serde_json::Value>,
        path: "/drive/list/items/{{id}}/microsoft.graph.getActivitiesByInterval()",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get special from drive",
        name: get_special,
        response: serde_json::Value,
        path: "/drive/special/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property special in drive",
        name: update_special,
        response: GraphResponse<Content>,
        path: "/drive/special/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property special for drive",
        name: delete_special,
        response: GraphResponse<Content>,
        path: "/drive/special/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get drive",
        name: get_drive,
        response: serde_json::Value,
        path: "/drive",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update drive",
        name: update_drive,
        response: GraphResponse<Content>,
        path: "/drive",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get following from drive",
        name: list_following,
        response: Collection<serde_json::Value>,
        path: "/drive/following",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to following for drive",
        name: create_following,
        response: serde_json::Value,
        path: "/drive/following",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function recent",
        name: recent,
        response: Collection<serde_json::Value>,
        path: "/drive/microsoft.graph.recent()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Invoke function sharedWithMe",
        name: shared_with_me,
        response: Collection<serde_json::Value>,
        path: "/drive/microsoft.graph.sharedWithMe()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get root from drive",
        name: get_root,
        response: serde_json::Value,
        path: "/drive/root",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property root in drive",
        name: update_root,
        response: GraphResponse<Content>,
        path: "/drive/root",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property root for drive",
        name: delete_root,
        response: GraphResponse<Content>,
        path: "/drive/root",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> DriveVersionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get fields from drive",
        name: get_fields,
        response: serde_json::Value,
        path: "/drive/list/items/{{id}}/versions/{{id2}}/fields",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property fields in drive",
        name: update_fields,
        response: GraphResponse<Content>,
        path: "/drive/list/items/{{id}}/versions/{{id2}}/fields",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property fields for drive",
        name: delete_fields,
        response: GraphResponse<Content>,
        path: "/drive/list/items/{{id}}/versions/{{id2}}/fields",
        params: 2,
        has_body: false
    });
}

impl<'a, Client> DriveContentTypesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get columnLinks from drive",
        name: list_column_links,
        response: Collection<serde_json::Value>,
        path: "/drive/list/contentTypes/{{id}}/columnLinks",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to columnLinks for drive",
        name: create_column_links,
        response: serde_json::Value,
        path: "/drive/list/contentTypes/{{id}}/columnLinks",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get columnLinks from drive",
        name: get_column_links,
        response: serde_json::Value,
        path: "/drive/list/contentTypes/{{id}}/columnLinks/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property columnLinks in drive",
        name: update_column_links,
        response: GraphResponse<Content>,
        path: "/drive/list/contentTypes/{{id}}/columnLinks/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property columnLinks for drive",
        name: delete_column_links,
        response: GraphResponse<Content>,
        path: "/drive/list/contentTypes/{{id}}/columnLinks/{{id2}}",
        params: 2,
        has_body: false
    });
}

impl<'a, Client> DriveListRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn content_types(&self) -> DriveContentTypesRequest<'a, Client> {
        DriveContentTypesRequest::new(&self.client)
    }

    pub fn items(&self) -> DriveListItemsRequest<'a, Client> {
        DriveListItemsRequest::new(&self.client)
    }

    get!({
        doc: "# Get contentTypes from drive",
        name: list_content_types,
        response: Collection<serde_json::Value>,
        path: "/drive/list/contentTypes",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to contentTypes for drive",
        name: create_content_types,
        response: serde_json::Value,
        path: "/drive/list/contentTypes",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get drive from drive",
        name: get_drive,
        response: serde_json::Value,
        path: "/drive/list/drive",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property drive in drive",
        name: update_drive,
        response: GraphResponse<Content>,
        path: "/drive/list/drive",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property drive for drive",
        name: delete_drive,
        response: GraphResponse<Content>,
        path: "/drive/list/drive",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get items from drive",
        name: get_items,
        response: serde_json::Value,
        path: "/drive/list/items/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property items in drive",
        name: update_items,
        response: GraphResponse<Content>,
        path: "/drive/list/items/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property items for drive",
        name: delete_items,
        response: GraphResponse<Content>,
        path: "/drive/list/items/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get columns from drive",
        name: get_columns,
        response: serde_json::Value,
        path: "/drive/list/columns/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property columns in drive",
        name: update_columns,
        response: GraphResponse<Content>,
        path: "/drive/list/columns/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property columns for drive",
        name: delete_columns,
        response: GraphResponse<Content>,
        path: "/drive/list/columns/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get items from drive",
        name: list_items,
        response: Collection<serde_json::Value>,
        path: "/drive/list/items",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to items for drive",
        name: create_items,
        response: serde_json::Value,
        path: "/drive/list/items",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get contentTypes from drive",
        name: get_content_types,
        response: serde_json::Value,
        path: "/drive/list/contentTypes/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property contentTypes in drive",
        name: update_content_types,
        response: GraphResponse<Content>,
        path: "/drive/list/contentTypes/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property contentTypes for drive",
        name: delete_content_types,
        response: GraphResponse<Content>,
        path: "/drive/list/contentTypes/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get columns from drive",
        name: list_columns,
        response: Collection<serde_json::Value>,
        path: "/drive/list/columns",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to columns for drive",
        name: create_columns,
        response: serde_json::Value,
        path: "/drive/list/columns",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get subscriptions from drive",
        name: list_subscriptions,
        response: Collection<serde_json::Value>,
        path: "/drive/list/subscriptions",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to subscriptions for drive",
        name: create_subscriptions,
        response: serde_json::Value,
        path: "/drive/list/subscriptions",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get subscriptions from drive",
        name: get_subscriptions,
        response: serde_json::Value,
        path: "/drive/list/subscriptions/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property subscriptions in drive",
        name: update_subscriptions,
        response: GraphResponse<Content>,
        path: "/drive/list/subscriptions/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property subscriptions for drive",
        name: delete_subscriptions,
        response: GraphResponse<Content>,
        path: "/drive/list/subscriptions/{{id}}",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> DriveListItemsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn versions(&self) -> DriveVersionsRequest<'a, Client> {
        DriveVersionsRequest::new(&self.client)
    }

    get!({
        doc: "# Get analytics from drive",
        name: get_analytics,
        response: serde_json::Value,
        path: "/drive/list/items/{{id}}/analytics",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get driveItem from drive",
        name: get_drive_item,
        response: serde_json::Value,
        path: "/drive/list/items/{{id}}/driveItem",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property driveItem in drive",
        name: update_drive_item,
        response: GraphResponse<Content>,
        path: "/drive/list/items/{{id}}/driveItem",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property driveItem for drive",
        name: delete_drive_item,
        response: GraphResponse<Content>,
        path: "/drive/list/items/{{id}}/driveItem",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get versions from drive",
        name: get_versions,
        response: serde_json::Value,
        path: "/drive/list/items/{{id}}/versions/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property versions in drive",
        name: update_versions,
        response: GraphResponse<Content>,
        path: "/drive/list/items/{{id}}/versions/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property versions for drive",
        name: delete_versions,
        response: GraphResponse<Content>,
        path: "/drive/list/items/{{id}}/versions/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get fields from drive",
        name: get_fields,
        response: serde_json::Value,
        path: "/drive/list/items/{{id}}/fields",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property fields in drive",
        name: update_fields,
        response: GraphResponse<Content>,
        path: "/drive/list/items/{{id}}/fields",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property fields for drive",
        name: delete_fields,
        response: GraphResponse<Content>,
        path: "/drive/list/items/{{id}}/fields",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get versions from drive",
        name: list_versions,
        response: Collection<serde_json::Value>,
        path: "/drive/list/items/{{id}}/versions",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to versions for drive",
        name: create_versions,
        response: serde_json::Value,
        path: "/drive/list/items/{{id}}/versions",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get ref of analytics from drive",
        name: get_ref_analytics,
        response: serde_json::Value,
        path: "/drive/list/items/{{id}}/analytics/$ref",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the ref of navigation property analytics in drive",
        name: update_ref_analytics,
        response: GraphResponse<Content>,
        path: "/drive/list/items/{{id}}/analytics/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete ref of navigation property analytics for drive",
        name: delete_ref_analytics,
        response: GraphResponse<Content>,
        path: "/drive/list/items/{{id}}/analytics/$ref",
        params: 1,
        has_body: false
    });
}
