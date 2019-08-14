use crate::drive::drive_item::createdby::CreatedBy;
use crate::drive::drive_item::file::File;
use crate::drive::drive_item::filesysteminfo::FileSystemInfo;
use crate::drive::drive_item::itemreference::ItemReference;
use crate::drive::drive_item::lastmodifiedby::LastModifiedBy;
use crate::drive::drive_item::package::Package;
use crate::drive::drive_item::shared::Shared;
use crate::drive::drive_item::sharepointid::SharePointIds;
use std::io::Write;

#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
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
    parent_reference: Option<ItemReference>,
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
        parent_reference: Option<ItemReference>,
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
