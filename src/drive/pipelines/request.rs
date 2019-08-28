use crate::drive::event::DriveEvent;
use crate::drive::item::IntoItem;
use crate::drive::pipelines::datapipeline::{Body, DataPipeline};
use crate::drive::pipelines::downloadpipeline::DownloadPipeline;
use crate::drive::pipelines::pipeline::Pipeline;
use crate::drive::pipelines::request::format_request_sealed::FormatRequest;
use crate::drive::ItemResult;
use crate::prelude::*;
use graph_error::{GraphError, GraphFailure};
use reqwest::{Client, RequestBuilder};
use serde_json::Value;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Clone, Eq, PartialEq)]
pub enum Formatting {
    ActivitiesLists(String),
    RestoreVersions(String),
    SingleThumbnail(String, String),
    ThumbnailBinary(String, String),
    UploadNew(OsString),
}

pub(crate) mod format_request_sealed {
    use crate::drive::driveurl::{DriveUrl, MutateUrl};
    use std::ffi::OsString;

    pub trait FormatRequest: MutateUrl + AsMut<DriveUrl> {
        fn format_me(&mut self, item_id: &str) {
            self.as_mut().extend_path(&["drive", "items", item_id]);
        }

        fn format_drives(&mut self, item_id: &str, resource_id: &str) {
            self.as_mut().extend_path(&[resource_id, "items", item_id]);
        }

        fn format_common(&mut self, item_id: &str, resource_id: &str) {
            if self.as_mut().ends_with("drives") {
                self.format_drives(item_id, resource_id);
            } else {
                self.as_mut()
                    .extend_path(&[resource_id, "drive", "items", item_id]);
            }
        }

        fn format_path_me(&mut self, path: OsString, end_colon: bool) {
            if !path.is_empty() {
                self.as_mut().extend_path(&["drive", "root:"]);
                if end_colon {
                    let mut s = path.clone();
                    s.push(":");
                    self.as_mut().extend_path_os_string(&[s]);
                } else {
                    self.as_mut().extend_path_os_string(&[path]);
                }
            }
        }

        fn format_path_drives(&mut self, resource_id: &str, path: OsString, end_colon: bool) {
            if !path.is_empty() {
                self.as_mut().extend_path(&[resource_id, "root:"]);
                if end_colon {
                    let mut s = path.clone();
                    s.push(":");
                    self.as_mut().extend_path_os_string(&[s]);
                } else {
                    self.as_mut().extend_path_os_string(&[path]);
                }
            }
        }

        fn format_path_common(&mut self, resource_id: &str, path: OsString, end_colon: bool) {
            if !path.is_empty() {
                if self.as_mut().ends_with("drives") {
                    self.format_path_drives(resource_id, path, end_colon);
                } else {
                    self.as_mut().extend_path(&[resource_id, "drive", "root:"]);
                    if end_colon {
                        let mut s = path.clone();
                        s.push(":");
                        self.as_mut().extend_path_os_string(&[s]);
                    } else {
                        self.as_mut().extend_path_os_string(&[path]);
                    }
                }
            }
        }

        fn activities_list_item(&mut self, item_id: &str, resource_id: &str, list_id: &str) {
            self.extend_path(&[
                resource_id,
                "lists",
                list_id,
                "items",
                item_id,
                "activities",
            ]);
        }

        fn activities_list_item_path(&mut self, resource_id: &str, list_id: &str, path: OsString) {
            let mut s = path.clone();
            s.push(":");
            self.extend_path(&[resource_id, "lists", list_id, "root:"]);
            self.extend_path_os_string(&[path]);
            self.extend_path(&["activities"]);
        }

        fn activities_list_item_me(&mut self, item_id: &str, list_id: &str) {
            self.extend_path(&["drive", "lists", list_id, "items", item_id, "activities"]);
        }

