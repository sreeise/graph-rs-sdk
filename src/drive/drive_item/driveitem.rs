use crate::drive::drive_item::audio::Audio;
use crate::drive::drive_item::deleted::Deleted;
use crate::drive::drive_item::driveitemversion::DriveItemVersion;
use crate::drive::drive_item::file::File;
use crate::drive::drive_item::filesysteminfo::FileSystemInfo;
use crate::drive::drive_item::folder::Folder;
use crate::drive::drive_item::geocorrdinates::GeoCoordinates;
use crate::drive::drive_item::identityset::IdentitySet;
use crate::drive::drive_item::image::Image;
use crate::drive::drive_item::itemactivity::ItemActivity;
use crate::drive::drive_item::itemreference::ItemReference;
use crate::drive::drive_item::lastmodifiedby::LastModifiedBy;
use crate::drive::drive_item::package::Package;
use crate::drive::drive_item::permissions::Permissions;
use crate::drive::drive_item::photo::Photo;
use crate::drive::drive_item::publicationfacet::PublicationFacet;
use crate::drive::drive_item::remoteitem::RemoteItem;
use crate::drive::drive_item::searchresult::SearchResult;
use crate::drive::drive_item::shared::Shared;
use crate::drive::drive_item::sharepointid::SharePointIds;
use crate::drive::drive_item::specialfolder::SpecialFolder;
use crate::drive::drive_item::video::Video;
use crate::drive::drive_item::Root;
use crate::drive::thumbnail::ThumbnailSet;
use crate::drive::ItemResult;
use from_to_file::*;
use graph_error::{GraphError, GraphFailure};
use reqwest::Response;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::io;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromToFile, Setters)]
#[set = "pub set"]
pub struct DriveItem {
    #[serde(rename = "@odata.context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    odata_context: Option<String>,
    #[serde(rename = "@odata.type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    odata_type: Option<String>,
    #[serde(rename = "fileSystemInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_info: Option<FileSystemInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio: Option<Audio>,
    // TODO: The odata.type for content is "Edm.Stream". Figure out what Edm.Stream is.
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(rename = "cTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    c_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deleted: Option<Deleted>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    folder: Option<Folder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<GeoCoordinates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    malware: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package: Option<Package>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo: Option<Photo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publication: Option<PublicationFacet>,
    #[serde(rename = "remoteItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_item: Option<RemoteItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root: Option<Root>,
    #[serde(rename = "searchResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    search_result: Option<SearchResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared: Option<Shared>,
    #[serde(rename = "sharepointIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sharepoint_ids: Option<SharePointIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
    #[serde(rename = "specialFolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    special_folder: Option<SpecialFolder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video: Option<Video>,
    #[serde(rename = "webDavUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    web_dav_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activities: Option<Vec<ItemActivity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<DriveItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnails: Option<Vec<ThumbnailSet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    versions: Option<Vec<DriveItemVersion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_by: Option<IdentitySet>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_date_time: Option<String>,
    #[serde(rename = "eTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    e_tag: Option<String>,
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_by: Option<LastModifiedBy>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "parentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_reference: Option<ItemReference>,
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<String>,
    #[serde(rename = "@microsoft.graph.conflictBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    microsoft_graph_conflict_behavior: Option<String>,
    #[serde(rename = "@microsoft.graph.downloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    microsoft_graph_download_url: Option<String>,
    #[serde(rename = "@microsoft.graph.sourceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    microsoft_graph_source_url: Option<String>,
}

impl DriveItem {
    pub fn copy(&self) -> io::Result<DriveItem> {
        Ok(self.clone())
    }

    pub fn data_type(&self) -> Option<String> {
        self.odata_type.clone()
    }

    pub fn data_context(&self) -> Option<String> {
        self.odata_context.clone()
    }

    pub fn created_date_time(&self) -> Option<String> {
        self.created_date_time.clone()
    }

    pub fn c_tag(&self) -> Option<String> {
        self.c_tag.clone()
    }

    pub fn e_tag(&self) -> Option<String> {
        self.e_tag.clone()
    }

    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn last_modified_date_time(&self) -> Option<String> {
        self.last_modified_date_time.clone()
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn size(&self) -> Option<i64> {
        self.size
    }

    pub fn web_url(&self) -> Option<String> {
        self.web_url.clone()
    }

    pub fn created_by(&self) -> Option<IdentitySet> {
        self.created_by.clone()
    }

    pub fn last_modified_by(&self) -> Option<LastModifiedBy> {
        self.last_modified_by.clone()
    }

    pub fn parent_reference(&self) -> Option<ItemReference> {
        self.parent_reference.clone()
    }

    pub fn file_system_info(&self) -> Option<FileSystemInfo> {
        self.file_system_info.clone()
    }

    pub fn folder(&self) -> Option<Folder> {
        self.folder.clone()
    }

    pub fn special_folder(&self) -> Option<SpecialFolder> {
        self.special_folder.clone()
    }

    pub fn microsoft_graph_download_url(&self) -> Option<String> {
        self.microsoft_graph_download_url.clone()
    }

    pub fn file(&self) -> Option<File> {
        self.file.clone()
    }

    pub fn image(&self) -> Option<Image> {
        self.image.clone()
    }

    pub fn photo(&self) -> Option<Photo> {
        self.photo.clone()
    }

    pub fn root(&self) -> Option<Root> {
        self.root.clone()
    }

    pub fn remote_item(&self) -> Option<RemoteItem> {
        self.remote_item.clone()
    }

    pub fn item_event_ids(&self) -> ItemResult<(String, String)> {
        let item_id = self
            .id()
            .clone()
            .ok_or_else(|| GraphFailure::none_err("drive_item -> id"))?;
        let pr = self
            .parent_reference()
            .clone()
            .ok_or_else(|| GraphFailure::none_err("drive_item -> parent_reference"))?;
        let drive_id = pr
            .drive_id()
            .clone()
            .ok_or_else(|| GraphFailure::none_err("drive_item -> parent_reference -> drive_id"))?;
        Ok((item_id, drive_id))
    }

    pub fn parent_event_ids(&self) -> ItemResult<(String, String)> {
        let pr = self
            .parent_reference()
            .clone()
            .ok_or_else(|| GraphFailure::none_err("drive_item -> parent_reference"))?;
        let parent_id = pr
            .id()
            .clone()
            .ok_or_else(|| GraphFailure::none_err("drive_item -> parent_reference -> id"))?;
        let drive_id = pr
            .drive_id()
            .clone()
            .ok_or_else(|| GraphFailure::none_err("drive_item -> parent_reference -> drive_id"))?;
        Ok((drive_id, parent_id))
    }
}

impl Eq for DriveItem {}

impl TryFrom<&mut Response> for DriveItem {
    type Error = GraphFailure;

    fn try_from(response: &mut Response) -> Result<Self, Self::Error> {
        let status = response.status().as_u16();
        if GraphError::is_error(status) {
            return Err(GraphFailure::from(
                GraphError::try_from(status).unwrap_or_default(),
            ));
        }

        let v: DriveItem = response.json()?;
        Ok(v)
    }
}
