use crate::client::*;
use crate::drive::driveevent::DriveEvent;
use crate::drive::IntoDownloadClient;
use crate::http::{FetchClient, Session, UploadSessionClient};
use crate::http::{GraphResponse, ResponseClient};
use crate::types::collection::Collection;
use crate::types::embeddableurl::EmbeddableUrl;
use crate::url::UrlOrdering;
use graph_error::GraphFailure;
use graph_error::GraphResult;
use graph_rs_types::complextypes::{ItemPreviewInfo, ItemReference, Thumbnail};
use graph_rs_types::entitytypes::{BaseItem, DriveItem, ItemActivity, ThumbnailSet};
use reqwest::header::{HeaderValue, CONTENT_LENGTH};
use reqwest::Method;
use serde::export::PhantomData;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

macro_rules! endpoint_method {
    ( $name:ident, $I:ty, $x:expr ) => {
      pub fn $name(&self) -> ResponseClient<'a, I, $I> {
        if !$x.eq("drive") && self.client.ident().ne(&Ident::Drives) {
            self.client.request().insert(UrlOrdering::ItemPath("drive".into()));
        }
        self.client.request().insert(UrlOrdering::Last($x.to_string()));
        if self.client.ident().eq(&Ident::Me) {
            self.client
                .request()
                .format_ord();
        }
        self.client.request().set_method(Method::GET);
        ResponseClient::new(self.client)
      }
    };
}

macro_rules! event_method {
    ( $name:ident, $I:ty, $x:expr, $m:expr ) => {
      pub fn $name(&self) -> ResponseClient<'a, I, $I> {
        self.client.request().set_method($m);
        self.update_ord();
        let s: &str = $x.as_ref();
        if !s.is_empty() {
            self.client.request().insert(UrlOrdering::Last(s.to_string()));
        }
        ResponseClient::new(self.client)
      }
    };
}

pub struct DriveRequest<'a, I> {
    client: &'a Graph,
    ident: PhantomData<I>,
}

impl<'a, I> DriveRequest<'a, I> {
    pub fn new(client: &'a Graph) -> DriveRequest<'a, I> {
        DriveRequest {
            client,
            ident: PhantomData,
        }
    }

    fn update_ord(&self) {
        self.client
            .request()
            .insert(UrlOrdering::RootOrItem("items".into()));
        if self.client.ident().ne(&Ident::Drives) {
            self.client
                .request()
                .insert(UrlOrdering::ItemPath("drive".into()));
        }
    }

    fn update_ord_with(&self, url_ord: UrlOrdering) {
        self.update_ord();
        self.client.request().insert(url_ord);
    }
}

impl<'a, I> DriveRequest<'a, I> {
    event_method!(get_item, DriveItem, DriveEvent::GetItem, Method::GET);
    event_method!(delete, (), DriveEvent::Delete, Method::DELETE);
    endpoint_method!(drive, BaseItem, "drive");
    endpoint_method!(root, DriveItem, "root");
    endpoint_method!(recent, Collection<DriveItem>, "recent");
    endpoint_method!(delta, Collection<DriveItem>, "root/delta");
    event_method!(
        list_children,
        DriveItem,
        DriveEvent::ListChildren,
        Method::GET
    );
    event_method!(
        list_versions,
        Collection<DriveItem>,
        DriveEvent::ListVersions,
        Method::GET
    );
    event_method!(
        list_item_activities,
        Collection<ItemActivity>,
        DriveEvent::Activities,
        Method::GET
    );
    event_method!(
        thumbnails,
        Collection<ThumbnailSet>,
        DriveEvent::Thumbnails,
        Method::GET
    );
    endpoint_method!(root_children, Collection<DriveItem>, "root/children");
    endpoint_method!(shared_with_me, Collection<DriveItem>, "sharedWithMe");
    endpoint_method!(
        special_documents,
        Collection<DriveItem>,
        "special/documents"
    );
    endpoint_method!(
        special_documents_child,
        Collection<DriveItem>,
        "special/documents/children"
    );
    endpoint_method!(special_photos, Collection<DriveItem>, "special/photos");
    endpoint_method!(
        special_photos_child,
        Collection<DriveItem>,
        "special/photos/children"
    );
    endpoint_method!(
        special_camera_roll,
        Collection<DriveItem>,
        "special/cameraroll"
    );
    endpoint_method!(
        special_camera_roll_child,
        Collection<DriveItem>,
        "special/cameraroll/children"
    );
    endpoint_method!(special_app_root, Collection<DriveItem>, "special/approot");
    endpoint_method!(
        special_app_root_child,
        Collection<DriveItem>,
        "special/approot/children"
    );
    endpoint_method!(special_music, Collection<DriveItem>, "special/music");
    endpoint_method!(
        special_music_child,
        Collection<DriveItem>,
        "special/music/children"
    );
    endpoint_method!(
        list_drive_activities,
        Collection<ItemActivity>,
        "activities"
    );