        fn activities_list_item_me_path(&mut self, list_id: &str, path: OsString) {
            let mut s = path.clone();
            s.push(":");
            self.extend_path(&["drive", "lists", list_id, "root:"]);
            self.extend_path_os_string(&[path]);
            self.extend_path(&["activities"]);
        }

        fn restore_version(&mut self, item_id: &str, resource_id: &str, version_id: &str) {
            if self.ends_with("drives") {
                self.extend_path(&[
                    resource_id,
                    "items",
                    item_id,
                    "versions",
                    version_id,
                    "restoreVersion",
                ]);
            } else {
                self.extend_path(&[
                    resource_id,
                    "drive",
                    "items",
                    item_id,
                    "versions",
                    version_id,
                    "restoreVersion",
                ]);
            }
        }

        fn restore_version_path(&mut self, resource_id: &str, version_id: &str, path: OsString) {
            let mut s = path.clone();
            s.push(":");
            if self.ends_with("drives") {
                self.extend_path(&[resource_id, "root:"]);
            } else {
                self.extend_path(&[resource_id, "drive", "root:"]);
            }
            self.extend_path_os_string(&[s]);
            self.extend_path(&["versions", version_id, "restoreVersion"]);
        }

        fn restore_version_me(&mut self, item_id: &str, version_id: &str) {
            self.extend_path(&[
                "drive",
                "items",
                item_id,
                "versions",
                version_id,
                "restoreVersion",
            ]);
        }

        fn restore_version_me_path(&mut self, version_id: &str, path: OsString) {
            let mut s = path.clone();
            s.push(":");
            self.extend_path(&["drive", "root:"]);
            self.extend_path_os_string(&[s]);
            self.extend_path(&["versions", version_id, "restoreVersion"]);
        }

        fn single_thumbnail(
            &mut self,
            item_id: &str,
            resource_id: &str,
            thumb_id: &str,
            size: &str,
        ) {
            if self.ends_with("drives") {
                self.extend_path(&[resource_id, "items", item_id, "thumbnails", thumb_id, size]);
            } else {
                self.extend_path(&[
                    resource_id,
                    "drive",
                    "items",
                    item_id,
                    "thumbnails",
                    thumb_id,
                    size,
                ]);
            }
        }

        fn single_thumbnail_path(
            &mut self,
            resource_id: &str,
            thumb_id: &str,
            size: &str,
            path: OsString,
        ) {
            let mut s = path.clone();
            s.push(":");
            self.extend_path(&[resource_id, "root:"]);
            self.extend_path_os_string(&[s]);
            self.extend_path(&["thumbnails", thumb_id, size]);
        }

        fn single_thumbnail_me(&mut self, item_id: &str, thumb_id: &str, size: &str) {
            self.extend_path(&["drive", "items", item_id, "thumbnails", thumb_id, size]);
        }

        fn single_thumbnail_me_path(&mut self, thumb_id: &str, size: &str, path: OsString) {
            let mut s = path.clone();
            s.push(":");
            self.extend_path(&["drive", "root:"]);
            self.extend_path_os_string(&[s]);
            self.extend_path(&["thumbnails", thumb_id, size]);
        }

        fn thumbnail_binary(
            &mut self,
            item_id: &str,
            resource_id: &str,
            thumb_id: &str,
            size: &str,
        ) {
            if self.ends_with("drives") {
                self.extend_path(&[
                    resource_id,
                    "items",
                    item_id,
                    "thumbnails",
                    thumb_id,
                    size,
                    "content",
                ]);
            } else {
                self.extend_path(&[
                    resource_id,
                    "drive",
                    "items",
                    item_id,
                    "thumbnails",
                    thumb_id,
                    size,
                    "content",
                ]);
            }
        }

        fn thumbnail_binary_path(
            &mut self,
            resource_id: &str,
            thumb_id: &str,
            size: &str,
            path: OsString,
        ) {
            let mut s = path.clone();
            s.push(":");
            self.extend_path(&[resource_id, "root:"]);
            self.extend_path_os_string(&[s]);
            self.extend_path(&["thumbnails", thumb_id, size, "content"]);
        }

