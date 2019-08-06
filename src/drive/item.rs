use crate::drive;
use crate::drive::drive_item::asyncjobstatus::AsyncJobStatus;
use crate::drive::drive_item::driveitem::DriveItem;
use crate::drive::drive_item::itemreference::ItemReference;
use crate::drive::drive_item::thumbnail::{Thumbnail, ThumbnailCollection};
use crate::drive::driverequest::ReqBuilder;
use crate::drive::event::ItemRefCopy;
use crate::drive::event::{DriveEvent, NewFolder};
use crate::drive::intoitem::IntoItem;
use crate::drive::item::inner_pipeline::MutatePipeline;
use crate::drive::pipeline::{Body, DataPipeline, DownloadPipeline, FetchPipeline, Pipeline};
use crate::drive::ItemResult;
use crate::prelude::{DriveUrl, MutateUrl};
use graph_error::GraphError;
use graph_error::GraphFailure;
use reqwest::{header, Response};
use std::convert::TryFrom;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ItemResponse {
    event: DriveEvent,
    response: Response,
}

impl ItemResponse {
    pub fn new(event: DriveEvent, response: Response) -> ItemResponse {
        ItemResponse { event, response }
    }

    pub fn drive_event(&self) -> DriveEvent {
        self.event
    }

    pub fn status(&self) -> u16 {
        self.response.status().as_u16()
    }

    pub fn async_job_status(&mut self) -> ItemResult<Option<AsyncJobStatus>> {
        let headers = self.response.headers();
        // The location header contains the URL for monitoring progress.
        let option_location: Option<&reqwest::header::HeaderValue> = headers.get(header::LOCATION);
        if let Some(location) = option_location {
            let location_str = location.to_str().map_err(GraphFailure::from)?;
            let client = reqwest::Client::builder().build()?;
            let mut response = client.get(location_str).send()?;

            let status = response.status().as_u16();
            if GraphError::is_error(status) {
                return Err(GraphFailure::from(
                    GraphError::try_from(status).unwrap_or_default(),
                ));
            }

            let progress: AsyncJobStatus = response.json()?;
            Ok(Some(progress))
        } else {
            Ok(None)
        }
    }

    pub fn success(&mut self) -> bool {
        match self.event {
            DriveEvent::Copy => self.status() == 202,
            _ => self.status() == 204,
        }
    }

    pub fn response(&self) -> &Response {
        &self.response
    }

    pub fn error(&self) -> Option<GraphError> {
        if GraphError::is_error(self.status()) {
            return Some(GraphError::try_from(self.status()).unwrap_or_default());
        }
        None
    }
}

pub struct BoxItemResponse {
    item: Box<dyn IntoItem<ItemResponse>>,
}

impl BoxItemResponse {
    fn new(item: Box<dyn IntoItem<ItemResponse>>) -> BoxItemResponse {
        BoxItemResponse { item }
    }

    pub fn send(&mut self) -> ItemResult<ItemResponse> {
        self.item.send()
    }
}

impl AsMut<DriveUrl> for BoxItemResponse {
    fn as_mut(&mut self) -> &mut DriveUrl {
        self.item.as_mut().as_mut()
    }
}

impl AsRef<DriveUrl> for BoxItemResponse {
    fn as_ref(&self) -> &DriveUrl {
        self.item.as_ref().as_ref()
    }
}

pub struct Request<T> {
    item: Box<dyn IntoItem<T>>,
}

impl<T> Request<T> {
    fn new(item: Box<dyn IntoItem<T>>) -> Request<T> {
        Request { item }
    }

    pub fn send(&mut self) -> ItemResult<T> {
        self.item.send()
    }
}

impl<T> From<Pipeline> for Request<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    fn from(pipeline: Pipeline) -> Self {
        Request::new(Box::new(pipeline))
    }
}

impl<T> MutateUrl for Request<T> {}

impl<T> AsRef<DriveUrl> for Request<T> {
    fn as_ref(&self) -> &DriveUrl {
        self.item.as_ref().as_ref()
    }
}

impl<T> AsMut<DriveUrl> for Request<T> {
    fn as_mut(&mut self) -> &mut DriveUrl {
        self.item.as_mut().as_mut()
    }
}

pub struct SelectEventMe(DataPipeline);

impl AsRef<DriveUrl> for SelectEventMe {
    fn as_ref(&self) -> &DriveUrl {
        &self.0.url
    }
}

