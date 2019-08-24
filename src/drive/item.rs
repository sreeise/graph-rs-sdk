use crate::drive::drive_item::collection::Collection;
use crate::drive::drive_item::driveitem::DriveItem;
use crate::drive::drive_item::driveitemversion::DriveItemVersion;
use crate::drive::drive_item::itemactivity::ItemActivity;
use crate::drive::drive_item::itemreference::ItemReference;
use crate::drive::drive_item::thumbnail::{Thumbnail, ThumbnailSet};
use crate::drive::driveurl::MutateUrl;
use crate::drive::event::{CreateUploadSession, ItemRefCopy, UploadSessionJson};
use crate::drive::event::{DriveEvent, NewFolder};
use crate::drive::item::item_sealed::MutateRequest;
use crate::drive::pipelines::datapipeline::Body;
use crate::drive::pipelines::datapipeline::DataPipeline;
use crate::drive::pipelines::downloadpipeline::DownloadPipeline;
use crate::drive::pipelines::downloadpipeline::FetchPipeline;
use crate::drive::pipelines::pipeline::Pipeline;
use crate::drive::pipelines::request::{ReqBuilder, Request};
use crate::drive::pipelines::uploadsessionpipeline::UploadSessionPipeline;
use crate::drive::statusresponse::StatusResponse;
use crate::drive::ItemResult;
use crate::prelude::DriveUrl;
use graph_error::{GraphError, GraphFailure};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use crate::drive::drive_item::driveinfo::DriveInfo;

pub trait IntoItem<T>: MutateUrl {
    fn send(&mut self) -> ItemResult<T>;
}

pub struct SelectEventMe(DataPipeline);

impl ItemMe for SelectEventMe {}

impl From<SelectResource> for SelectEventMe {
    fn from(resource: SelectResource) -> Self {
        SelectEventMe(resource.0)
    }
}

pub struct SelectEvent(DataPipeline);

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