        fn thumbnail_binary_me(&mut self, item_id: &str, thumb_id: &str, size: &str) {
            self.extend_path(&[
                "drive",
                "items",
                item_id,
                "thumbnails",
                thumb_id,
                size,
                "content",
            ]);
        }

        fn thumbnail_binary_me_path(&mut self, thumb_id: &str, size: &str, path: OsString) {
            let mut s = path.clone();
            s.push(":");
            self.extend_path(&["drive", "root:"]);
            self.extend_path_os_string(&[s]);
            self.extend_path(&[thumb_id, size, "content"]);
        }
    }

}

pub struct BoxedDownload {
    request: DownloadPipeline,
}

impl BoxedDownload {
    pub fn by_id(mut self, item_id: &str) -> DownloadPipeline {
        self.format_me(item_id);
        self.event(DriveEvent::Download);
        self.request
    }

    pub fn by_path(mut self, path: OsString) -> DownloadPipeline {
        let path_buf: PathBuf = PathBuf::from(path.to_os_string());
        if let Some(file_name) = path_buf.file_name() {
            self.request.pipeline.file_name = Some(file_name.to_os_string());
        }
        self.format_path_me(path, true);
        self.event(DriveEvent::Download);
        self.request
    }

    pub fn by_drive_item(mut self, value: &DriveItem) -> ItemResult<DownloadPipeline> {
        if let Some(download_url) = value.microsoft_graph_download_url() {
            // Build the non-download url in the case that a format has been
            // selected after this. The format download always uses the redirect.
            if let Some(item_id) = value.id() {
                self.format_me(item_id.as_str());
                self.extend_path(&["content"]);
            }

            self.request.pipeline.download_url = download_url.to_string();
            self.request.pipeline.is_download = true;
            self.request.is_direct_download = true;

            if let Some(name) = value.name() {
                self.request.pipeline.file_name = Some(OsString::from(name));
            }

            Ok(self.request)
        } else {
            let item_id = value
                .id()
                .as_ref()
                .ok_or_else(|| GraphFailure::GraphError(Box::new(GraphError::default())))?;
            self.format_me(item_id.as_str());
            self.extend_path(&["content"]);

            if let Some(name) = value.name() {
                self.request.pipeline.file_name = Some(OsString::from(name));
            }

            Ok(self.request)
        }
    }
}

impl From<DownloadPipeline> for BoxedDownload {
    fn from(dp: DownloadPipeline) -> Self {
        BoxedDownload { request: dp }
    }
}

impl FormatRequest for BoxedDownload {}
impl MutateUrl for BoxedDownload {}

impl AsRef<DriveUrl> for BoxedDownload {
    fn as_ref(&self) -> &DriveUrl {
        self.request.as_ref()
    }
}

impl AsMut<DriveUrl> for BoxedDownload {
    fn as_mut(&mut self) -> &mut DriveUrl {
        self.request.as_mut()
    }
}

pub struct BoxedDownloadCommon {
    request: DownloadPipeline,
}

impl BoxedDownloadCommon {
    pub fn by_id(mut self, item_id: &str, resource_id: &str) -> DownloadPipeline {
        self.format_common(item_id, resource_id);
        self.event(DriveEvent::Download);
        self.request
    }

    pub fn by_path(mut self, resource_id: &str, path: OsString) -> DownloadPipeline {
        self.format_path_common(resource_id, path, true);
        self.event(DriveEvent::Download);
        self.request
    }

    pub fn by_drive_item(mut self, drive_item: &DriveItem) -> ItemResult<DownloadPipeline> {
        let (item_id, drive_id) = drive_item.item_event_ids()?;
        self.format_common(item_id.as_str(), drive_id.as_str());
        self.event(DriveEvent::Download);
        Ok(self.request)
    }
}

