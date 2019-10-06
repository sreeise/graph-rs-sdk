use crate::client::*;
use crate::http::{GraphResponse, GraphRequestType};
use crate::http::IntoResponse;
use crate::http::{FetchClient, UploadSessionClient};
use crate::types::collection::Collection;
use graph_error::{GraphFailure, GraphResult};
use graph_rs_types::complextypes::{ItemPreviewInfo, Thumbnail};
use graph_rs_types::entitytypes::{
    BaseItem, DriveItem, DriveItemVersion, ItemActivity, ThumbnailSet,
};
use handlebars::*;
use reqwest::header::{HeaderValue, CONTENT_LENGTH};
use reqwest::Method;
use serde::export::PhantomData;
use serde_json::json;
use std::fs::File;
use std::path::Path;

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

register_client!(
    DriveRequest,
    drive_item => "drive/items", "items", Ident::Drives,
    drive_root => "drive", "", Ident::Drives,
    drive_root_path => "drive/root", "root", Ident::Drives,
);

impl<'a, I> DriveRequest<'a, I> {
    get!( drive, BaseItem => "{{drive_root}}" );
    get!( root, DriveItem => "{{drive_root}}/root" );
    get!( recent, Collection<DriveItem> => "{{drive_root}}/recent" );
    get!( delta, Collection<DriveItem> => "{{drive_root}}/root/delta" );
    get!( root_children, Collection<DriveItem> => "{{drive_root}}/root/children" );
    get!( drive_activity, Collection<ItemActivity> => "{{drive_root}}/activities" );
    get!( thumbnails, Collection<ThumbnailSet> => "{{drive_item}}/thumbnails" );
    get!( shared_with_me, Collection<DriveItem> => "{{drive_root}}/sharedWithMe" );
    get!( special_documents, Collection<DriveItem> => "{{drive_root}}/special/documents" );
    get!( special_documents_children, Collection<DriveItem> => "{{drive_root}}/special/documents/children" );
    get!( special_photos, Collection<DriveItem> => "{{drive_root}}/special/photos" );
    get!( special_photos_children, Collection<DriveItem> => "{{drive_root}}/special/photos/children" );
    get!( special_camera_roll, Collection<DriveItem> => "{{drive_root}}/special/cameraroll" );
    get!( special_camera_roll_children, Collection<DriveItem> => "{{drive_root}}/special/cameraroll/children" );
    get!( special_app_root, Collection<DriveItem> => "{{drive_root}}/special/approot" );
    get!( special_app_root_children, Collection<DriveItem> => "{{drive_root}}/special/approot/children" );
    get!( special_music, Collection<DriveItem> => "{{drive_root}}/special/music" );
    get!( special_music_children, Collection<DriveItem> => "{{drive_root}}/special/music/children" );

