use crate::client::*;
use crate::http::{
    DownloadClient, GraphRequestType, GraphResponse, IntoResponse, UploadSessionClient,
};
use crate::types::collection::Collection;
use crate::types::content::Content;
use crate::types::delta::DeltaRequest;
use graph_error::{GraphFailure, GraphRsError};
use graph_rs_types::complextypes::{ItemPreviewInfo, Thumbnail};
use graph_rs_types::entitytypes::{
    BaseItem, DriveItem, DriveItemVersion, ItemActivity, ItemActivityStat, ThumbnailSet,
};
use handlebars::*;
use reqwest::header::{HeaderValue, CONTENT_LENGTH};
use reqwest::Method;
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

impl<'a> DriveRequest<'a> {
    get!( drive, BaseItem => "{{drive_root}}" );
    get!( root, DriveItem => "{{drive_root}}/root" );
    get!( recent, Collection<DriveItem> => "{{drive_root}}/recent" );
    get!( delta, DeltaRequest<Collection<DriveItem>> => "{{drive_root}}/root/delta" );
    get!( root_children, Collection<DriveItem> => "{{drive_root}}/root/children" );
    get!( drive_activity, Collection<ItemActivity> => "{{drive_root}}/activities" );
    get!( thumbnails, Collection<ThumbnailSet> => "{{drive_item}}/thumbnails" );
    get!( shared_with_me, Collection<DriveItem> => "{{drive_root}}/sharedWithMe" );
    get!( | special_folder, DriveItem => "{{drive_root}}/special/{{id}}" );
    get!( special_documents, DriveItem => "{{drive_root}}/special/documents" );
    get!( special_documents_children, Collection<DriveItem> => "{{drive_root}}/special/documents/children" );
    get!( special_photos, DriveItem => "{{drive_root}}/special/photos" );
    get!( special_photos_children, Collection<DriveItem> => "{{drive_root}}/special/photos/children" );
    get!( special_camera_roll, DriveItem => "{{drive_root}}/special/cameraroll" );
    get!( special_camera_roll_children, Collection<DriveItem> => "{{drive_root}}/special/cameraroll/children" );
    get!( special_app_root, DriveItem => "{{drive_root}}/special/approot" );
    get!( special_app_root_children, Collection<DriveItem> => "{{drive_root}}/special/approot/children" );
    get!( special_music, DriveItem => "{{drive_root}}/special/music" );
    get!( special_music_children, Collection<DriveItem> => "{{drive_root}}/special/music/children" );