impl From<DownloadPipeline> for BoxedDownloadCommon {
    fn from(dp: DownloadPipeline) -> Self {
        BoxedDownloadCommon { request: dp }
    }
}

impl FormatRequest for BoxedDownloadCommon {}
impl MutateUrl for BoxedDownloadCommon {}

impl AsRef<DriveUrl> for BoxedDownloadCommon {
    fn as_ref(&self) -> &DriveUrl {
        self.request.as_ref()
    }
}

impl AsMut<DriveUrl> for BoxedDownloadCommon {
    fn as_mut(&mut self) -> &mut DriveUrl {
        self.request.as_mut()
    }
}

pub struct BoxedRequest<T> {
    request: Request<T>,
    pub formatting: Option<Formatting>,
}

impl<T> BoxedRequest<T> {
    pub fn by_id(mut self, item_id: &str) -> Request<T> {
        let formatting = self.formatting.clone();
        if let Some(format) = formatting {
            match format {
                Formatting::ActivitiesLists(s) => {
                    self.activities_list_item_me(item_id, s.clone().as_str());
                },
                Formatting::RestoreVersions(s) => {
                    self.restore_version_me(item_id, s.clone().as_str());
                },
                Formatting::SingleThumbnail(thumb_id, size) => {
                    self.single_thumbnail_me(item_id, thumb_id.as_str(), size.as_str());
                },
                Formatting::ThumbnailBinary(thumb_id, size) => {
                    self.thumbnail_binary_me(item_id, thumb_id.as_str(), size.as_str());
                },
                Formatting::UploadNew(s) => {
                    let id = format!("{}:", item_id);
                    self.request.extend_path(&["drive", "items", &id]);
                    let mut s = s.to_os_string();
                    s.push(":");
                    self.request.extend_path_os_string(&[s]);
                    self.request.extend_path(&["content"]);
                }
            }
        } else {
            self.format_me(item_id);
            let event = self.drive_event();
            if !event.as_str().is_empty() {
                self.event(event);
            }
        }
        self.request
    }

    pub fn by_path(mut self, path: OsString) -> Request<T> {
        let formatting = self.formatting.clone();
        if let Some(format) = formatting {
            match format {
                Formatting::ActivitiesLists(s) => {
                    self.activities_list_item_me_path(s.clone().as_str(), path);
                },
                Formatting::RestoreVersions(s) => {
                    self.restore_version_me_path(s.clone().as_str(), path);
                },
                Formatting::SingleThumbnail(thumb_id, size) => {
                    self.single_thumbnail_me_path(thumb_id.as_str(), size.as_str(), path);
                },
                Formatting::ThumbnailBinary(thumb_id, size) => {
                    self.thumbnail_binary_me_path(thumb_id.as_str(), size.as_str(), path);
                },
                Formatting::UploadNew(_) => {
                    self.request.extend_path(&["drive", "root:"]);
                    let mut s = path.to_os_string();
                    s.push(":");
                    self.request.extend_path_os_string(&[s]);
                    self.request.extend_path(&["content"]);
                }
            }
        } else {
            let event = self.request.drive_event();
            let empty = event.as_str().is_empty();
            self.request.format_path_me(path, !empty);
            if !empty {
                self.request.event(event);
            }
        }

        self.request
    }
}

impl<T> From<Request<T>> for BoxedRequest<T> {
    fn from(request: Request<T>) -> Self {
        BoxedRequest {
            request,
            formatting: None,
        }
    }
}

impl<T> AsRef<DriveUrl> for BoxedRequest<T> {
    fn as_ref(&self) -> &DriveUrl {
        self.request.as_ref()
    }
}

impl<T> AsMut<DriveUrl> for BoxedRequest<T> {
    fn as_mut(&mut self) -> &mut DriveUrl {
        self.request.as_mut()
    }
}

