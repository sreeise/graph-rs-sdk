use crate::client::*;
use crate::http::{FetchClient, UploadSessionClient};
use crate::types::statusresponse::StatusResponse;
use graph_error::{GraphError, GraphFailure, GraphResult};
use graph_rs_types::complextypes::UploadSession;
use graph_rs_types::entitytypes::DriveItem;
use std::convert::TryFrom;
use std::ffi::OsString;
use std::marker::PhantomData;
use std::path::Path;

pub trait IntoItem<T> {
    fn send(&self) -> GraphResult<T>;
}

pub struct IntoResponse<'a, I, T> {
    client: &'a Graph,
    ident: PhantomData<I>,
    phantom: PhantomData<T>,
}

impl<'a, I, T> IntoResponse<'a, I, T> {
    pub fn new(client: &'a Graph) -> IntoResponse<I, T> {
        IntoResponse {
            client,
            ident: PhantomData,
            phantom: PhantomData,
        }
    }

    pub fn send_as_value(&self) -> GraphResult<serde_json::Value> {
        self.client.send()
    }

    pub fn json<U>(&self) -> GraphResult<U>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        self.client.send()
    }
}

impl<'a, I> IntoItem<StatusResponse> for IntoResponse<'a, I, StatusResponse> {
    fn send(&self) -> GraphResult<StatusResponse> {
        self.client.status_response()
    }
}

impl<'a, I, T> IntoItem<T> for IntoResponse<'a, I, T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    fn send(&self) -> Result<T, GraphFailure> {
        self.client.send()
    }
}

impl<'a, I> IntoItem<UploadSessionClient> for IntoResponse<'a, I, UploadSessionClient> {
    fn send(&self) -> GraphResult<UploadSessionClient> {
        let builder = self.client.request.borrow_mut().builder();
        let mut response = builder.send()?;
        let status = response.status().as_u16();
        if GraphError::is_error(status) {
            return Err(GraphFailure::try_from(&mut response).unwrap_or_default());
        }

        let upload_session: UploadSession = response.json()?;
        let mut session = UploadSessionClient::new(upload_session)?;
        let file = self
            .client
            .request
            .borrow()
            .upload_session_file
            .clone()
            .ok_or_else(|| GraphFailure::none_err("file for upload session"))?;
        session.set_file(file)?;
        Ok(session)
    }
}

macro_rules! drive_item_ids {
    ( $drive_item:expr ) => {{
        let resource_id = $drive_item
            .parent_reference
            .as_ref()
            .ok_or_else(|| GraphFailure::none_err("drive_item -> parent_reference"))?
            .drive_id
            .clone()
            .ok_or_else(|| GraphFailure::none_err("drive_item -> parent_reference -> drive_id"))?;
        let item_id = $drive_item
            .id
            .as_ref()
            .ok_or_else(|| GraphFailure::none_err("drive_item -> item_id"))?;
        (item_id.to_string(), resource_id.to_string())
    }};
}

pub struct ResponseClient<'a, I, T> {
    client: &'a Graph,
    item: PhantomData<T>,
    ident: PhantomData<I>,
}

