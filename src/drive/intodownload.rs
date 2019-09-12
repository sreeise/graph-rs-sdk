use crate::client::*;
use crate::http::{Download, FetchClient};
use crate::url::UrlOrdering;
use graph_error::{GraphFailure, GraphResult};
use graph_rs_types::entitytypes::DriveItem;
use std::ffi::OsString;

use std::marker::PhantomData;
use std::path::Path;

pub struct IntoDownloadClient<'a, I, T> {
    client: &'a Graph,
    item: PhantomData<T>,
    ident: PhantomData<I>,
}

impl<'a, I, T> IntoDownloadClient<'a, I, T> {
    pub fn new(client: &'a Graph) -> IntoDownloadClient<'a, I, T> {
        IntoDownloadClient {
            client,
            item: PhantomData,
            ident: PhantomData,
        }
    }

    pub fn by_id(&self, id: &str) -> GraphResult<FetchClient> {
        if self.client.ident().eq(&Ident::Me) {
            self.client.request().insert(UrlOrdering::Id(id.into()));
        } else {
            self.client
                .request()
                .insert(UrlOrdering::ResourceId(id.into()));
        }
        self.client.request().format_ord().download()
    }

    pub fn by_drive_item(&self, drive_item: &DriveItem) -> GraphResult<FetchClient> {
        if let Some(name) = drive_item.name.as_ref() {
            self.client.request().rename_download(OsString::from(name));
        }

        if let Some(url) = drive_item.download_url.as_ref() {
            self.client
                .request()
                .set_direct_download(true, url.as_str())
                .download()
        } else {
            let id = drive_item
                .id
                .as_ref()
                .ok_or_else(|| GraphFailure::none_err("drive_item -> item_id"))?;
            if self.client.ident().eq(&Ident::Me) {
                self.client
                    .request()
                    .insert(UrlOrdering::Id(id.to_string()))
                    .format_ord()
                    .download()
            } else {
                let rid = drive_item
                    .parent_reference
                    .as_ref()
                    .ok_or_else(|| GraphFailure::none_err("drive_item -> parent_reference"))?
                    .drive_id
                    .clone()
                    .ok_or_else(|| {
                        GraphFailure::none_err("drive_item -> parent_reference -> drive_id")
                    })?;

                self.client
                    .request()
                    .insert(UrlOrdering::ResourceId(rid))
                    .insert(UrlOrdering::Id(id.to_string()))
                    .format_ord()
                    .download()
            }
        }
    }
}

impl<'a> IntoDownloadClient<'a, IdentifyMe, FetchClient> {
    pub fn by_path<P: AsRef<Path>>(&'a self, path: P) -> GraphResult<FetchClient> {
        self.client
            .request()
            .remove(UrlOrdering::FileName("".into()))
            .replace(UrlOrdering::RootOrItem("root:".into()))
            .replace(UrlOrdering::Path(path.as_ref().to_path_buf()))
            .format_ord()
            .download()
    }
}

impl<'a> IntoDownloadClient<'a, IdentifyCommon, FetchClient> {
    pub fn by_ids(&'a self, item_id: &str, resource_id: &str) -> GraphResult<FetchClient> {
        self.client
            .request()
            .insert(UrlOrdering::ResourceId(resource_id.into()))
            .insert(UrlOrdering::Id(item_id.into()))
            .format_ord()
            .download()
    }

    pub fn by_path_id<P: AsRef<Path>>(
        &'a self,
        resource_id: &str,
        path: P,
    ) -> GraphResult<FetchClient> {
        self.client
            .request()
            .insert(UrlOrdering::ResourceId(resource_id.into()))
            .remove(UrlOrdering::FileName("".into()))
            .replace(UrlOrdering::RootOrItem("root:".into()))
            .insert(UrlOrdering::Path(path.as_ref().to_path_buf()))
            .format_ord()
            .download()
    }
}
