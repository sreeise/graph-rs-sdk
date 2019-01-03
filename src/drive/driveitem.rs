use crate::drive::DriveSerDe;
use crate::process::jsonio::JsonFile;
use std::io;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveItem {
    #[serde(rename = "@odata.context")]
    _odata_context: String,
    value: Vec<Value>,
}

impl DriveItem {
    pub fn new(data_context: String, value: Vec<Value>) -> DriveItem {
        DriveItem {
            _odata_context: data_context,
            value,
        }
    }

    pub fn data_context(&self) -> String {
        self._odata_context.clone()
    }

    pub fn value(&self) -> Vec<Value> {
        self.value.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Value {
    #[serde(rename = "createdDateTime")]
    created_date_time: String,
    #[serde(rename = "cTag")]
    c_tag: String,
    #[serde(rename = "eTag")]
    e_tag: String,
    id: String,
    #[serde(rename = "lastModifiedDateTime")]
    last_modified_date_time: String,
    name: String,
    size: i64,
    #[serde(rename = "webUrl")]
    web_url: String,
    #[serde(rename = "createdBy")]
    created_by: CreatedBy,
    #[serde(rename = "lastModifiedBy")]
    last_modified_by: LastModifiedBy,
    #[serde(rename = "parentReference")]
    parent_reference: ParentReference,
    #[serde(rename = "fileSystemInfo")]
    file_system_info: FileSystemInfo,
    folder: Option<Folder>,
    #[serde(rename = "specialFolder")]
    special_folder: Option<SpecialFolder>,
    #[serde(rename = "@microsoft.graph.downloadUrl")]
    _microsoft_graph_download_url: Option<String>,
    file: Option<File>,
}

impl Value {
    pub fn new(
        created_date_time: String,
        c_tag: String,
        e_tag: String,
        id: String,
        last_modified_date_time: String,
        name: String,
        size: i64,
        web_url: String,
        created_by: CreatedBy,
        last_modified_by: LastModifiedBy,
        parent_reference: ParentReference,
        file_system_info: FileSystemInfo,
        folder: Option<Folder>,
        special_folder: Option<SpecialFolder>,
        _microsoft_graph_download_url: Option<String>,
        file: Option<File>,
    ) -> Self {
        Value {
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
        }
    }
}

impl Value {
    pub fn copy(&self) -> io::Result<Value> {
        Ok(self.clone())
    }

    pub fn created_date_time(&self) -> String {
        self.created_date_time.clone()
    }

    pub fn c_tag(&self) -> String {
        self.c_tag.clone()
    }

    pub fn e_tag(&self) -> String {
        self.e_tag.clone()
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn last_modified_date_time(&self) -> String {
        self.last_modified_date_time.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn size(&self) -> i64 {
        self.size.clone()
    }

    pub fn web_url(&self) -> String {
        self.web_url.clone()
    }

    pub fn created_by(&self) -> CreatedBy {
        self.created_by.clone()
    }

    pub fn last_modified_by(&self) -> LastModifiedBy {
        self.last_modified_by.clone()
    }

    pub fn parent_reference(&self) -> ParentReference {
        self.parent_reference.clone()
    }

    pub fn file_system_info(&self) -> FileSystemInfo {
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatedBy {
    user: User,
    application: Option<Application>,
}

impl CreatedBy {
    pub fn user(&self) -> User {
        self.user.clone()
    }

    pub fn application(&self) -> Option<Application> {
        self.application.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "displayName")]
    display_name: String,
    id: String,
}

impl User {
    pub fn display_name(&self) -> String {
        self.display_name.clone()
    }

    pub fn id(&self) -> String {
        self.display_name.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(rename = "displayName")]
    display_name: String,
    id: String,
}

impl Application {
    pub fn display_name(&self) -> String {
        self.display_name.clone()
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastModifiedBy {
    user: User,
    application: Option<Application>,
}

impl LastModifiedBy {
    pub fn user(&self) -> User {
        self.user.clone()
    }

    pub fn application(&self) -> Option<Application> {
        self.application.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParentReference {
    #[serde(rename = "driveId")]
    drive_id: String,
    #[serde(rename = "driveType")]
    drive_type: String,
    id: String,
    path: String,
}

impl ParentReference {
    pub fn drive_id(&self) -> String {
        self.drive_id.clone()
    }

    pub fn drive_type(&self) -> String {
        self.drive_type.clone()
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileSystemInfo {
    #[serde(rename = "createdDateTime")]
    created_date_time: String,
    #[serde(rename = "lastModifiedDateTime")]
    last_modified_date_time: String,
}

impl FileSystemInfo {
    pub fn created_date_time(&self) -> String {
        self.created_date_time.clone()
    }

    pub fn last_modified_time(&self) -> String {
        self.last_modified_date_time.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Folder {
    #[serde(rename = "childCount")]
    child_count: i64,
    view: View,
}

impl Folder {
    pub fn child_count(&self) -> i64 {
        self.child_count.clone()
    }

    pub fn view(&self) -> View {
        self.view.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct View {
    #[serde(rename = "viewType")]
    view_type: String,
    #[serde(rename = "sortBy")]
    sort_by: String,
    #[serde(rename = "sortOrder")]
    sort_order: String,
}

impl View {
    pub fn view_type(&self) -> String {
        self.view_type.clone()
    }

    pub fn sort_by(&self) -> String {
        self.sort_by.clone()
    }

    pub fn sort_order(&self) -> String {
        self.sort_order.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecialFolder {
    name: String,
}

impl SpecialFolder {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "mimeType")]
    mime_type: String,
    hashes: Hashes,
}

impl File {
    pub fn mime_type(&self) -> String {
        self.mime_type.clone()
    }

    pub fn hashes(&self) -> Hashes {
        self.hashes.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hashes {
    #[serde(rename = "sha1Hash")]
    sha1_hash: String,
    #[serde(rename = "quickXorHash")]
    quick_xor_hash: Option<String>,
    #[serde(rename = "crc32Hash")]
    crc32_hash: Option<String>,
}

impl Hashes {
    pub fn sha1_hash(&self) -> String {
        self.sha1_hash.clone()
    }

    pub fn quick_xor_hash(&self) -> Option<String> {
        self.quick_xor_hash.clone()
    }

    pub fn crc32_hash(&self) -> Option<String> {
        self.crc32_hash.clone()
    }
}

impl DriveSerDe<DriveItem> for DriveItem {
    fn serialize(item: DriveItem) -> io::Result<String> {
        let drive_item_string =
            serde_json::to_string(&item).expect("Error serializing DriveItem to String");
        Ok(drive_item_string)
    }

    fn serialize_to_file(path: &str, item: DriveItem) -> io::Result<()> {
        JsonFile::json_file(path, &item).expect("Error serializing DriveItem to file");
        Ok(())
    }

    fn deserialize_from_str(item_str: &str) -> io::Result<DriveItem> {
        let drive_item: DriveItem = serde_json::from_slice(item_str.as_bytes())
            .expect("Error demoralizing DriveItem from str");
        Ok(drive_item)
    }

    fn deserialize_from_file(path: &str) -> io::Result<DriveItem> {
        let drive_item: DriveItem =
            JsonFile::from_file(path).expect("Error demoralizing DriveItem from file");
        Ok(drive_item)
    }
}