impl<'a, I, T> ResponseClient<'a, I, T> {
    pub fn new(client: &'a Graph) -> ResponseClient<'a, I, T> {
        ResponseClient {
            client,
            item: PhantomData,
            ident: PhantomData,
        }
    }

    pub fn by_id(&'a self, id: &str) -> IntoResponse<'a, I, T>
    {
        if self.client.ident().eq(&Ident::Me) {
            self.client.insert_ord(UrlOrdering::Id(id.into()))
                .format();
        } else {
            self.client.insert_ord(UrlOrdering::ResourceId(id.into()))
                .format();
        }
        IntoResponse::new(self.client)
    }

    pub fn send_as_value(&self) -> GraphResult<serde_json::Value> {
        self.client.send()
    }

    pub fn json<U>(&self) -> GraphResult<U>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        self.client.send()
    }
}

impl<'a, T> ResponseClient<'a, IdentifyMe, T> {
    pub fn by_path<P: AsRef<Path>>(&self, path: P) -> IntoResponse<'a, IdentifyMe, T>
    {
        self.client
            .remove_ord(UrlOrdering::FileName("".into()))
            .replace_ord(UrlOrdering::RootOrItem("root:".into()))
            .replace_ord(UrlOrdering::Path(path.as_ref().to_path_buf()))
            .format();
        IntoResponse::new(self.client)
    }
}

impl<'a, T> ResponseClient<'a, IdentifyCommon, T> {
    pub fn by_ids(&'a self, item_id: &str, resource_id: &str) -> IntoResponse<'a, IdentifyCommon, T>
    {
        self.client
            .insert_ord(UrlOrdering::ResourceId(resource_id.into()))
            .insert_ord(UrlOrdering::Id(item_id.into()))
            .format();
        IntoResponse::new(self.client)
    }

    pub fn by_path_id<P: AsRef<Path>>(&'a self, resource_id: &str, path: P) -> IntoResponse<'a, IdentifyCommon, T>
    {
        self.client
            .insert_ord(UrlOrdering::ResourceId(resource_id.into()))
            .remove_ord(UrlOrdering::FileName("".into()))
            .replace_ord(UrlOrdering::RootOrItem("root:".into()))
            .insert_ord(UrlOrdering::Path(path.as_ref().to_path_buf()))
            .format();
        IntoResponse::new(self.client)
    }
}

// For requests that don't return JSON in the body but does return information
// on triggering status events.
impl<'a, I> ResponseClient<'a, I, StatusResponse> {
    pub fn send(&self) -> GraphResult<StatusResponse> {
        self.client.status_response()
    }
}

impl<'a, I, T> IntoItem<T> for ResponseClient<'a, I, T>
    where
            for<'de> T: serde::Deserialize<'de>,
{
    fn send(&self) -> Result<T, GraphFailure> {
        self.client.send()
    }
}

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
            self.client.insert_ord(UrlOrdering::Id(id.into()));
        } else {
            self.client.insert_ord(UrlOrdering::ResourceId(id.into()));
        }
        self.client.format().download_client()
    }

    pub fn by_drive_item(&self, drive_item: &DriveItem) -> GraphResult<FetchClient> {
        let mut download_client = self.client.download_client()?;

        if let Some(name) = drive_item.name.as_ref() {
            download_client.rename(OsString::from(name.to_string()));
        }

        if let Some(url) = drive_item.download_url.as_ref() {
            self.client.set_direct_download(true, url.as_str());
            Ok(download_client)
        } else {
            let (id, rid) = drive_item_ids!(drive_item);
            self.client
                .insert_ord(UrlOrdering::ResourceId(rid))
                .insert_ord(UrlOrdering::Id(id))
                .format();
            Ok(download_client)
        }
    }
}

impl<'a> IntoDownloadClient<'a, IdentifyMe, FetchClient> {
    pub fn by_path<P: AsRef<Path>>(&'a self, path: P) -> GraphResult<FetchClient> {
        self.client
            .remove_ord(UrlOrdering::FileName("".into()))
            .replace_ord(UrlOrdering::RootOrItem("root:".into()))
            .replace_ord(UrlOrdering::Path(path.as_ref().to_path_buf()))
            .format()
            .download_client()
    }
}

impl<'a> IntoDownloadClient<'a, IdentifyCommon, FetchClient> {
    pub fn by_ids(&'a self, item_id: &str, resource_id: &str) -> GraphResult<FetchClient> {
        self.client
            .insert_ord(UrlOrdering::ResourceId(resource_id.into()))
            .insert_ord(UrlOrdering::Id(item_id.into()))
            .format()
            .download_client()
    }

    pub fn by_path_id<P: AsRef<Path>>(
        &'a self,
        resource_id: &str,
        path: P,
    ) -> GraphResult<FetchClient> {
        self.client
            .insert_ord(UrlOrdering::ResourceId(resource_id.into()))
            .remove_ord(UrlOrdering::FileName("".into()))
            .replace_ord(UrlOrdering::RootOrItem("root:".into()))
            .insert_ord(UrlOrdering::Path(path.as_ref().to_path_buf()))
            .format()
            .download_client()
    }
}