impl<T> FormatRequest for BoxedRequest<T> {}

impl<T> MutateUrl for BoxedRequest<T> {}

pub struct BoxedRequestCommon<T> {
    request: Request<T>,
    pub formatting: Option<Formatting>,
}

impl<T> BoxedRequestCommon<T> {
    pub fn by_id(mut self, item_id: &str, resource_id: &str) -> Request<T> {
        let formatting = self.formatting.clone();
        if let Some(format) = formatting {
            match format {
                Formatting::ActivitiesLists(s) => {
                    self.activities_list_item(item_id, resource_id, s.clone().as_str());
                },
                Formatting::RestoreVersions(s) => {
                    self.restore_version(item_id, resource_id, s.clone().as_str());
                },
                Formatting::SingleThumbnail(thumb_id, size) => {
                    self.single_thumbnail(item_id, resource_id, thumb_id.as_str(), size.as_str());
                },
                Formatting::ThumbnailBinary(thumb_id, size) => {
                    self.thumbnail_binary(item_id, resource_id, thumb_id.as_str(), size.as_str());
                },
                Formatting::UploadNew(s) => {
                    let parent_id = format!("{}:", item_id);
                    if self.ends_with("drives") {
                        self.request.extend_path(&[resource_id, "items", &parent_id]);
                    } else {
                        self.request.extend_path(&[resource_id, "drive", "items", &parent_id]);
                    }
                    let mut s = s.to_os_string();
                    s.push(":");
                    self.request.extend_path_os_string(&[s]);
                    self.request.extend_path(&["content"]);
                }
            }
        } else {
            self.format_common(item_id, resource_id);
            let event = self.drive_event();
            let event = event.as_str();
            if !event.is_empty() {
                self.extend_path(&[event]);
            }
        }
        self.request
    }

    pub fn by_path(mut self, resource_id: &str, path: OsString) -> Request<T> {
        let formatting = self.formatting.clone();
        if let Some(format) = formatting {
            match format {
                Formatting::ActivitiesLists(s) => {
                    self.activities_list_item_path(resource_id, s.clone().as_str(), path)
                },
                Formatting::RestoreVersions(s) => {
                    self.restore_version_path(resource_id, s.clone().as_str(), path)
                },
                Formatting::SingleThumbnail(thumb_id, size) => {
                    self.single_thumbnail_path(resource_id, thumb_id.as_str(), size.as_str(), path);
                },
                Formatting::ThumbnailBinary(thumb_id, size) => {
                    self.thumbnail_binary_path(resource_id, thumb_id.as_str(), size.as_str(), path);
                },
                Formatting::UploadNew(_) => {
                    if self.ends_with("drives") {
                        self.request.extend_path(&[resource_id, "root:"]);
                    } else {
                        self.request.extend_path(&[resource_id, "drive", "root:"]);
                    }
                    let mut s = path;
                    s.push(":");
                    self.request.extend_path_os_string(&[s]);
                    self.request.extend_path(&["content"]);
                }
            }
        } else {
            let event = self.request.drive_event();
            if event.as_str().is_empty() {
                self.request.format_path_common(resource_id, path, false);
            } else {
                self.request.format_path_common(resource_id, path, true);
            }
            if !event.as_str().is_empty() {
                self.request.event(event);
            }
        }
        self.request
    }
}

impl<T> From<Request<T>> for BoxedRequestCommon<T> {
    fn from(request: Request<T>) -> Self {
        BoxedRequestCommon {
            request,
            formatting: None,
        }
    }
}

impl<T> AsRef<DriveUrl> for BoxedRequestCommon<T> {
    fn as_ref(&self) -> &DriveUrl {
        self.request.as_ref()
    }
}

impl<T> AsMut<DriveUrl> for BoxedRequestCommon<T> {
    fn as_mut(&mut self) -> &mut DriveUrl {
        self.request.as_mut()
    }
}

