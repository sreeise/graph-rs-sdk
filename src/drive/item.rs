use crate::drive::drive_item::collection::Collection;
use crate::drive::drive_item::driveinfo::DriveInfo;
use crate::drive::drive_item::driveitem::DriveItem;
use crate::drive::drive_item::driveitemversion::DriveItemVersion;
use crate::drive::drive_item::itemactivity::ItemActivity;
use crate::drive::drive_item::itemreference::ItemReference;
use crate::drive::drive_item::preview::Preview;
use crate::drive::drive_item::thumbnail::{Thumbnail, ThumbnailSet};
use crate::drive::driveurl::MutateUrl;
use crate::drive::event::{CreateUploadSession, EmbeddableUrl, ItemRefCopy, UploadSessionJson};
use crate::drive::event::{DriveEvent, NewFolder};
use crate::drive::item::item_sealed::MutateRequest;
use crate::drive::pipelines::datapipeline::Body;
use crate::drive::pipelines::datapipeline::DataPipeline;
use crate::drive::pipelines::downloadpipeline::DownloadPipeline;
use crate::drive::pipelines::downloadpipeline::FetchPipeline;
use crate::drive::pipelines::pipeline::Pipeline;
use crate::drive::pipelines::request::{
    BoxedDownload, BoxedDownloadCommon, BoxedRequest, BoxedRequestCommon, ByItem, Formatting,
    ReqBuilder, Request,
};
use crate::drive::pipelines::uploadsessionpipeline::UploadSessionPipeline;
use crate::drive::statusresponse::StatusResponse;
use crate::drive::ItemResult;
use crate::prelude::DriveUrl;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use graph_error::{GraphFailure, GraphError};

pub trait IntoItem<T>: MutateUrl + AsRef<DriveUrl> + AsRef<DriveEvent> {
    fn send(&mut self) -> ItemResult<T>;

    fn drive_event(&mut self) -> DriveEvent;
}

pub struct SelectEventMe(DataPipeline);

impl SelectEventMe {
    /// Enumerate activities (preview)
    /// May only work with the Graph beta endpoint.
    /// # See
    /// [Enumerate Activities](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/activities_list?view=odsp-graph-online)
    pub fn list_drive_activities(&mut self) -> Request<Collection<ItemActivity>> {
        self.0.url.extend_path(&["drive", "activities"]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Activities))
    }
}

impl From<SelectResource> for SelectEventMe {
    fn from(resource: SelectResource) -> Self {
        SelectEventMe(resource.0)
    }
}

pub struct SelectEvent(DataPipeline);

impl SelectEvent {
    pub fn drive(&mut self, resource_id: &str) -> Request<DriveInfo> {
        self.0.url.extend_path(&[resource_id, "drives"]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::GetItem))
    }

    /// Enumerate activities (preview)
    /// May only work with the Graph beta endpoint.
    /// # See
    /// [Enumerate Activities](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/activities_list?view=odsp-graph-online)
    pub fn list_drive_activities(
        &mut self,
        resource_id: &str,
    ) -> Request<Collection<ItemActivity>> {
        self.0.url.extend_path(&[resource_id, "activities"]);
        Request::from(Pipeline::new(self.pipeline_data(), DriveEvent::Activities))
    }
}

#[derive(Clone)]
pub struct SelectResource(DataPipeline);

impl SelectResource {
    pub fn new(data: DataPipeline) -> SelectResource {
        SelectResource(data)
    }

    pub fn me(mut self) -> SelectEventMe {
        self.0.url.extend_path(&["me"]);
        self.into()
    }

    pub fn drives(mut self) -> SelectEvent {
        self.0.url.extend_path(&["drives"]);
        self.into()
    }

    pub fn sites(mut self) -> SelectEvent {
        self.0.url.extend_path(&["sites"]);
        self.into()
    }

    pub fn groups(mut self) -> SelectEvent {
        self.0.url.extend_path(&["groups"]);
        self.into()
    }

    pub fn users(mut self) -> SelectEvent {
        self.0.url.extend_path(&["users"]);
        self.into()
    }

    pub fn get(&mut self) -> ReqBuilder {
        self.0.as_get();
        ReqBuilder::new(self.0.clone())
    }

    pub fn post(&mut self) -> ReqBuilder {
        self.0.as_post();
        ReqBuilder::new(self.0.clone())
    }

