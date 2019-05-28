use std::io;

use crate::drive::drive_item::createdby::CreatedBy;
use crate::drive::drive_item::file::File;
use crate::drive::drive_item::filesysteminfo::FileSystemInfo;
use crate::drive::drive_item::folder::Folder;
use crate::drive::drive_item::image::Image;
use crate::drive::drive_item::lastmodifiedby::LastModifiedBy;
use crate::drive::drive_item::parentreference::ParentReference;
use crate::drive::drive_item::photo::Photo;
use crate::drive::drive_item::remoteitem::RemoteItem;
use crate::drive::drive_item::specialfolder::SpecialFolder;
use crate::drive::drive_item::Root;
use from_to_file::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromToFile, Setters)]
#[set = "pub set"]
pub struct Value {
    #[serde(rename = "@odata.context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    _odata_context: Option<String>,
    #[serde(rename = "@odata.type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    _odata_type: Option<String>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_date_time: Option<String>,
    #[serde(rename = "cTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    c_tag: Option<String>,
    #[serde(rename = "eTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    e_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_by: Option<CreatedBy>,
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_by: Option<LastModifiedBy>,
    #[serde(rename = "parentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_reference: Option<ParentReference>,
    #[serde(rename = "fileSystemInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_info: Option<FileSystemInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    folder: Option<Folder>,
    #[serde(rename = "specialFolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    special_folder: Option<SpecialFolder>,
    #[serde(rename = "microsoft_graph_download_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    _microsoft_graph_download_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo: Option<Photo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root: Option<Root>,
    #[serde(rename = "remoteItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_item: Option<RemoteItem>,
}

impl Value {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        _odata_context: Option<String>,
        _odata_type: Option<String>,
        created_date_time: Option<String>,
        c_tag: Option<String>,
        e_tag: Option<String>,
        id: Option<String>,
        last_modified_date_time: Option<String>,
        name: Option<String>,
        size: Option<i64>,
        web_url: Option<String>,
        created_by: Option<CreatedBy>,
        last_modified_by: Option<LastModifiedBy>,
        parent_reference: Option<ParentReference>,
        file_system_info: Option<FileSystemInfo>,
        folder: Option<Folder>,
        special_folder: Option<SpecialFolder>,
        _microsoft_graph_download_url: Option<String>,
        file: Option<File>,
        image: Option<Image>,
        photo: Option<Photo>,
        root: Option<Root>,
        remote_item: Option<RemoteItem>,
    ) -> Self {
        Value {
            _odata_context,
            _odata_type,
            created_date_time,
            c_tag,
            e_tag,
            id,
            last_modified_date_time,
            name,
            size,
            web_url,
            created_by,
            last_modified_by,
            parent_reference,
            file_system_info,
            folder,
            special_folder,
            _microsoft_graph_download_url,
            file,
            image,
            photo,
            root,
            remote_item,
        }
    }
}

impl Value {
    pub fn copy(&self) -> io::Result<Value> {
        Ok(self.clone())
    }

    pub fn data_type(&self) -> Option<String> {
        self._odata_type.clone()
    }

    pub fn data_context(&self) -> Option<String> {
        self._odata_context.clone()
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

    pub fn created_by(&self) -> Option<CreatedBy> {
        self.created_by.clone()
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

    pub fn folder(&self) -> Option<Folder> {
        self.folder.clone()
    }

    pub fn special_folder(&self) -> Option<SpecialFolder> {
        self.special_folder.clone()
    }

    pub fn microsoft_graph_download_url(&self) -> Option<String> {
        self._microsoft_graph_download_url.clone()
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
}
