use crate::client::*;
use crate::http::UploadSessionClient;
use crate::types::statusresponse::StatusResponse;
use graph_error::{GraphError, GraphFailure, GraphResult};
use graph_rs_types::complextypes::UploadSession;
use graph_rs_types::entitytypes::DriveItem;
use std::convert::TryFrom;
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

    pub fn by_id(&'a self, id: &str) -> IntoResponse<'a, I, T> {
        if self.client.ident().eq(&Ident::Me) {
            self.client
                .insert_ord(UrlOrdering::Id(id.into()))
                .format_ord();
        } else {
            self.client
                .insert_ord(UrlOrdering::ResourceId(id.into()))
                .format_ord();
        }
        IntoResponse::new(self.client)
    }

    pub fn by_drive_item(&'a self, drive_item: &DriveItem) -> GraphResult<IntoResponse<'a, I, T>> {
        let (id, rid) = drive_item_ids!(drive_item);
        self.client
            .insert_ord(UrlOrdering::ResourceId(rid))
            .insert_ord(UrlOrdering::Id(id))
            .format_ord();
        Ok(IntoResponse::new(self.client))
    }

    pub fn send_as_value(&self) -> GraphResult<serde_json::Value> {
        self.client.format_ord().send()
    }

    pub fn json<U>(&self) -> GraphResult<U>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        self.client.format_ord().send()
    }
}

impl<'a, T> ResponseClient<'a, IdentifyMe, T> {
    pub fn by_path<P: AsRef<Path>>(&self, path: P) -> IntoResponse<'a, IdentifyMe, T> {
        self.client
            .remove_ord(UrlOrdering::FileName("".into()))
            .replace_ord(UrlOrdering::RootOrItem("root:".into()))
            .replace_ord(UrlOrdering::Path(path.as_ref().to_path_buf()))
            .format_ord();
        IntoResponse::new(self.client)
    }
}

impl<'a, T> ResponseClient<'a, IdentifyCommon, T> {
    pub fn by_ids(
        &'a self,
        item_id: &str,
        resource_id: &str,
    ) -> IntoResponse<'a, IdentifyCommon, T> {
        self.client
            .insert_ord(UrlOrdering::ResourceId(resource_id.into()))
            .insert_ord(UrlOrdering::Id(item_id.into()))
            .format_ord();
        IntoResponse::new(self.client)
    }

    pub fn by_path_id<P: AsRef<Path>>(
        &'a self,
        resource_id: &str,
        path: P,
    ) -> IntoResponse<'a, IdentifyCommon, T> {
        self.client
            .insert_ord(UrlOrdering::ResourceId(resource_id.into()))
            .remove_ord(UrlOrdering::FileName("".into()))
            .replace_ord(UrlOrdering::RootOrItem("root:".into()))
            .insert_ord(UrlOrdering::Path(path.as_ref().to_path_buf()))
            .format_ord();
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
