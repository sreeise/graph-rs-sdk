use crate::drive::base::createdby::CreatedBy;
use crate::drive::base::file::File;
use crate::drive::base::filesysteminfo::FileSystemInfo;
use crate::drive::base::folder::Folder;
use crate::drive::base::image::Image;
use crate::drive::base::lastmodifiedby::LastModifiedBy;
use crate::drive::base::parentreference::ParentReference;
use crate::drive::base::photo::Photo;
use crate::drive::base::remoteitem::RemoteItem;
use crate::drive::base::specialfolder::SpecialFolder;
use crate::drive::base::Root;
use std::io;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Value {
    #[serde(rename = "@odata.context")]
    _odata_context: Option<String>,
    #[serde(rename = "@odata.type")]
    _odata_type: Option<String>,
    #[serde(rename = "createdDateTime")]
    created_date_time: Option<String>,
    #[serde(rename = "cTag")]
    c_tag: Option<String>,
    #[serde(rename = "eTag")]
    e_tag: Option<String>,
    id: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    last_modified_date_time: Option<String>,
    name: Option<String>,
    size: Option<i64>,
    #[serde(rename = "webUrl")]
    web_url: Option<String>,
    #[serde(rename = "createdBy")]
    created_by: Option<CreatedBy>,
    #[serde(rename = "lastModifiedBy")]
    last_modified_by: Option<LastModifiedBy>,
    #[serde(rename = "parentReference")]
    parent_reference: Option<ParentReference>,
    #[serde(rename = "fileSystemInfo")]
    file_system_info: Option<FileSystemInfo>,
    folder: Option<Folder>,
    #[serde(rename = "specialFolder")]
    special_folder: Option<SpecialFolder>,
    #[serde(rename = "@microsoft.graph.downloadUrl")]
    _microsoft_graph_download_url: Option<String>,
    file: Option<File>,
    image: Option<Image>,
    photo: Option<Photo>,
    root: Option<Root>,
    #[serde(rename = "remoteItem")]
    remote_item: Option<RemoteItem>,
}

impl Value {
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
        self.size.clone()
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