    pub fn list_children<S: AsRef<str>>(
        &'a self,
        id: S,
    ) -> IntoResponse<'a, I, Collection<DriveItem>> {
        self.client.builder().set_method(Method::GET);
        render_path!(
            self.client,
            &template(id.as_ref(), "children"),
            &json!({ "id": encode(id.as_ref()) })
        );
        IntoResponse::new(self.client)
    }

    pub fn item_activity<S: AsRef<str>>(
        &'a self,
        id: S,
    ) -> IntoResponse<'a, I, Collection<DriveItem>> {
        self.client.builder().set_method(Method::GET);
        render_path!(
            self.client,
            &template(id.as_ref(), "activities"),
            &json!({ "id": encode(id.as_ref()) })
        );
        IntoResponse::new(self.client)
    }

    pub fn get_item<S: AsRef<str>>(&'a self, id: S) -> IntoResponse<'a, I, DriveItem> {
        self.client.builder().set_method(Method::GET);
        render_path!(
            self.client,
            template(id.as_ref(), "").as_str(),
            &json!({ "id": encode(id.as_ref()) })
        );
        IntoResponse::new(self.client)
    }

    pub fn update<S: AsRef<str>, B: serde::Serialize>(
        &'a self,
        id: S,
        body: &B,
    ) -> IntoResponse<'a, I, DriveItem> {
        self.client
            .builder()
            .set_method(Method::PATCH)
            .set_body(serde_json::to_string(body).unwrap());
        render_path!(
            self.client,
            template(id.as_ref(), "").as_str(),
            &json!({"id": encode(id.as_ref()) })
        );
        IntoResponse::new(self.client)
    }

    pub fn delete<S: AsRef<str>>(&'a self, id: S) -> IntoResponse<'a, I, GraphResponse<()>> {
        self.client.builder().set_method(Method::DELETE);
        render_path!(
            self.client,
            template(id.as_ref(), "").as_str(),
            &json!({"id": encode(id.as_ref()) })
        );
        IntoResponse::new(self.client)
    }

    pub fn create_folder<S: AsRef<str>, B: serde::Serialize>(
        &'a self,
        id: S,
        drive_item: &B,
    ) -> IntoResponse<'a, I, DriveItem> {
        self.client
            .builder()
            .set_method(Method::POST)
            .set_body(serde_json::to_string(drive_item).unwrap());

        if id.as_ref().is_empty() {
            render_path!(self.client, "{{drive_root_path}}/children", &json!({}));
        } else {
            render_path!(
                self.client,
                template(id.as_ref(), "children").as_str(),
                &json!({ "id": encode(id.as_ref()) })
            );
        }
        IntoResponse::new(self.client)
    }

    pub fn copy<S: AsRef<str>, B: serde::Serialize>(
        &'a self,
        id: S,
        body: &B,
    ) -> IntoResponse<'a, I, GraphResponse<()>> {
        self.client
            .builder()
            .set_method(Method::POST)
            .set_body(serde_json::to_string(body).unwrap());
        render_path!(
            self.client,
            template(id.as_ref(), "copy").as_str(),
            &json!({"id": encode(id.as_ref()) })
        );
        IntoResponse::new(self.client)
    }

    pub fn list_versions<S: AsRef<str>>(
        &self,
        id: S,
    ) -> IntoResponse<'a, I, Collection<DriveItemVersion>> {
        self.client.builder().set_method(Method::GET);
        render_path!(
            self.client,
            template(id.as_ref(), "versions").as_str(),
            &json!({ "id": encode(id.as_ref()) })
        );
        IntoResponse::new(self.client)
    }

    pub fn single_thumbnail<S: AsRef<str>>(
        &'a self,
        id: S,
        thumb_id: &str,
        size: &str,
    ) -> IntoResponse<'a, I, Thumbnail> {
        self.client.builder().set_method(Method::GET);
        render_path!(
            self.client,
            template(id.as_ref(), "thumbnails/{{thumb_id}}/{{size}}").as_str(),
            &json!({
               "id": encode(id.as_ref()),
               "thumb_id": thumb_id,
               "size": size
            })
        );
        IntoResponse::new(self.client)
    }

    pub fn thumbnail_binary<S: AsRef<str>>(
        &'a self,
        id: S,
        thumb_id: &str,
        size: &str,
    ) -> IntoResponse<'a, I, Vec<u8>> {
        self.client.builder().set_method(Method::GET);
        render_path!(
            self.client,
            template(id.as_ref(), "thumbnails/{{thumb_id}}/{{size}}/content").as_str(),
            &json!({
               "id": encode(id.as_ref()),
               "thumb_id": thumb_id,
               "size": size
            })
        );
        IntoResponse::new(self.client)
    }

    pub fn upload_replace<S: AsRef<str>, P: AsRef<Path>>(
        &'a self,
        id: S,
        file: P,
    ) -> GraphResult<IntoResponse<'a, I, DriveItem>> {
        self.client
            .builder()
            .set_method(Method::PUT)
            .set_body(File::open(file)?);
        render_path!(
            self.client,
            template(id.as_ref(), "content").as_str(),
            &json!({"id": encode(id.as_ref()) })
        );
        Ok(IntoResponse::new(self.client))
    }

    pub fn upload_new<S: AsRef<str>, P: AsRef<Path>>(
        &'a self,
        id: S,
        file: P,
    ) -> GraphResult<IntoResponse<'a, I, DriveItem>> {
        if id.as_ref().starts_with(':') {
            self.client
                .builder()
                .set_method(Method::PUT)
                .set_body(File::open(file)?);
            render_path!(
                self.client,
                template(id.as_ref(), "content").as_str(),
                &json!({"id": encode(id.as_ref()) })
            );
        } else {
            let name = file
                .as_ref()
                .file_name()
                .ok_or_else(|| GraphFailure::none_err("file_name"))?
                .to_string_lossy()
                .to_string();
            self.client
                .builder()
                .set_method(Method::PUT)
                .set_body(File::open(file)?);
            render_path!(
                self.client,
                "{{drive_item}}/{{id}}:/{{file_name}}:/content",
                &json!({
                    "id": id.as_ref(),
                    "file_name": name,
                })
            );
        }
        Ok(IntoResponse::new(self.client))
    }

    pub fn restore_version<S: AsRef<str>>(
        &'a self,
        id: S,
        version_id: S,
    ) -> IntoResponse<'a, I, GraphResponse<()>> {
        self.client.builder().set_method(Method::POST);
        render_path!(
            self.client,
            template(id.as_ref(), "versions/{{version_id}}/restoreVersion").as_str(),
            &json!({
                "id": encode(id.as_ref()),
                "version_id": version_id.as_ref(),
            })
        );
        IntoResponse::new(self.client)
    }

    pub fn upload_session<S: AsRef<str>, P: AsRef<Path>, B: serde::Serialize>(
        &'a self,
        id: S,
        file: P,
        body: B,
    ) -> IntoResponse<'a, I, UploadSessionClient> {
        self.client
            .builder()
            .set_method(Method::POST)
            .set_upload_session(file)
            .set_body(serde_json::to_string(&json!({ "item": body })).unwrap());
        render_path!(
            self.client,
            template(id.as_ref(), "createUploadSession").as_str(),
            &json!({ "id": encode(id.as_ref()) })
        );
        IntoResponse::new(self.client)
    }

    pub fn preview<S: AsRef<str>, B: serde::Serialize>(
        &'a self,
        id: S,
        body: Option<&B>,
    ) -> IntoResponse<'a, I, ItemPreviewInfo> {
        if let Some(body) = body {
            self.client
                .builder()
                .set_method(Method::POST)
                .set_body(serde_json::to_string(body).unwrap());
        } else {
            self.client
                .builder()
                .set_method(Method::POST)
                .header(CONTENT_LENGTH, HeaderValue::from(0));
        }
        render_path!(
            self.client,
            template(id.as_ref(), "preview").as_str(),
            &json!({ "id": encode(id.as_ref()) })
        );
        IntoResponse::new(self.client)
    }

    pub fn download<S: AsRef<str>, P: AsRef<Path>>(&'a self, id: S, directory: P) -> FetchClient {
        render_path!(
            self.client,
            template(id.as_ref(), "content").as_str(),
            &json!({ "id": encode(id.as_ref()) })
        );
        self.client
            .builder()
            .set_method(Method::GET)
            .set_download_dir(directory.as_ref())
            .set_request_type(GraphRequestType::Redirect);
        self.client.request().download(self.client.take_builder())
    }

    pub fn check_out<S: AsRef<str>>(&'a self, id: S) -> IntoResponse<'a, I, GraphResponse<()>> {
        render_path!(
            self.client,
            template(id.as_ref(), "checkout").as_str(),
            &json!({ "id": encode(id.as_ref()) })
        );
        self.client
            .builder()
            .set_method(Method::POST)
            .header(CONTENT_LENGTH, HeaderValue::from(0));
        IntoResponse::new(self.client)
    }

    pub fn check_in<S: AsRef<str>, B: serde::Serialize>(
        &'a self,
        id: S,
        body: &B,
    ) -> IntoResponse<'a, I, GraphResponse<()>> {
        render_path!(
            self.client,
            template(id.as_ref(), "checkin").as_str(),
            &json!({ "id": encode(id.as_ref()) })
        );
        self.client
            .builder()
            .set_method(Method::POST)
            .set_body(serde_json::to_string(body).unwrap());
        IntoResponse::new(self.client)
    }
}