pub trait ItemMe: MutateRequest + AsMut<DriveUrl> + AsMut<DataPipeline> {
    /// Retrieve the metadata for a DriveItem in a Drive by file system path or ID.
    ///
    /// # See
    /// [Delete a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_delete?view=odsp-graph-online)
    fn delete(&mut self, item_id: &str) -> Request<StatusResponse> {
        self.format_me(item_id);
        self.as_delete();
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Delete))
    }

    fn delete_drive_item(&mut self, value: &DriveItem) -> ItemResult<Request<StatusResponse>> {
        let item_id = value
            .id()
            .as_ref()
            .ok_or_else(|| GraphFailure::none_err("item_id"))?;
        Ok(self.delete(item_id.as_str()))
    }

    /// Retrieve the metadata for a DriveItem in a Drive by file system path or ID.
    ///
    /// # See
    /// [Get a DriveItem resource](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_get?view=odsp-graph-online)
    fn get_item(&mut self, item_id: &str) -> Request<DriveItem> {
        self.format_me(item_id);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::GetItem))
    }

    fn get_item_path(&mut self, path: &str) -> Request<DriveItem> {
        self.extend_path(&["drive", "root:", path]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::GetItem))
    }

    /// Create a new folder or DriveItem in a Drive with a specified id.
    ///
    /// # See
    /// [Create a new folder in a drive](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_copy?view=odsp-graph-online)
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

    /// Create a new folder or DriveItem in a Drive with a specified  path.
    ///
    /// # See
    /// [Create a new folder in a drive](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_copy?view=odsp-graph-online)
    fn create_folder_path(
        &mut self,
        path_from_root: &str,
        new_folder: NewFolder,
    ) -> Request<DriveItem> {
        if path_from_root.is_empty() {
            self.extend_path(&["drive", "root", "children"])
        } else if !path_from_root.ends_with(':') {
            self.extend_path(&[
                "drive",
                "root:",
                format!("{}{}", path_from_root, ":").as_str(),
                "children",
            ]);
        } else {
            self.extend_path(&["drive", "root:", path_from_root, "children"]);
        }

        self.body(Body::String(
            serde_json::to_string_pretty(&new_folder).unwrap(),
        ));
        self.as_post();
        Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::CreateFolder,
        ))
    }

    /// Asynchronously creates a copy of an driveItem (including any children),
    /// under a new parent item or with a new name.
    ///
    /// # See
    /// [Copy a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_copy?view=odsp-graph-online)
    fn copy(
        &mut self,
        item_id: &str,
        item_ref: &ItemReference,
        name: Option<&str>,
    ) -> Request<StatusResponse> {
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
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Copy))
    }

    fn copy_drive_item(
        &mut self,
        drive_item: &DriveItem,
        name: Option<&str>,
    ) -> ItemResult<Request<StatusResponse>> {
        let item_id = drive_item
            .id()
            .as_ref()
            .ok_or_else(|| GraphFailure::none_err("item_id"))?;
        let item_ref = drive_item
            .parent_reference()
            .as_ref()
            .ok_or_else(|| GraphFailure::none_err("parent_reference"))?;
        Ok(self.copy(item_id.as_str(), &item_ref, name))
    }

    /// Download the contents of the primary stream (file) of a DriveItem
    ///
    /// # See
    /// [Download the contents of a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_get_content?view=odsp-graph-online)
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
            fetch_pipeline.download_url = download_url.to_string();
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
                .as_ref()
                .ok_or_else(|| GraphFailure::GraphError(Box::new(GraphError::default())))?;
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

    /// Retrieve a single thumbnail for a DriveItem
    ///
    /// # See
    /// [List thumbnails for a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_list_thumbnails?view=odsp-graph-online)
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

    /// Retrieve a collection of ThumbnailSet resources for a DriveItem resource.
    ///
    /// # See
    /// [List thumbnails for a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_list_thumbnails?view=odsp-graph-online)
    fn thumbnails(&mut self, item_id: &str) -> Request<Collection<ThumbnailSet>> {
        self.format_me(item_id);
        self.extend_path(&["thumbnails"]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Thumbnails))
    }

    fn thumbnails_drive_item(
        &mut self,
        value: DriveItem,
    ) -> ItemResult<Request<Collection<ThumbnailSet>>> {
        let item_id = value
            .id()
            .as_ref()
            .ok_or_else(|| GraphFailure::GraphError(Box::new(GraphError::default())))?;
        Ok(self.thumbnails(item_id.as_str()))
    }

    fn thumbnail_binary(&mut self, item_id: &str, thumb_id: &str, size: &str) -> Request<Vec<u8>> {
        self.format_me(item_id);
        self.extend_path(&["thumbnails", thumb_id, size, "content"]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Thumbnails))
    }

    /// Update the metadata for a DriveItem by ID.
    ///
    /// # See
    /// [Update DriveItem properties](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_put_content?view=odsp-graph-online)
    fn update(&mut self, item_id: &str, new_value: &DriveItem) -> Request<DriveItem> {
        self.format_me(item_id);
        self.body(Body::String(
            serde_json::to_string_pretty(&new_value).unwrap(),
        ));
        self.as_patch();
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Update))
    }

    /// Update the metadata for a DriveItem by using a previous DriveItem
    ///
    /// # See
    /// [Update DriveItem properties](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_put_content?view=odsp-graph-online)
    fn update_drive_item(
        &mut self,
        old_value: &DriveItem,
        new_value: &DriveItem,
    ) -> ItemResult<Request<DriveItem>> {
        let item_id = old_value
            .id()
            .as_ref()
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

    /// The simple upload API allows you to provide the contents of a new
    /// file or update the contents of an existing file in a single API call.
    /// This method only supports files up to 4MB in size.
    ///
    /// # See
    /// [Upload or replace the contents of a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_put_content?view=odsp-graph-online)
    fn upload_replace<P: AsRef<Path>>(&mut self, item_id: &str, file: P) -> Request<DriveItem> {
        self.format_me(item_id);
        self.extend_path(&["content"]);
        self.body(Body::File(OsString::from(file.as_ref())));
        self.as_put();
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Upload))
    }

    /// Crate a new file or folder. The simple upload API allows you to provide
    /// the contents of a new file or update the contents of an existing file in
    /// a single API call. This method only supports files up to 4MB in size.
    ///
    /// # See
    /// [Upload or replace the contents of a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_put_content?view=odsp-graph-online)
    fn upload_new<P: AsRef<Path>>(
        &mut self,
        parent_id: &str,
        file: P,
    ) -> ItemResult<Request<DriveItem>> {
        let name = file
            .as_ref()
            .file_name()
            .ok_or_else(|| GraphFailure::GraphError(Box::new(GraphError::default())))?;

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

    /// List versions for a DriveItem. OneDrive and SharePoint can be configured
    /// to retain the history for files. Depending on the service and configuration,
    /// a new version can be created for each edit, each time the file is saved, manually,
    ///
    /// # See
    /// [Listing versions of a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_list_versions?view=odsp-graph-online)
    fn list_versions(&mut self, item_id: &str) -> Request<Collection<DriveItemVersion>> {
        self.format_me(item_id);
        self.extend_path(&["versions"]);
        Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::ListVersions,
        ))
    }

    /// List versions for a DriveItem. OneDrive and SharePoint can be configured
    /// to retain the history for files. Depending on the service and configuration,
    /// a new version can be created for each edit, each time the file is saved, manually,
    ///
    /// # See
    /// [Listing versions of a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_list_versions?view=odsp-graph-online)
    fn list_versions_drive_item(
        &mut self,
        drive_item: &DriveItem,
    ) -> ItemResult<Request<Collection<DriveItemVersion>>> {
        let item_id = drive_item
            .id()
            .as_ref()
            .ok_or_else(|| GraphFailure::none_err("item_id"))?;
        Ok(self.list_versions(item_id.as_str()))
    }

    /// Restore a previous version of a DriveItem to be the current version.
    /// This will create a new version with the contents of the previous version,
    /// but preserves all existing versions of the file.
    ///
    /// # See
    /// [Restore a previous version of a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitemversion_restore?view=odsp-graph-online)
    fn restore_version(&mut self, item_id: &str, version_id: &str) -> Request<StatusResponse> {
        self.format_me(item_id);
        self.as_post();
        self.extend_path(&["versions", version_id]);
        self.event(DriveEvent::RestoreVersion);
        Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::RestoreVersion,
        ))
    }

    /// Restore a previous version of a DriveItem to be the current version.
    /// This will create a new version with the contents of the previous version,
    /// but preserves all existing versions of the file.
    ///
    /// # See
    /// [Restore a previous version of a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitemversion_restore?view=odsp-graph-online)
    fn restore_drive_item_version(
        &mut self,
        item_id: &str,
        version: &DriveItemVersion,
    ) -> ItemResult<Request<StatusResponse>> {
        let version_id = version
            .id()
            .clone()
            .ok_or_else(|| GraphFailure::none_err("drive_item_version -> id"))?;
        Ok(self.restore_version(item_id, version_id.as_str()))
    }

    /// Enumerate activities (preview)
    /// May only work with the Graph beta endpoint.
    /// # See
    /// [Enumerate Activities](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/activities_list?view=odsp-graph-online)
    fn list_drive_activities(&mut self) -> Request<Collection<ItemActivity>> {
        self.extend_path(&["drive", "activities"]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Activities))
    }

    /// Enumerate activities (preview)
    /// May only work with the Graph beta endpoint.
    /// # See
    /// [Enumerate Activities](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/activities_list?view=odsp-graph-online)
    fn list_item_activities(&mut self, item_id: &str) -> Request<Collection<ItemActivity>> {
        self.extend_path(&["drive", "items", item_id, "activities"]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Activities))
    }

    /// Enumerate activities (preview)
    /// May only work with the Graph beta endpoint.
    /// # See
    /// [Enumerate Activities](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/activities_list?view=odsp-graph-online)
    fn activities_from_list_item(
        &mut self,
        item_id: &str,
        list_id: &str,
    ) -> Request<Collection<ItemActivity>> {
        self.extend_path(&["drive", "lists", list_id, "items", item_id, "activities"]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Activities))
    }

    /// Create an upload session to allow your app to upload files
    /// up to the maximum file size. This method is used for updating
    /// or replacing the content of an existing file.
    ///
    /// # See
    /// [Upload large files with an upload session](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_createuploadsession?view=odsp-graph-online)
    fn upload_session_replace(
        &mut self,
        item_id: &str,
        file: OsString,
    ) -> Request<UploadSessionPipeline> {
        self.format_me(item_id);
        self.extend_path(&["createUploadSession"]);
        self.set_upload_session(file);
        self.as_post();
        Request::new(Box::new(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::CreateUploadSession,
        )))
    }

    /// Create an upload session to allow your app to upload files
    /// up to the maximum file size. This method is used for uploading
    /// new files.
    ///
    /// # See
    /// [Upload large files with an upload session](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_createuploadsession?view=odsp-graph-online)
    fn upload_session_new(
        &mut self,
        path_from_root: OsString,
        file: OsString,
        create_upload_session: Option<CreateUploadSession>,
    ) -> Request<UploadSessionPipeline> {
        let mut s = path_from_root.to_os_string();
        s.push(":");
        self.extend_path(&["drive", "root:"]);
        self.extend_path_os_string(&[s]);
        self.extend_path(&["createUploadSession"]);
        self.set_upload_session(file);
        self.as_post();
        if let Some(upload) = create_upload_session {
            let upload = UploadSessionJson::new(upload);
            let upload_json = upload.as_json().unwrap();
            self.body(Body::String(upload_json));
        }
        Request::new(Box::new(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::CreateUploadSession,
        )))
    }
}