    pub fn put(&mut self) -> ReqBuilder {
        self.0.as_put();
        ReqBuilder::new(self.0.clone())
    }

    pub fn patch(&mut self) -> ReqBuilder {
        self.0.as_patch();
        ReqBuilder::new(self.0.clone())
    }

    pub fn delete(&mut self) -> ReqBuilder {
        self.0.as_delete();
        ReqBuilder::new(self.0.clone())
    }
}

pub trait AsEvent: MutateRequest + AsMut<DataPipeline> {
    type AsStatusResponse: ByItem<StatusResponse>;
    type AsDriveItem: ByItem<DriveItem>;
    type AsPathBuf: ByItem<PathBuf>;
    type AsDownload: From<DownloadPipeline>;
    type AsBinary: ByItem<Vec<u8>>;
    type AsCollectionThumbnailSet: ByItem<Collection<ThumbnailSet>>;
    type AsThumbnail: ByItem<Thumbnail>;
    type AsCollectionItemActivity: ByItem<Collection<ItemActivity>>;
    type AsUploadSession: ByItem<UploadSessionPipeline>;
    type AsPreview: ByItem<Preview>;
    type AsCollectionVersion: ByItem<Collection<DriveItemVersion>>;

    fn delete(&mut self) -> Self::AsStatusResponse {
        self.as_delete();
        Self::AsStatusResponse::from(Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::Delete,
        )))
    }

    fn get_item(&mut self) -> Self::AsDriveItem {
        Self::AsDriveItem::from(Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::GetItem,
        )))
    }

    fn create_folder(&mut self, folder: NewFolder) -> Self::AsDriveItem {
        self.body(Body::String(serde_json::to_string_pretty(&folder).unwrap()));
        self.as_post();
        Self::AsDriveItem::from(Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::CreateFolder,
        )))
    }

    /// Asynchronously creates a copy of an driveItem (including any children),
    /// under a new parent item or with a new name.
    ///
    /// # See
    /// [Copy a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_copy?view=odsp-graph-online)
    fn copy(&mut self, name: Option<&str>, item_ref: &ItemReference) -> Self::AsStatusResponse {
        let prc = ItemRefCopy::new(item_ref.clone(), name.map(|s| s.to_string()));
        self.body(Body::String(prc.as_json().unwrap()));
        self.as_post();
        Self::AsStatusResponse::from(Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::Copy,
        )))
    }

    /// Download the contents of the primary stream (file) of a DriveItem.
    ///
    /// # See
    /// [Download the contents of a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_get_content?view=odsp-graph-online)
    fn download<P: AsRef<Path>>(&mut self, directory: P) -> Self::AsDownload {
        let mut fetch_pipeline = FetchPipeline::from(self.pipeline_data());
        let mut path_buf = PathBuf::new();
        path_buf.push(directory);
        fetch_pipeline.directory = path_buf;

        Self::AsDownload::from(DownloadPipeline {
            pipeline: fetch_pipeline,
            is_direct_download: false,
        })
    }

    /// Retrieve a single thumbnail for a DriveItem
    ///
    /// # See
    /// [List thumbnails for a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_list_thumbnails?view=odsp-graph-online)
    fn single_thumbnail(&mut self, thumb_id: &str, size: &str) -> Self::AsThumbnail {
        let mut request = Self::AsThumbnail::from(Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::Thumbnails,
        )));
        request.custom_format(Formatting::SingleThumbnail(thumb_id.into(), size.into()));
        request
    }

    /// Retrieve a collection of ThumbnailSet resources for a DriveItem resource.
    ///
    /// # See
    /// [List thumbnails for a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_list_thumbnails?view=odsp-graph-online)
    fn thumbnails(&mut self) -> Self::AsCollectionThumbnailSet {
        Self::AsCollectionThumbnailSet::from(Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::Thumbnails,
        )))
    }

    /// Update the metadata for a DriveItem by ID.
    ///
    /// # See
    /// [Update DriveItem properties](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_put_content?view=odsp-graph-online)
    fn thumbnail_binary(&mut self, thumb_id: &str, size: &str) -> Self::AsBinary {
        let mut request = Self::AsBinary::from(self.pipeline_data().get(DriveEvent::Thumbnails));
        request.custom_format(Formatting::ThumbnailBinary(thumb_id.into(), size.into()));
        request
    }

    /// Update the metadata for a DriveItem by ID.
    ///
    /// # See
    /// [Update DriveItem properties](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_put_content?view=odsp-graph-online)
    fn update(&mut self, new_value: &DriveItem) -> Self::AsDriveItem {
        self.body(Body::String(
            serde_json::to_string_pretty(&new_value).unwrap(),
        ));
        Self::AsDriveItem::from(self.pipeline_data().patch(DriveEvent::Update))
    }

    /// The simple upload API allows you to provide the contents of a new
    /// file or update the contents of an existing file in a single API call.
    /// This method only supports files up to 4MB in size.
    ///
    /// # See
    /// [Upload or replace the contents of a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_put_content?view=odsp-graph-online)
    fn upload_replace<P: AsRef<Path>>(&mut self, file: P) -> Self::AsDriveItem {
        self.body(Body::File(OsString::from(file.as_ref())));
        Self::AsDriveItem::from(self.pipeline_data().put(DriveEvent::Upload))
    }

    /// Crate a new file or folder. The simple upload API allows you to provide
    /// the contents of a new file or update the contents of an existing file in
    /// a single API call. This method only supports files up to 4MB in size.
    ///
    /// # See
    /// [Upload or replace the contents of a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_put_content?view=odsp-graph-online)
    fn upload_new<P: AsRef<Path>>(&mut self, file: P) -> ItemResult<Self::AsDriveItem> {
        let name = file
            .as_ref()
            .file_name()
            .ok_or_else(|| GraphFailure::GraphError(Box::new(GraphError::default())))?;

        let mut f = OsString::new();
        f.push(file.as_ref());
        self.body(Body::File(f));
        let mut request = Self::AsDriveItem::from(self.pipeline_data().put(DriveEvent::Upload));
        request.custom_format(Formatting::UploadNew(name.to_os_string()));
        Ok(request)
    }

    /// List versions for a DriveItem given a DriveItem id or path.
    ///
    /// OneDrive and SharePoint can be configured to retain the history for files.
    /// Depending on the service and configuration, a new version can be created for
    /// each edit, each time the file is saved, manually,
    ///
    /// # See
    /// [Listing versions of a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_list_versions?view=odsp-graph-online)
    fn list_versions(&mut self) -> Self::AsCollectionVersion {
        Self::AsCollectionVersion::from(self.pipeline_data().get(DriveEvent::ListVersions))
    }

    /// Restore a previous version of a DriveItem to be the current version.
    /// This will create a new version with the contents of the previous version,
    /// but preserves all existing versions of the file.
    ///
    /// # See
    /// [Restore a previous version of a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitemversion_restore?view=odsp-graph-online)
    fn restore_version(&mut self, version_id: &str) -> Self::AsStatusResponse {
        self.as_post();
        let mut request = Self::AsStatusResponse::from(Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::RestoreVersion,
        )));
        request.custom_format(Formatting::RestoreVersions(version_id.into()));
        request
    }

    /// Enumerate activities (preview)
    /// May only work with the Graph beta endpoint.
    /// # See
    /// [Enumerate Activities](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/activities_list?view=odsp-graph-online)
    fn list_item_activities(&mut self) -> Self::AsCollectionItemActivity {
        Self::AsCollectionItemActivity::from(self.pipeline_data().get(DriveEvent::Activities))
    }

    /// Enumerate activities (preview)
    /// May only work with the Graph beta endpoint.
    /// # See
    /// [Enumerate Activities](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/activities_list?view=odsp-graph-online)
    fn activities_from_list_item(&mut self, list_id: &str) -> Self::AsCollectionItemActivity {
        let mut request = Self::AsCollectionItemActivity::from(self.pipeline_data().get(DriveEvent::Activities));
        request.custom_format(Formatting::ActivitiesLists(list_id.into()));
        request
    }

    /// Create an upload session to allow your app to upload files
    /// up to the maximum file size. This method is used for updating
    /// or replacing the content of an existing file.
    ///
    /// # See
    /// [Upload large files with an upload session](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_createuploadsession?view=odsp-graph-online)
    fn upload_session_replace(&mut self, file: OsString) -> Self::AsUploadSession {
        self.set_upload_session(file);
        self.as_post();
        Self::AsUploadSession::from(Request::new(Box::new(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::CreateUploadSession,
        ))))
    }

    /// Create an upload session to allow your app to upload files
    /// up to the maximum file size. This method is used for uploading
    /// new files.
    ///
    /// # See
    /// [Upload large files with an upload session](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_createuploadsession?view=odsp-graph-online)
    fn upload_session_new<P: AsRef<Path>>(
        &mut self,
        file: P,
        create_upload_session: Option<CreateUploadSession>,
    ) -> Self::AsUploadSession {
        self.set_upload_session(OsString::from(file.as_ref()));
        self.as_post();
        if let Some(upload) = create_upload_session {
            let upload = UploadSessionJson::new(upload);
            let upload_json = upload.as_json().unwrap();
            self.body(Body::String(upload_json));
        } else {
            self.header(
                reqwest::header::CONTENT_LENGTH,
                reqwest::header::HeaderValue::from(0),
            );
        }
        Self::AsUploadSession::from(Request::new(Box::new(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::CreateUploadSession,
        ))))
    }

    /// Previews using drive item id or path.
    /// This action allows you to obtain short-lived embeddable URLs for an item.
    /// Only works for SharePoint and OneDrive for business accounts.
    ///
    /// # See
    /// [Embeddable file previews](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_preview?view=odsp-graph-online)
    fn preview(&mut self, embeddable_url: Option<EmbeddableUrl>) -> Self::AsPreview {
        self.as_post();
        if let Some(embeddable_url) = embeddable_url {
            self.body(Body::String(embeddable_url.as_json().unwrap()))
        } else {
            self.header(
                reqwest::header::CONTENT_LENGTH,
                reqwest::header::HeaderValue::from(0),
            );
        }
        Self::AsPreview::from(Request::from(Pipeline::new(
            self.pipeline_data(),
            DriveEvent::Preview,
        )))
    }
}