impl AsMut<DriveUrl> for SelectEventMe {
    fn as_mut(&mut self) -> &mut DriveUrl {
        &mut self.0.url
    }
}

impl AsMut<DataPipeline> for SelectEventMe {
    fn as_mut(&mut self) -> &mut DataPipeline {
        &mut self.0
    }
}

impl ItemMe for SelectEventMe {}

pub struct SelectEvent(DataPipeline);

impl AsRef<DriveUrl> for SelectEvent {
    fn as_ref(&self) -> &DriveUrl {
        &self.0.url
    }
}

impl AsMut<DriveUrl> for SelectEvent {
    fn as_mut(&mut self) -> &mut DriveUrl {
        &mut self.0.url
    }
}

impl AsMut<DataPipeline> for SelectEvent {
    fn as_mut(&mut self) -> &mut DataPipeline {
        &mut self.0
    }
}

impl From<SelectResource> for SelectEvent {
    fn from(s: SelectResource) -> Self {
        SelectEvent(s.0)
    }
}

impl ItemCommon for SelectEvent {}

#[derive(Clone, PartialEq)]
pub struct SelectResource(DataPipeline);

impl SelectResource {
    pub fn new(data: DataPipeline) -> SelectResource {
        SelectResource(data)
    }
}

impl SelectResource {
    pub fn me(mut self) -> SelectEventMe {
        self.extend_path(&["me"]);
        self.into()
    }

    pub fn drives(mut self) -> SelectEvent {
        self.extend_path(&["drives"]);
        self.into()
    }

    pub fn sites(mut self) -> SelectEvent {
        self.extend_path(&["sites"]);
        self.into()
    }

    pub fn groups(mut self) -> SelectEvent {
        self.extend_path(&["groups"]);
        self.into()
    }

    pub fn users(mut self) -> SelectEvent {
        self.extend_path(&["users"]);
        self.into()
    }

    pub fn get(&mut self) -> ReqBuilder {
        self.0.as_get();
        ReqBuilder::new(self.pipeline_data())
    }

    pub fn post(&mut self) -> ReqBuilder {
        self.0.as_post();
        ReqBuilder::new(self.pipeline_data())
    }

    pub fn put(&mut self) -> ReqBuilder {
        self.0.as_put();
        ReqBuilder::new(self.pipeline_data())
    }

    pub fn patch(&mut self) -> ReqBuilder {
        self.0.as_patch();
        ReqBuilder::new(self.pipeline_data())
    }

    pub fn delete(&mut self) -> ReqBuilder {
        self.0.as_delete();
        ReqBuilder::new(self.pipeline_data())
    }
}

impl From<SelectResource> for SelectEventMe {
    fn from(resource: SelectResource) -> Self {
        SelectEventMe(resource.0)
    }
}

impl AsMut<DataPipeline> for SelectResource {
    fn as_mut(&mut self) -> &mut DataPipeline {
        &mut self.0
    }
}