pub trait ItemCommon:
    MutateRequest + AsMut<DriveUrl> + AsMut<DataPipeline> + Into<SelectEvent>
{
    fn drive(&mut self, resource_id: &str) -> Request<DriveInfo> {
        self.extend_path(&[resource_id, "drives"]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Delete))
    }

    fn delete(&mut self, item_id: &str, resource_id: &str) -> Request<StatusResponse> {
        self.format_common(item_id, resource_id);
        self.as_delete();
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Delete))
    }

    fn delete_drive_item(&mut self, value: &DriveItem) -> ItemResult<Request<StatusResponse>> {
        let (item_id, resource_id) = value.item_event_ids()?;
        Ok(self.delete(item_id.as_str(), resource_id.as_str()))
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
    ) -> Request<StatusResponse> {
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
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Copy))
    }

    fn copy_drive_item(
        &mut self,
        drive_item: &DriveItem,
        name: Option<&str>,
    ) -> ItemResult<Request<StatusResponse>> {
        let (item_id, resource_id) = drive_item.item_event_ids()?;
        let item_ref = drive_item
            .parent_reference()
            .as_ref()
            .ok_or_else(|| GraphFailure::none_err("parent_reference"))?;
        Ok(self.copy(item_id.as_str(), resource_id.as_str(), &item_ref, name))
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

    fn create_folder_path(
        &mut self,
        resource_id: &str,
        path_from_root: &str,
        new_folder: NewFolder,
    ) -> Request<DriveItem> {
        if path_from_root.is_empty() {
            self.extend_path(&[resource_id, "root", "children"])
        } else if !path_from_root.ends_with(':') {
            self.extend_path(&[
                resource_id,
                "root:",
                format!("{}{}", path_from_root, ":").as_str(),
                "children",
            ]);
        } else {
            self.extend_path(&[resource_id, "root:", path_from_root, "children"]);
        }

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

    fn thumbnails(
        &mut self,
        item_id: &str,
        resource_id: &str,
    ) -> Request<Collection<ThumbnailSet>> {
        self.format_common(item_id, resource_id);
        self.extend_path(&["thumbnails"]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Thumbnails))
    }

    fn thumbnails_drive_item(&mut self, value: &DriveItem) -> Request<Collection<ThumbnailSet>> {
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
    ) -> Request<Vec<u8>> {
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
            .ok_or_else(|| GraphFailure::GraphError(Box::new(GraphError::default())))?;

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

    fn list_versions(
        &mut self,
        item_id: &str,
        resource_id: &str,
    ) -> Request<Collection<DriveItemVersion>> {
        self.format_common(item_id, resource_id);
        self.extend_path(&["versions"]);
        Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::ListVersions,
        ))
    }

    fn list_versions_drive_item(
        &mut self,
        resource_id: &str,
        drive_item: &DriveItem,
    ) -> ItemResult<Request<Collection<DriveItemVersion>>> {
        let item_id = drive_item
            .id()
            .as_ref()
            .ok_or_else(|| GraphFailure::none_err("item_id"))?;
        Ok(self.list_versions(item_id.as_str(), resource_id))
    }

    fn restore_version(
        &mut self,
        item_id: &str,
        version_id: &str,
        resource_id: &str,
    ) -> Request<StatusResponse> {
        self.format_common(item_id, resource_id);
        self.as_post();
        self.extend_path(&["versions", version_id]);
        self.event(DriveEvent::RestoreVersion);
        Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::RestoreVersion,
        ))
    }

    fn restore_drive_item_version(
        &mut self,
        item_id: &str,
        version: &DriveItemVersion,
        resource_id: &str,
    ) -> ItemResult<Request<StatusResponse>> {
        let version_id = version
            .id()
            .clone()
            .ok_or_else(|| GraphFailure::none_err("drive_item_version -> id"))?;
        Ok(self.restore_version(item_id, version_id.as_str(), resource_id))
    }

    /// Enumerate activities (preview)
    /// May only work with the Graph beta endpoint.
    /// # See
    /// [Enumerate Activities](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/activities_list?view=odsp-graph-online)
    fn list_drive_activities(&mut self, resource_id: &str) -> Request<Collection<ItemActivity>> {
        self.extend_path(&[resource_id, "activities"]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Activities))
    }

    /// Enumerate activities (preview)
    /// May only work with the Graph beta endpoint.
    /// # See
    /// [Enumerate Activities](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/activities_list?view=odsp-graph-online)
    fn list_item_activities(
        &mut self,
        item_id: &str,
        resource_id: &str,
    ) -> Request<Collection<ItemActivity>> {
        self.extend_path(&[resource_id, "items", item_id, "activities"]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Activities))
    }

    /// Enumerate activities (preview)
    /// May only work with the Graph beta endpoint.
    /// # See
    /// [Enumerate Activities](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/activities_list?view=odsp-graph-online)
    fn activities_from_list_item(
        &mut self,
        item_id: &str,
        list_id: &str,
        resource_id: &str,
    ) -> Request<Collection<ItemActivity>> {
        self.extend_path(&[
            resource_id,
            "lists",
            list_id,
            "items",
            item_id,
            "activities",
        ]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Activities))
    }

    /// Create an upload session to allow your app to upload files
    /// up to the maximum file size. This method is used for updating
    /// or replacing the content of an existing file.
    ///
    /// # See
    /// [Upload large files with an upload session](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_createuploadsession?view=odsp-graph-online)
    fn upload_session_replace(
        &mut self,
        item_id: &str,
        resource_id: &str,
        file: OsString,
    ) -> Request<UploadSessionPipeline> {
        self.format_common(item_id, resource_id);
        self.extend_path(&["createUploadSession"]);
        self.set_upload_session(file);
        self.as_post();
        Request::new(Box::new(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::CreateUploadSession,
        )))
    }

    /// Create an upload session to allow your app to upload files
    /// up to the maximum file size. This method is used for uploading
    /// new files.
    ///
    /// # See
    /// [Upload large files with an upload session](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_createuploadsession?view=odsp-graph-online)
    fn upload_session_new(
        &mut self,
        resource_id: &str,
        path_from_root: OsString,
        file: OsString,
        create_upload_session: Option<CreateUploadSession>,
    ) -> Request<UploadSessionPipeline> {
        let mut s = path_from_root.to_os_string();
        s.push(":");
        self.extend_path(&[resource_id, "root:"]);
        self.extend_path_os_string(&[s]);
        self.extend_path(&["createUploadSession"]);
        self.set_upload_session(file);
        self.as_post();
        if let Some(upload) = create_upload_session {
            let upload = UploadSessionJson::new(upload);
            let upload_json = upload.as_json().unwrap();
            self.body(Body::String(upload_json));
        }
        Request::new(Box::new(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::CreateUploadSession,
        )))
    }
}