impl<T> FormatRequest for BoxedRequestCommon<T> {}
impl<T> MutateUrl for BoxedRequestCommon<T> {}

pub trait ByItem<T>: FormatRequest + From<Request<T>> {
    fn drive_event(&mut self) -> DriveEvent;
    fn custom_format(&mut self, format: Formatting);
}

impl<T> ByItem<T> for BoxedRequest<T> {
    fn drive_event(&mut self) -> DriveEvent {
        self.request.drive_event()
    }

    fn custom_format(&mut self, format: Formatting) {
        self.formatting = Some(format);
    }
}

impl<T> ByItem<T> for BoxedRequestCommon<T> {
    fn drive_event(&mut self) -> DriveEvent {
        self.request.drive_event()
    }

    fn custom_format(&mut self, format: Formatting) {
        self.formatting = Some(format);
    }
}

pub trait PipelineRequest<T, U> {
    fn send(&self, pipeline: U) -> ItemResult<T>;
}

impl<T, U, F> PipelineRequest<T, U> for F
where
    F: Fn(U) -> ItemResult<T>,
{
    fn send(&self, pipeline: U) -> ItemResult<T> {
        self(pipeline)
    }
}

pub fn pipeline_request<T>() -> impl PipelineRequest<T, DataPipeline>
where
    for<'de> T: serde::Deserialize<'de>,
{
    move |data: DataPipeline| {
        let builder: RequestBuilder = data.request_builder()?;
        let mut response = builder.send()?;

        if let Some(err) = GraphFailure::err_from(&mut response) {
            Err(err)
        } else {
            let value: T = response.json()?;
            Ok(value)
        }
    }
}

pub struct Request<T> {
    item: Box<dyn IntoItem<T>>,
}

impl<T> Request<T> {
    pub fn new(item: Box<dyn IntoItem<T>>) -> Request<T> {
        Request { item }
    }

    pub fn send(&mut self) -> ItemResult<T> {
        self.item.send()
    }

    pub fn client() -> ItemResult<Client> {
        reqwest::Client::builder()
            .build()
            .map_err(GraphFailure::from)
    }

    fn drive_event(&mut self) -> DriveEvent {
        self.item.drive_event()
    }
}

impl<T> MutateUrl for Request<T> {}

impl<T> FormatRequest for Request<T> {}

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

impl<T> AsRef<DriveEvent> for Request<T> {
    fn as_ref(&self) -> &DriveEvent {
        self.item.as_ref().as_ref()
    }
}

impl<T> From<DownloadPipeline> for Request<T>
where
    DownloadPipeline: IntoItem<T>,
{
    fn from(dp: DownloadPipeline) -> Self {
        Request::new(Box::new(dp))
    }
}

pub struct ReqBuilder {
    pipeline: DataPipeline,
}

impl ReqBuilder {
    pub fn new(pipeline: DataPipeline) -> ReqBuilder {
        ReqBuilder { pipeline }
    }

    pub fn body(&mut self, body: Body) -> &mut Self {
        self.pipeline.body = Some(body);
        self
    }

    pub fn content_type(&mut self, s: &str) -> &mut Self {
        self.pipeline.content_type = s.to_string();
        self
    }

    pub fn send(&mut self) -> ItemResult<Value> {
        pipeline_request().send(self.pipeline.clone())
    }
}

impl AsRef<DriveUrl> for ReqBuilder {
    fn as_ref(&self) -> &DriveUrl {
        &self.pipeline.url
    }
}

impl AsMut<DriveUrl> for ReqBuilder {
    fn as_mut(&mut self) -> &mut DriveUrl {
        &mut self.pipeline.url
    }
}

impl<T> From<&ReqBuilder> for Request<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    fn from(dr: &ReqBuilder) -> Self {
        Request::from(Pipeline::new(dr.pipeline.clone(), DriveEvent::Custom))
    }
}

impl MutateUrl for ReqBuilder {}