impl AsEvent for SelectEventMe {
    type AsStatusResponse = BoxedRequest<StatusResponse>;
    type AsDriveItem = BoxedRequest<DriveItem>;
    type AsPathBuf = BoxedRequest<PathBuf>;
    type AsDownload = BoxedDownload;
    type AsBinary = BoxedRequest<Vec<u8>>;
    type AsCollectionThumbnailSet = BoxedRequest<Collection<ThumbnailSet>>;
    type AsThumbnail = BoxedRequest<Thumbnail>;
    type AsCollectionItemActivity = BoxedRequest<Collection<ItemActivity>>;
    type AsUploadSession = BoxedRequest<UploadSessionPipeline>;
    type AsPreview = BoxedRequest<Preview>;
    type AsCollectionVersion = BoxedRequest<Collection<DriveItemVersion>>;
}

impl AsEvent for SelectEvent {
    type AsStatusResponse = BoxedRequestCommon<StatusResponse>;
    type AsDriveItem = BoxedRequestCommon<DriveItem>;
    type AsPathBuf = BoxedRequestCommon<PathBuf>;
    type AsDownload = BoxedDownloadCommon;
    type AsBinary = BoxedRequestCommon<Vec<u8>>;
    type AsCollectionThumbnailSet = BoxedRequestCommon<Collection<ThumbnailSet>>;
    type AsThumbnail = BoxedRequestCommon<Thumbnail>;
    type AsCollectionItemActivity = BoxedRequestCommon<Collection<ItemActivity>>;
    type AsUploadSession = BoxedRequestCommon<UploadSessionPipeline>;
    type AsPreview = BoxedRequestCommon<Preview>;
    type AsCollectionVersion = BoxedRequestCommon<Collection<DriveItemVersion>>;
}

pub(crate) mod item_sealed {
    use crate::drive::driveurl::DriveUrl;
    use crate::drive::endpoint::DriveEndPoint;
    use crate::drive::event::DriveEvent;
    use crate::drive::item::SelectEventMe;
    use crate::drive::item::{SelectEvent, SelectResource};
    use crate::drive::pipelines::datapipeline::{Body, DataPipeline, RequestType};
    use reqwest::header::{HeaderValue, IntoHeaderName};
    use std::ffi::OsString;

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

        fn header<K>(&mut self, key: K, val: HeaderValue)
        where
            Self: AsMut<DataPipeline>,
            K: IntoHeaderName,
        {
            self.as_mut().header(key, val);
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