    pub fn update(&'a self, drive_item: &DriveItem) -> ResponseClient<'a, I, DriveItem> {
        self.update_ord();
        self.client
            .request()
            .set_method(Method::PATCH)
            .set_body(serde_json::to_string_pretty(drive_item).unwrap());
        ResponseClient::new(self.client)
    }

    pub fn create_folder(
        &'a self,
        name: &str,
        conflict_behavior: Option<&str>,
    ) -> ResponseClient<'a, I, DriveItem> {
        let folder: HashMap<String, serde_json::Value> = HashMap::new();
        if let Some(c) = conflict_behavior {
            let data =
                json!({ "name": name, "folder": folder,  "microsoft_graph_conflict_behavior": c });
            self.client
                .request()
                .set_method(Method::POST)
                .set_body(serde_json::to_string(&data).unwrap());
        } else {
            let data = json!({ "name": name, "folder": folder });
            self.client
                .request()
                .set_method(Method::POST)
                .set_body(serde_json::to_string(&data).unwrap());
        }
        self.update_ord_with(UrlOrdering::Last("children".into()));
        ResponseClient::new(self.client)
    }

    pub fn copy(
        &'a self,
        name: Option<&str>,
        item_ref: &ItemReference,
    ) -> ResponseClient<'a, I, GraphResponse<()>> {
        if let Some(name) = name {
            let data = json!({ "name": name, "parent_reference": item_ref });
            self.client
                .request()
                .set_method(Method::POST)
                .set_body(serde_json::to_string(&data).unwrap());
        } else {
            let data = json!({ "parent_reference": item_ref });
            self.client
                .request()
                .set_method(Method::POST)
                .set_body(serde_json::to_string(&data).unwrap());
        }
        ResponseClient::new(self.client)
    }