mod item_sealed {
    use std::ffi::OsString;

    use crate::drive::driveurl::DriveUrl;
    use crate::drive::endpoint::DriveEndPoint;
    use crate::drive::event::DriveEvent;
    use crate::drive::item::SelectEventMe;
    use crate::drive::item::{SelectEvent, SelectResource};
    use crate::drive::pipelines::datapipeline::{Body, DataPipeline, RequestType};

    pub trait MutateRequest {
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

        fn set_upload_session(&mut self, file: OsString)
        where
            Self: AsMut<DataPipeline>,
        {
            self.as_mut().set_upload_session(file);
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

        fn extend_path_os_string(&mut self, path: &[OsString])
        where
            Self: AsMut<DriveUrl>,
        {
            self.as_mut().extend_path_os_string(path);
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

    impl MutateRequest for SelectResource {
        fn pipeline_data(&self) -> DataPipeline {
            self.0.clone()
        }
    }

    impl AsMut<DriveUrl> for SelectResource {
        fn as_mut(&mut self) -> &mut DriveUrl {
            self.0.as_mut()
        }
    }

    impl AsMut<DataPipeline> for SelectResource {
        fn as_mut(&mut self) -> &mut DataPipeline {
            &mut self.0
        }
    }

    impl From<DataPipeline> for SelectResource {
        fn from(data: DataPipeline) -> Self {
            SelectResource(data)
        }
    }

    impl MutateRequest for SelectEventMe {
        fn pipeline_data(&self) -> DataPipeline {
            self.0.clone()
        }
    }

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

    impl MutateRequest for SelectEvent {
        fn pipeline_data(&self) -> DataPipeline {
            self.0.clone()
        }
    }

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
}