pub trait ItemMe: MutatePipeline + AsMut<DriveUrl> + AsMut<DataPipeline> {
    fn delete(&mut self, item_id: &str) -> BoxItemResponse {
        self.format_me(item_id);
        self.as_delete();
        BoxItemResponse::new(Box::new(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::Delete,
        )))
    }

    fn delete_drive_item(&mut self, value: &DriveItem) -> BoxItemResponse {
        let item_id = value.id().unwrap();
        self.delete(item_id.as_str())
    }

    fn get_item(&mut self, item_id: &str) -> Request<DriveItem> {
        self.format_me(item_id);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::GetItem))
    }

    fn get_item_path(&mut self, path: &str) -> Request<DriveItem> {
        self.extend_path(&["drive", "root:", path]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::GetItem))
    }

    fn create_folder(&mut self, parent_id: &str, folder: NewFolder) -> Request<DriveItem> {
        self.format_me(parent_id);
        self.event(DriveEvent::CreateFolder);
        self.body(Body::String(serde_json::to_string_pretty(&folder).unwrap()));
        self.as_post();
        Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::CreateFolder,
        ))
    }

    fn create_folder_path(
        &mut self,
        new_folder: NewFolder,
        path_from_root: String,
    ) -> Request<DriveItem> {
        let mut s = path_from_root;
        if !s.ends_with(':') {
            s.push(':');
        }

        self.format_me(s.as_str());
        self.extend_path(&["children"]);
        self.body(Body::String(
            serde_json::to_string_pretty(&new_folder).unwrap(),
        ));
        self.as_post();
        Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::CreateFolder,
        ))
    }

    fn copy(
        &mut self,
        item_id: &str,
        item_ref: &ItemReference,
        name: Option<&str>,
    ) -> BoxItemResponse {
        if let Some(name) = name {
            let prc = ItemRefCopy::new(item_ref.clone(), Some(name.into()));
            self.body(Body::String(prc.as_json().unwrap()));
        } else {
            let prc = ItemRefCopy::new(item_ref.clone(), None);
            self.body(Body::String(prc.as_json().unwrap()));
        }

        self.format_me(item_id);
        self.extend_path(&["copy"]);
        self.as_post();
        BoxItemResponse::new(Box::new(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::Copy,
        )))
    }

    fn copy_drive_item(
        &mut self,
        drive_item: &DriveItem,
        item_ref: &ItemReference,
        name: Option<&str>,
    ) -> ItemResult<BoxItemResponse> {
        let item_id = drive_item
            .id()
            .ok_or_else(|| GraphFailure::none_err("item_id"))?;
        Ok(self.copy(item_id.as_str(), item_ref, name))
    }

    fn download<P: AsRef<Path>>(&mut self, item_id: &str, directory: P) -> DownloadPipeline {
        self.format_me(item_id);
        self.extend_path(&["content"]);

        let mut fetch_pipeline = FetchPipeline::from(self.pipeline_data());
        let mut path_buf = PathBuf::new();
        path_buf.push(directory);
        fetch_pipeline.directory = path_buf;

        DownloadPipeline {
            pipeline: fetch_pipeline,
            is_direct_download: false,
        }
    }

    fn download_drive_item<P: AsRef<Path>>(
        &mut self,
        value: &mut DriveItem,
        directory: P,
    ) -> ItemResult<DownloadPipeline> {
        if let Some(download_url) = value.microsoft_graph_download_url() {
            // Build the non-download url in the case that a format has been
            // selected after this. The format download always uses the redirect.
            if let Some(item_id) = value.id() {
                self.format_me(item_id.as_str());
                self.extend_path(&["content"]);
            }

            let mut fetch_pipeline = FetchPipeline::from(self.pipeline_data());
            let mut path_buf = PathBuf::new();
            path_buf.push(directory);
            fetch_pipeline.directory = path_buf;
            fetch_pipeline.download_url = download_url;
            fetch_pipeline.is_download = true;

            if let Some(name) = value.name() {
                fetch_pipeline.file_name = Some(OsString::from(name));
            }

            Ok(DownloadPipeline {
                pipeline: fetch_pipeline,
                is_direct_download: true,
            })
        } else {
            let item_id = value
                .id()
                .ok_or_else(|| GraphFailure::GraphError(GraphError::default()))?;
            self.format_me(item_id.as_str());
            self.extend_path(&["content"]);

            let mut fetch_pipeline = FetchPipeline::from(self.pipeline_data());
            let mut path_buf = PathBuf::new();
            path_buf.push(directory);
            fetch_pipeline.directory = path_buf;
            if let Some(name) = value.name() {
                fetch_pipeline.file_name = Some(OsString::from(name));
            }

            Ok(DownloadPipeline {
                pipeline: fetch_pipeline,
                is_direct_download: false,
            })
        }
    }

    fn single_thumbnail(
        &mut self,
        item_id: &str,
        thumb_id: &str,
        size: &str,
    ) -> Request<Thumbnail> {
        self.format_me(item_id);
        self.extend_path(&["thumbnails", thumb_id, size]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Thumbnails))
    }

    fn thumbnails(&mut self, item_id: &str) -> Request<ThumbnailCollection> {
        self.format_me(item_id);
        self.extend_path(&["thumbnails"]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Thumbnails))
    }

    fn thumbnails_drive_item(
        &mut self,
        value: drive::driveitem::DriveItem,
    ) -> ItemResult<Request<ThumbnailCollection>> {
        let item_id = value
            .id()
            .ok_or_else(|| GraphFailure::GraphError(GraphError::default()))?;
        Ok(self.thumbnails(item_id.as_str()))
    }

    fn thumbnail_binary(&mut self, item_id: &str, thumb_id: &str, size: &str) -> Request<String> {
        self.format_me(item_id);
        self.extend_path(&["thumbnails", thumb_id, size, "content"]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Thumbnails))
    }

    fn update(&mut self, item_id: &str, new_value: &DriveItem) -> Request<DriveItem> {
        self.format_me(item_id);
        self.body(Body::String(serde_json::to_string_pretty(&new_value).unwrap()));
        self.as_patch();
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Update))
    }

    fn update_drive_item(
        &mut self,
        old_value: &DriveItem,
        new_value: &DriveItem,
    ) -> ItemResult<Request<DriveItem>> {
        let item_id = old_value
            .id()
            .ok_or_else(|| GraphFailure::none_err("item id"))?;
        self.format_me(item_id.as_str());
        self.body(Body::String(
            serde_json::to_string_pretty(&new_value).unwrap(),
        ));
        Ok(Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::Update,
        )))
    }

    fn upload_replace<P: AsRef<Path>>(&mut self, item_id: &str, file: P) -> Request<DriveItem> {
        self.format_me(item_id);
        self.extend_path(&["content"]);
        self.body(Body::File(OsString::from(file.as_ref())));
        self.as_put();
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Upload))
    }

    fn upload_new<P: AsRef<Path>>(
        &mut self,
        parent_id: &str,
        file: P,
    ) -> ItemResult<Request<DriveItem>> {
        let name = file
            .as_ref()
            .file_name()
            .ok_or_else(|| GraphFailure::GraphError(GraphError::default()))?;

        let mut id = String::new();
        id.push_str(parent_id);
        id.push(':');
        let mut os_string = name.to_os_string();
        os_string.push(":");

        self.format_me(id.as_str());
        self.extend_path(&[os_string.to_str().unwrap(), "content"]);

        let mut f = OsString::new();
        f.push(file.as_ref());
        self.body(Body::File(f));
        self.as_put();
        Ok(Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::Upload,
        )))
    }
}