    pub fn single_thumbnail(
        &'a self,
        thumb_id: &str,
        size: &str,
    ) -> ResponseClient<'a, I, Thumbnail> {
        self.update_ord_with(UrlOrdering::Last(format!(
            "{}/{}/{}",
            "thumbnails", thumb_id, size
        )));
        self.client.request().set_method(Method::GET);
        ResponseClient::new(self.client)
    }

    pub fn thumbnail_binary(
        &'a self,
        thumb_id: &str,
        size: &str,
    ) -> ResponseClient<'a, I, Vec<u8>> {
        self.update_ord_with(UrlOrdering::Last(format!(
            "{}/{}/{}/{}",
            "thumbnails", thumb_id, size, "content"
        )));
        self.client.request().set_method(Method::GET);
        ResponseClient::new(self.client)
    }

    pub fn activities_from_list_item(
        &'a self,
        _list_id: &str,
    ) -> ResponseClient<'a, I, Collection<ItemActivity>> {
        self.update_ord_with(UrlOrdering::Last(format!(
            "{}/activities",
            DriveEvent::Activities.as_ref()
        )));
        self.client.request().set_method(Method::GET);
        ResponseClient::new(self.client)
    }

    pub fn upload_replace<P: AsRef<Path>>(&'a self, file: P) -> ResponseClient<'a, I, DriveItem> {
        self.update_ord();
        self.client
            .request()
            .set_method(Method::PUT)
            .insert(UrlOrdering::Last("content".into()))
            .set_body(File::open(file).unwrap());
        ResponseClient::new(self.client)
    }

    pub fn upload_new<P: AsRef<Path>>(
        &'a self,
        file: P,
    ) -> GraphResult<ResponseClient<'a, I, DriveItem>> {
        let name = file
            .as_ref()
            .file_name()
            .ok_or_else(|| GraphFailure::none_err("file_name"))?
            .to_string_lossy()
            .to_string();
        self.update_ord();
        self.client
            .request()
            .set_method(Method::PUT)
            .set_body(File::open(file).unwrap())
            .insert(UrlOrdering::FileName(name))
            .insert(UrlOrdering::Last("content".to_string()));
        Ok(ResponseClient::new(self.client))
    }

    pub fn restore_version(&'a self, version_id: &str) -> ResponseClient<'a, I, GraphResponse<()>> {
        self.update_ord_with(UrlOrdering::Last(format!(
            "{}/{}/{}",
            "versions", version_id, "restoreVersion",
        )));
        ResponseClient::new(self.client)
    }

    pub fn upload_session<P: AsRef<Path>>(
        &'a self,
        file: P,
        body: Session,
    ) -> ResponseClient<'a, I, UploadSessionClient> {
        self.client
            .request()
            .set_method(Method::POST)
            .set_upload_session(file)
            .insert(UrlOrdering::Last("createUploadSession".into()))
            .set_body(serde_json::to_string(&json!({ "item": body })).unwrap());
        self.update_ord();
        ResponseClient::new(self.client)
    }

    pub fn preview(
        &'a self,
        embeddable_url: Option<EmbeddableUrl>,
    ) -> ResponseClient<'a, I, ItemPreviewInfo> {
        if let Some(embeddable_url) = embeddable_url {
            self.client
                .request()
                .set_body(serde_json::to_string(&embeddable_url).unwrap());
        } else {
            self.client
                .request()
                .header(CONTENT_LENGTH, HeaderValue::from(0));
        }
        self.update_ord_with(UrlOrdering::Last(DriveEvent::Preview.to_string()));
        ResponseClient::new(self.client)
    }

    pub fn download<P: AsRef<Path>>(
        &'a self,
        directory: P,
    ) -> IntoDownloadClient<'a, I, FetchClient> {
        self.update_ord_with(UrlOrdering::Last("content".into()));
        self.client
            .request()
            .download_request
            .set_directory(PathBuf::from(directory.as_ref()));
        IntoDownloadClient::new(self.client)
    }

    pub fn check_out(&'a self) -> ResponseClient<'a, I, GraphResponse<()>> {
        self.update_ord();
        self.client
            .request()
            .set_method(Method::POST)
            .insert(UrlOrdering::Last("checkout".into()))
            .header(CONTENT_LENGTH, HeaderValue::from(0));
        ResponseClient::new(self.client)
    }

    pub fn check_in(
        &'a self,
        check_in_as: Option<&str>,
        comment: Option<&str>,
    ) -> ResponseClient<'a, I, GraphResponse<()>> {
        self.update_ord();
        if let Some(check_in_as) = check_in_as {
            if let Some(comment) = comment {
                self.client.request().set_body(
                    serde_json::to_string_pretty(
                        &json!({ "checkInAs": check_in_as, "comment": comment }),
                    )
                    .unwrap(),
                );
            } else {
                self.client.request().set_body(
                    serde_json::to_string_pretty(&json!({ "checkInAs": check_in_as })).unwrap(),
                );
            }
        } else if let Some(comment) = comment {
            self.client
                .request()
                .set_body(serde_json::to_string_pretty(&json!({ "comment": comment })).unwrap());
        } else {
            self.client
                .request()
                .header(CONTENT_LENGTH, HeaderValue::from(0));
        }
        self.client
            .request()
            .insert(UrlOrdering::Last("checkin".into()))
            .set_method(Method::POST);
        ResponseClient::new(self.client)
    }
}
