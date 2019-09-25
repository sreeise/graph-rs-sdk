use crate::client::*;
use crate::drive::IntoDownloadClient;
use crate::http::{FetchClient, UploadSessionClient};
use crate::http::{GraphResponse, ResponseClient};
use crate::types::collection::Collection;
use crate::url::FormatOrd;
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

fn drive_ord_vec() -> Vec<FormatOrd> {
    vec![
        FormatOrd::Insert(UrlOrdering::RootOrItem("items".into())),
        FormatOrd::InsertNe(UrlOrdering::ItemPath("drive".into()), Ident::Drives),
    ]
}

fn ord_vec_last(last: &str) -> Vec<FormatOrd> {
    vec![
        FormatOrd::InsertNe(UrlOrdering::RootOrItem("drive".into()), Ident::Drives),
        FormatOrd::Insert(UrlOrdering::Last(last.into())),
    ]
}

fn ord_vec_last_items(last: &str) -> Vec<FormatOrd> {
    vec![
        FormatOrd::Insert(UrlOrdering::RootOrItem("items".into())),
        FormatOrd::InsertNe(UrlOrdering::ItemPath("drive".into()), Ident::Drives),
        FormatOrd::Insert(UrlOrdering::Last(last.into())),
    ]
}

client_struct!(DriveRequest);

impl<'a, I> DriveRequest<'a, I> {
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
    get!(get_item, DriveItem, drive_ord_vec(), false);
    patch!(update, DriveItem, drive_ord_vec(), true, ());
    delete!(delete, GraphResponse<()>, drive_ord_vec(), false);
    get!(
        drive,
        BaseItem,
        vec![FormatOrd::InsertNe(
            UrlOrdering::RootOrItem("drive".into()),
            Ident::Drives
        )]
    );
    get!(root, DriveItem, ord_vec_last("root"), true);
    get!(recent, Collection<DriveItem>, ord_vec_last("recent"), true);
    get!(delta, Collection<DriveItem>, ord_vec_last("root/delta"));
    get!(
        list_children,
        DriveItem,
        ord_vec_last_items("children"),
        false
    );
    get!(
        list_versions,
        Collection<DriveItem>,
        ord_vec_last_items("versions"),
        false
    );
    get!(
        item_activity,
        Collection<ItemActivity>,
        ord_vec_last_items("activities"),
        false
    );
    get!(
        drive_activity,
        Collection<ItemActivity>,
        vec![
            FormatOrd::InsertNe(UrlOrdering::RootOrItem("drive".into()), Ident::Drives),
            FormatOrd::Insert(UrlOrdering::Last("activities".into()))
        ]
    );
    get!(
        thumbnails,
        Collection<ThumbnailSet>,
        ord_vec_last_items("thumbnails"),
        false
    );
    get!(
        root_children,
        Collection<DriveItem>,
        ord_vec_last("root/children")
    );
    get!(
        shared_with_me,
        Collection<DriveItem>,
        ord_vec_last("sharedWithMe")
    );
    get!(
        special_documents,
        Collection<DriveItem>,
        ord_vec_last("special/documents")
    );
    get!(
        special_documents_children,
        Collection<DriveItem>,
        ord_vec_last("special/documents/children")
    );
    get!(
        special_photos,
        Collection<DriveItem>,
        ord_vec_last("special/photos")
    );
    get!(
        special_photos_children,
        Collection<DriveItem>,
        ord_vec_last("special/photos/children")
    );
    get!(
        special_camera_roll,
        Collection<DriveItem>,
        ord_vec_last("special/cameraroll")
    );
    get!(
        special_camera_roll_children,
        Collection<DriveItem>,
        ord_vec_last("special/cameraroll/children")
    );
    get!(
        special_app_root,
        Collection<DriveItem>,
        ord_vec_last("special/approot")
    );
    get!(
        special_app_root_children,
        Collection<DriveItem>,
        ord_vec_last("special/approot/children")
    );
    get!(
        special_music,
        Collection<DriveItem>,
        ord_vec_last("special/music")
    );
    get!(
        special_music_children,
        Collection<DriveItem>,
        ord_vec_last("special/music/children")
    );

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
