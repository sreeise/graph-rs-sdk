use crate::client::*;
use crate::drive::IntoDownloadClient;
use crate::http::{FetchClient, UploadSessionClient};
use crate::http::{GraphResponse, ResponseClient};
use crate::types::collection::Collection;
use crate::url::UrlOrdering;
use graph_error::GraphFailure;
use graph_error::GraphResult;
use graph_rs_types::complextypes::{ItemPreviewInfo, Thumbnail};
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

macro_rules! item_method {
    ( $name:ident, $I:ty, $x:expr, $m:expr ) => {
      pub fn $name(&self) -> ResponseClient<'a, I, $I> {
        self.client.request().set_method($m);
        self.update_ord();
        if !$x.is_empty() {
            self.client.request().insert(UrlOrdering::Last($x.to_string()));
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
    item_method!(get_item, DriveItem, "", Method::GET);
    item_method!(delete, GraphResponse<()>, "", Method::DELETE);
    endpoint_method!(drive, BaseItem, "drive");
    endpoint_method!(root, DriveItem, "root");
    endpoint_method!(recent, Collection<DriveItem>, "recent");
    endpoint_method!(delta, Collection<DriveItem>, "root/delta");
    item_method!(list_children, DriveItem, "children", Method::GET);
    item_method!(
        list_versions,
        Collection<DriveItem>,
        "versions",
        Method::GET
    );
    item_method!(
        item_activity,
        Collection<ItemActivity>,
        "activities",
        Method::GET
    );
    endpoint_method!(drive_activity, Collection<ItemActivity>, "activities");
    item_method!(
        thumbnails,
        Collection<ThumbnailSet>,
        "thumbnails",
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

    pub fn update<T: serde::Serialize>(
        &'a self,
        drive_item: &T,
    ) -> ResponseClient<'a, I, DriveItem> {
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

    pub fn copy<T: serde::Serialize>(
        &'a self,
        name: Option<&str>,
        item_ref: &T,
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

    pub fn upload_session<P: AsRef<Path>, T: serde::Serialize>(
        &'a self,
        file: P,
        body: T,
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

    pub fn preview<T: serde::Serialize>(
        &'a self,
        embeddable_url: Option<&T>,
    ) -> ResponseClient<'a, I, ItemPreviewInfo> {
        if let Some(embeddable_url) = embeddable_url {
            self.client
                .request()
                .set_body(serde_json::to_string(embeddable_url).unwrap());
        } else {
            self.client
                .request()
                .header(CONTENT_LENGTH, HeaderValue::from(0));
        }
        self.update_ord_with(UrlOrdering::Last("preview".into()));
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