pub trait ItemCommon:
    MutatePipeline + AsMut<DriveUrl> + AsMut<DataPipeline> + Into<SelectEvent>
{
    fn delete(&mut self, item_id: &str, resource_id: &str) -> BoxItemResponse {
        self.format_common(item_id, resource_id);
        self.as_delete();
        BoxItemResponse::new(Box::new(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::Delete,
        )))
    }

    fn delete_drive_item(&mut self, value: DriveItem) -> ItemResult<BoxItemResponse> {
        let (item_id, resource_id) = value.item_event_ids()?;
        self.format_common(item_id.as_str(), resource_id.as_str());
        self.as_delete();
        Ok(BoxItemResponse::new(Box::new(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::Delete,
        ))))
    }

    fn get_item(&mut self, item_id: &str, resource_id: &str) -> Request<DriveItem> {
        self.format_common(item_id, resource_id);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::GetItem))
    }

    fn get_item_path(&mut self, resource_id: &str, path: &str) -> Request<DriveItem> {
        self.extend_path(&[resource_id, "root:", path]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::GetItem))
    }

    fn copy(
        &mut self,
        item_id: &str,
        drive_id: &str,
        item_ref: &ItemReference,
        name: Option<&str>,
    ) -> BoxItemResponse {
        if name.is_some() {
            let item_ref = ItemRefCopy::new(item_ref.clone(), name.map(|s| s.to_string()));
            self.body(Body::String(item_ref.as_json().unwrap()));
        } else {
            let item_ref = ItemRefCopy::new(item_ref.clone(), None);
            self.body(Body::String(item_ref.as_json().unwrap()));
        }

        self.format_common(item_id, drive_id);
        self.extend_path(&["copy"]);
        self.as_post();
        BoxItemResponse::new(Box::new(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::Copy,
        )))
    }

    fn copy_drive_item(
        &mut self,
        drive_item: &DriveItem,
        item_ref: &ItemReference,
        name: Option<&str>,
    ) -> ItemResult<BoxItemResponse> {
        let (item_id, resource_id) = drive_item.item_event_ids()?;
        Ok(self.copy(item_id.as_str(), resource_id.as_str(), item_ref, name))
    }

    fn create_folder(
        &mut self,
        resource_id: &str,
        parent_item_id: &str,
        folder: NewFolder,
    ) -> Request<DriveItem> {
        self.format_common(parent_item_id, resource_id);
        self.extend_path(&["children"]);
        self.body(Body::String(serde_json::to_string_pretty(&folder).unwrap()));
        self.as_post();
        Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::CreateFolder,
        ))
    }

    fn create_folder_by_path(
        &mut self,
        resource_id: &str,
        path_from_root: &str,
        new_folder: NewFolder,
    ) -> Request<DriveItem> {
        let mut s = String::from(path_from_root);
        if !s.ends_with(':') {
            s.push(':');
        }
        self.format_common(s.as_str(), resource_id);
        self.extend_path(&["content"]);
        self.body(Body::String(
            serde_json::to_string_pretty(&new_folder).unwrap(),
        ));
        self.as_post();
        Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::CreateFolder,
        ))
    }

    fn download<P: AsRef<Path>>(
        &mut self,
        directory: P,
        item_id: &str,
        resource_id: &str,
    ) -> DownloadPipeline {
        self.format_common(item_id, resource_id);
        self.extend_path(&["content"]);
        let mut fetch_pipeline = FetchPipeline::from(self.pipeline_data());
        let mut path_buf = PathBuf::new();
        path_buf.push(directory);
        fetch_pipeline.directory = path_buf;

        DownloadPipeline {
            pipeline: fetch_pipeline,
            is_direct_download: false,
        }
    }

    fn download_path<P: AsRef<Path>>(
        &mut self,
        directory: P,
        resource_id: &str,
        path: &[&str],
    ) -> ItemResult<DownloadPipeline> {
        if self.ends_with("drives") {
            self.extend_path(&[resource_id, "items"]);
            self.extend_path(path);
            self.extend_path(&["content"])
        } else {
            self.extend_path(&[resource_id, "drive", "items"]);
            self.extend_path(path);
            self.extend_path(&["content"]);
        }

        let mut fetch_pipeline = FetchPipeline::from(self.pipeline_data());
        let mut path_buf = PathBuf::new();
        path_buf.push(directory);
        fetch_pipeline.directory = path_buf;

        Ok(DownloadPipeline {
            pipeline: fetch_pipeline,
            is_direct_download: false,
        })
    }

    fn thumbnails(&mut self, item_id: &str, resource_id: &str) -> Request<ThumbnailCollection> {
        self.format_common(item_id, resource_id);
        self.extend_path(&["thumbnails"]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Thumbnails))
    }

    fn thumbnails_drive_item(
        &mut self,
        value: drive::driveitem::DriveItem,
    ) -> Request<ThumbnailCollection> {
        let (item_id, resource_id) = value.item_event_ids().unwrap();
        self.thumbnails(item_id.as_str(), resource_id.as_str())
    }

    fn single_thumbnail(
        &mut self,
        item_id: &str,
        resource_id: &str,
        thumb_id: &str,
        size: &str,
    ) -> Request<Thumbnail> {
        self.format_common(item_id, resource_id);
        self.extend_path(&["thumbnails", thumb_id, size]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Thumbnails))
    }

    fn thumbnail_binary(
        &mut self,
        item_id: &str,
        resource_id: &str,
        thumb_id: &str,
        size: &str,
    ) -> Request<String> {
        self.format_common(item_id, resource_id);
        self.extend_path(&["thumbnails", thumb_id, size, "content"]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Thumbnails))
    }

    fn update(
        &mut self,
        item_id: &str,
        resource_id: &str,
        new_value: &DriveItem,
    ) -> Request<DriveItem> {
        self.format_common(item_id, resource_id);
        self.body(Body::String(
            serde_json::to_string_pretty(&new_value).unwrap(),
        ));
        self.as_patch();
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Update))
    }

    fn update_drive_item(
        &mut self,
        old_value: &DriveItem,
        new_value: &DriveItem,
    ) -> ItemResult<Request<DriveItem>> {
        let (item_id, resource_id) = old_value.item_event_ids()?;
        self.format_common(item_id.as_str(), resource_id.as_str());
        self.body(Body::String(
            serde_json::to_string_pretty(&new_value).unwrap(),
        ));
        Ok(Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::Update,
        )))
    }

    fn upload_replace<P: AsRef<Path>>(
        &mut self,
        item_id: &str,
        resource_id: &str,
        file: P,
    ) -> Request<DriveItem> {
        self.format_common(item_id, resource_id);
        self.extend_path(&["content"]);
        self.body(Body::File(OsString::from(file.as_ref())));
        self.as_put();
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Upload))
    }

    fn upload_new<P: AsRef<Path>>(
        &mut self,
        parent_id: &str,
        resource_id: &str,
        file: P,
    ) -> ItemResult<Request<DriveItem>> {
        let name = file
            .as_ref()
            .file_name()
            .ok_or_else(|| GraphFailure::GraphError(GraphError::default()))?;

        let mut id = String::new();
        id.push_str(parent_id);
        id.push(':');
        let mut os_string = name.to_os_string();
        os_string.push(":");

        self.format_common(id.as_str(), resource_id);
        self.extend_path(&[os_string.to_str().unwrap(), "content"]);

        let mut f = OsString::new();
        f.push(file.as_ref());
        self.body(Body::File(f));
        self.as_put();
        Ok(Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::Upload,
        )))
    }
}

