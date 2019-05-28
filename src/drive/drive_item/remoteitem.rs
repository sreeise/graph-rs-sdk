use crate::drive::drive_item::createdby::CreatedBy;
use crate::drive::drive_item::file::File;
use crate::drive::drive_item::filesysteminfo::FileSystemInfo;
use crate::drive::drive_item::lastmodifiedby::LastModifiedBy;
use crate::drive::drive_item::package::Package;
use crate::drive::drive_item::parentreference::ParentReference;
use crate::drive::drive_item::shared::Shared;
use crate::drive::drive_item::sharepointid::SharePointIds;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
pub struct RemoteItem {
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
    #[serde(rename = "webDavUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    web_dav_url: Option<String>,
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_by: Option<CreatedBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<File>,
    #[serde(rename = "fileSystemInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_info: Option<FileSystemInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package: Option<Package>,
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_by: Option<LastModifiedBy>,
    #[serde(rename = "parentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_reference: Option<ParentReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared: Option<Shared>,
    #[serde(rename = "sharepointIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    share_point_ids: Option<SharePointIds>,
}

impl RemoteItem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        created_date_time: Option<String>,
        id: Option<String>,
        last_modified_date_time: Option<String>,
        name: Option<String>,
        size: Option<i64>,
        web_dav_url: Option<String>,
        web_url: Option<String>,
        created_by: Option<CreatedBy>,
        file: Option<File>,
        file_system_info: Option<FileSystemInfo>,
        package: Option<Package>,
        last_modified_by: Option<LastModifiedBy>,
        parent_reference: Option<ParentReference>,
        shared: Option<Shared>,
        share_point_ids: Option<SharePointIds>,
    ) -> Self {
        RemoteItem {
            created_date_time,
            id,
            last_modified_date_time,
            name,
            size,
            web_dav_url,
            web_url,
            created_by,
            file,
            file_system_info,
            package,
            last_modified_by,
            parent_reference,
            shared,
            share_point_ids,
        }
    }
}

impl RemoteItem {
    pub fn created_date_time(&self) -> Option<String> {
        self.created_date_time.clone()
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

    pub fn web_dav_url(&self) -> Option<String> {
        self.web_dav_url.clone()
    }

    pub fn created_by(&self) -> Option<CreatedBy> {
        self.created_by.clone()
    }

    pub fn file(&self) -> Option<File> {
        self.file.clone()
    }

    pub fn last_modified_by(&self) -> Option<LastModifiedBy> {
        self.last_modified_by.clone()
    }

    pub fn parent_reference(&self) -> Option<ParentReference> {
        self.parent_reference.clone()
    }

    pub fn file_system_info(&self) -> Option<FileSystemInfo> {
        self.file_system_info.clone()
    }

    pub fn package(&self) -> Option<Package> {
        self.package.clone()
    }

    pub fn shared(&self) -> Option<Shared> {
        self.shared.clone()
    }

    pub fn share_point_ids(&self) -> Option<SharePointIds> {
        self.share_point_ids.clone()
    }
}