    pub fn list_children<S: AsRef<str>>(
        &'a self,
        id: S,
    ) -> IntoResponse<'a, Collection<DriveItem>> {
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
    ) -> IntoResponse<'a, Collection<DriveItem>> {
        self.client.builder().set_method(Method::GET);
        render_path!(
            self.client,
            &template(id.as_ref(), "activities"),
            &json!({ "id": encode(id.as_ref()) })
        );
        IntoResponse::new(self.client)
    }

    pub fn get_item<S: AsRef<str>>(&'a self, id: S) -> IntoResponse<'a, DriveItem> {
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
    ) -> IntoResponse<'a, DriveItem> {
        let body = serde_json::to_string(body);
        if let Ok(body) = body {
            self.client
                .builder()
                .set_method(Method::PATCH)
                .set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client, GraphFailure::from(e));
        }
        render_path!(
            self.client,
            template(id.as_ref(), "").as_str(),
            &json!({"id": encode(id.as_ref()) })
        );
        IntoResponse::new(self.client)
    }

    pub fn delete<S: AsRef<str>>(&'a self, id: S) -> IntoResponse<'a, GraphResponse<Content>> {
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
        body: &B,
    ) -> IntoResponse<'a, DriveItem> {
        let body = serde_json::to_string(body);
        if let Ok(body) = body {
            self.client
                .builder()
                .set_method(Method::POST)
                .set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client, GraphFailure::from(e));
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
        IntoResponse::new(self.client)
    }

    pub fn copy<S: AsRef<str>, B: serde::Serialize>(
        &'a self,
        id: S,
        body: &B,
    ) -> IntoResponse<'a, GraphResponse<Content>> {
        let body = serde_json::to_string(body);
        if let Ok(body) = body {
            self.client
                .builder()
                .set_method(Method::POST)
                .set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client, GraphFailure::from(e));
        }
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
    ) -> IntoResponse<'a, Collection<DriveItemVersion>> {
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
    ) -> IntoResponse<'a, Thumbnail> {
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
    ) -> IntoResponse<'a, Vec<u8>> {
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
    ) -> IntoResponse<'a, DriveItem> {
        let file = File::open(file).map_err(GraphFailure::from);
        if let Err(err) = file {
            return IntoResponse::new_error(self.client, err);
        }

        self.client
            .builder()
            .set_method(Method::PUT)
            .set_body(file.unwrap());
        render_path!(
            self.client,
            template(id.as_ref(), "content").as_str(),
            &json!({"id": encode(id.as_ref()) })
        );
        IntoResponse::new(self.client)
    }

    pub fn upload_new<S: AsRef<str>, P: AsRef<Path>>(
        &'a self,
        id: S,
        file: P,
    ) -> IntoResponse<'a, DriveItem> {
        if id.as_ref().starts_with(':') {
            let file = File::open(file).map_err(GraphFailure::from);
            if let Err(err) = file {
                return IntoResponse::new_error(self.client, err);
            }
            self.client
                .builder()
                .set_method(Method::PUT)
                .set_body(file.unwrap());
            render_path!(
                self.client,
                template(id.as_ref(), "content").as_str(),
                &json!({"id": encode(id.as_ref()) })
            );
        } else {
            let name = file.as_ref().file_name();
            if name.is_none() {
                return IntoResponse::new_error(self.client, GraphFailure::invalid("file_name"));
            }
            let name = name.unwrap().to_str();
            if name.is_none() {
                return IntoResponse::new_error(
                    self.client,
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

            let file = File::open(file).map_err(GraphFailure::from);
            if let Err(err) = file {
                return IntoResponse::new_error(self.client, err);
            }
            self.client
                .builder()
                .set_method(Method::PUT)
                .set_body(file.unwrap());
        }
        IntoResponse::new(self.client)
    }

    pub fn restore_version<S: AsRef<str>>(
        &'a self,
        id: S,
        version_id: S,
    ) -> IntoResponse<'a, GraphResponse<Content>> {
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
        body: &B,
    ) -> IntoResponse<'a, UploadSessionClient> {
        let body = serde_json::to_string(body);
        if let Ok(body) = body {
            self.client
                .builder()
                .set_method(Method::POST)
                .set_upload_session(file)
                .set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client, GraphFailure::from(e));
        }
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
    ) -> IntoResponse<'a, ItemPreviewInfo> {
        if let Some(body) = body {
            let body = serde_json::to_string(body);
            if let Ok(body) = body {
                self.client
                    .builder()
                    .set_method(Method::POST)
                    .set_body(body);
            } else if let Err(e) = body {
                return IntoResponse::new_error(self.client, GraphFailure::from(e));
            }
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

    pub fn content<S: AsRef<str>>(&'a self, id: S) -> IntoResponse<'a, GraphResponse<Content>> {
        render_path!(
            self.client,
            template(id.as_ref(), "content").as_str(),
            &json!({ "id": encode(id.as_ref()) })
        );
        self.client.builder().set_method(Method::GET);
        IntoResponse::new(self.client)
    }

    pub fn download<S: AsRef<str>, P: AsRef<Path>>(
        &'a self,
        id: S,
        directory: P,
    ) -> DownloadClient {
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

    pub fn check_out<S: AsRef<str>>(&'a self, id: S) -> IntoResponse<'a, GraphResponse<Content>> {
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
    ) -> IntoResponse<'a, GraphResponse<Content>> {
        render_path!(
            self.client,
            template(id.as_ref(), "checkin").as_str(),
            &json!({ "id": encode(id.as_ref()) })
        );

        let body = serde_json::to_string(body);
        if let Ok(body) = body {
            self.client
                .builder()
                .set_method(Method::POST)
                .set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client, GraphFailure::from(e));
        }
        IntoResponse::new(self.client)
    }

    pub fn move_item<S: AsRef<str>, B: serde::Serialize>(
        &'a self,
        id: S,
        body: &B,
    ) -> IntoResponse<'a, DriveItem> {
        let body = serde_json::to_string(body);
        if let Ok(body) = body {
            self.client
                .builder()
                .set_method(Method::PATCH)
                .set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client, GraphFailure::from(e));
        }
        render_path!(
            self.client,
            template(id.as_ref(), "").as_str(),
            &json!({ "id": id.as_ref() })
        );
        IntoResponse::new(self.client)
    }

    pub fn activities_by_interval<S: AsRef<str>>(
        &'a self,
        id: S,
        start: &str,
        end: Option<&str>,
        interval: &str,
    ) -> IntoResponse<'a, ItemActivityStat> {
        self.client.builder().set_method(Method::GET);
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
        IntoResponse::new(self.client)
    }
}