impl AsMut<DriveUrl> for SelectResource {
    fn as_mut(&mut self) -> &mut DriveUrl {
        self.0.as_mut()
    }
}

impl From<DataPipeline> for SelectResource {
    fn from(data: DataPipeline) -> Self {
        SelectResource(data)
    }
}

mod inner_pipeline {
    use crate::drive::driveurl::DriveUrl;
    use crate::drive::event::DriveEvent;
    use crate::drive::item::{SelectResource, SelectEvent};
    use crate::drive::pipeline::{Body, DataPipeline, RequestType};
    use crate::drive::{DriveEndPoint, SelectEventMe};

    pub trait MutatePipeline {
        fn pipeline_data(&self) -> DataPipeline;

        fn as_get(&mut self)
        where
            Self: AsMut<DataPipeline>,
        {
            self.as_mut().as_get();
        }

        fn as_post(&mut self)
        where
            Self: AsMut<DataPipeline>,
        {
            self.as_mut().as_post();
        }

        fn as_put(&mut self)
        where
            Self: AsMut<DataPipeline>,
        {
            self.as_mut().as_put();
        }

        fn as_patch(&mut self)
        where
            Self: AsMut<DataPipeline>,
        {
            self.as_mut().as_patch();
        }

        fn as_delete(&mut self)
        where
            Self: AsMut<DataPipeline>,
        {
            self.as_mut().as_delete();
        }

        fn request_type(&mut self, r: RequestType)
        where
            Self: AsMut<DataPipeline>,
        {
            self.as_mut().request_type = r;
        }

        fn body(&mut self, body: Body)
        where
            Self: AsMut<DataPipeline>,
        {
            self.as_mut().body = Some(body);
        }

        fn extend_path(&mut self, path: &[&str])
        where
            Self: AsMut<DriveUrl>,
        {
            self.as_mut().extend_path(path);
        }

        fn endpoint(&mut self, endpoint: DriveEndPoint)
        where
            Self: AsMut<DriveUrl>,
        {
            self.as_mut().endpoint(endpoint);
        }

        fn event(&mut self, event: DriveEvent)
        where
            Self: AsMut<DriveUrl>,
        {
            self.as_mut().event(event);
        }

        fn strip_set_endpoint(&mut self, endpoint: DriveEndPoint)
        where
            Self: AsMut<DriveUrl>,
        {
            let mut vec: Vec<&str> = endpoint.as_str().split('/').collect();
            vec.retain(|s| !s.trim().is_empty());
            vec.remove(0);
            self.extend_path(&vec);
        }

        fn format_me(&mut self, item_id: &str)
        where
            Self: AsMut<DriveUrl>,
        {
            self.as_mut().extend_path(&["drive", "items", item_id]);
        }

        fn format_drives(&mut self, item_id: &str, resource_id: &str)
        where
            Self: AsMut<DriveUrl>,
        {
            self.as_mut().extend_path(&[resource_id, "items", item_id]);
        }

        fn format_common(&mut self, item_id: &str, resource_id: &str)
        where
            Self: AsMut<DriveUrl>,
        {
            if self.as_mut().ends_with("drives") {
                self.format_drives(item_id, resource_id);
            } else {
                self.as_mut()
                    .extend_path(&[resource_id, "drive", "items", item_id]);
            }
        }

        fn ends_with(&mut self, s: &str) -> bool
        where
            Self: AsMut<DriveUrl>,
        {
            self.as_mut().ends_with(s)
        }
    }

    impl MutatePipeline for SelectEventMe {
        fn pipeline_data(&self) -> DataPipeline {
            self.0.clone()
        }
    }

    impl MutatePipeline for SelectResource {
        fn pipeline_data(&self) -> DataPipeline {
            self.0.clone()
        }
    }

    impl MutatePipeline for SelectEvent {
        fn pipeline_data(&self) -> DataPipeline {
            self.0.clone()
        }
    }
}
