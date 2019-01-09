use crate::drive::DriveSerDe;
use crate::process::jsonio::JsonFile;
use std::io;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveInfo {
    #[serde(rename = "@odata.context")]
    _odata_context: Option<String>,
    #[serde(rename = "createdDateTime")]
    created_date_time: Option<String>,
    description: Option<String>,
    id: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    last_modified_date_time: Option<String>,
    name: Option<String>,
    #[serde(rename = "webUrl")]
    web_url: Option<String>,
    #[serde(rename = "driveType")]
    drive_type: Option<String>,
    #[serde(rename = "createdBy")]
    created_by: Option<CreatedBy>,
    #[serde(rename = "lastModifiedBy")]
    last_modified_by: Option<LastModifiedBy>,
    owner: Option<Owner>,
    quota: Option<Quota>,
}

impl DriveInfo {
    pub fn new(
        _odata_context: Option<String>,
        created_date_time: Option<String>,
        description: Option<String>,
        id: Option<String>,
        last_modified_date_time: Option<String>,
        name: Option<String>,
        web_url: Option<String>,
        drive_type: Option<String>,
        created_by: Option<CreatedBy>,
        last_modified_by: Option<LastModifiedBy>,
        owner: Option<Owner>,
        quota: Option<Quota>,
    ) -> Self {
        DriveInfo {
            _odata_context,
            created_date_time,
            description,
            id,
            last_modified_date_time,
            name,
            web_url,
            drive_type,
            created_by,
            last_modified_by,
            owner,
            quota,
        }
    }

    pub fn data_context(&self) -> Option<String> {
        self._odata_context.clone()
    }

    pub fn created_date_time(&self) -> Option<String> {
        self.created_date_time.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn last_modified_date_time(&self) -> Option<String> {
        self.last_modified_date_time.clone()
    }

    pub fn web_url(&self) -> Option<String> {
        self.web_url.clone()
    }

    pub fn created_by(&self) -> Option<CreatedBy> {
        self.created_by.clone()
    }

    pub fn drive_type(&self) -> Option<String> {
        self.drive_type.clone()
    }

    pub fn owner(&self) -> Option<Owner> {
        self.owner.clone()
    }

    pub fn quota(&self) -> Option<Quota> {
        self.quota.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quota {
    deleted: i64,
    remaining: i64,
    state: String,
    total: i64,
    used: i64,
}

impl Quota {
    pub fn new(deleted: i64, remaining: i64, state: String, total: i64, used: i64) -> Self {
        Quota {
            deleted,
            remaining,
            state,
            total,
            used,
        }
    }

    pub fn deleted(&self) -> i64 {
        self.deleted
    }

    pub fn remaining(&self) -> i64 {
        self.remaining
    }

    pub fn state(&self) -> String {
        self.state.clone()
    }

    pub fn total(&self) -> i64 {
        self.total
    }

    pub fn used(&self) -> i64 {
        self.used
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Owner {
    user: Option<User>,
    group: Option<Group>,
}

impl Owner {
    pub fn new(user: Option<User>, group: Option<Group>) -> Self {
        Owner { user, group }
    }

    pub fn user(&self) -> Option<User> {
        self.user.clone()
    }

    pub fn group(&self) -> Option<Group> {
        self.group.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    id: Option<String>,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveItem {
    #[serde(rename = "@odata.context")]
    _odata_context: Option<String>,
    #[serde(rename = "@odata.nextLink")]
    _odata_next_link: Option<String>,
    value: Option<Vec<Value>>,
}

impl DriveItem {
    pub fn new(
        data_context: Option<String>,
        next_link: Option<String>,
        value: Option<Vec<Value>>,
    ) -> DriveItem {
        DriveItem {
            _odata_context: data_context,
            _odata_next_link: next_link,
            value,
        }
    }

    pub fn data_context(&self) -> Option<String> {
        self._odata_context.clone()
    }

    pub fn value(&self) -> Option<Vec<Value>> {
        self.value.clone()
    }

    pub fn value_idx(&self, idx: usize) -> Value {
        let value = self
            .value
            .to_owned()
            .expect("Could not get Value struct from DriveItem");
        value[idx].clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Value {
    #[serde(rename = "@odata.context")]
    _odata_context: Option<String>,
    #[serde(rename = "@odata.type")]
    _odata_type: Option<String>,
    #[serde(rename = "createdDateTime")]
    created_date_time: String,
    #[serde(rename = "cTag")]
    c_tag: Option<String>,
    #[serde(rename = "eTag")]
    e_tag: Option<String>,
    id: String,
    #[serde(rename = "lastModifiedDateTime")]
    last_modified_date_time: String,
    name: String,
    size: i64,
    #[serde(rename = "webUrl")]
    web_url: String,
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
        created_date_time: String,
        c_tag: Option<String>,
        e_tag: Option<String>,
        id: String,
        last_modified_date_time: String,
        name: String,
        size: i64,
        web_url: String,
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

    pub fn created_date_time(&self) -> String {
        self.created_date_time.clone()
    }

    pub fn c_tag(&self) -> Option<String> {
        self.c_tag.clone()
    }

    pub fn e_tag(&self) -> Option<String> {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatedBy {
    user: Option<User>,
    application: Option<Application>,
}

impl CreatedBy {
    pub fn new(user: Option<User>, application: Option<Application>) -> CreatedBy {
        CreatedBy { user, application }
    }

    pub fn user(&self) -> Option<User> {
        self.user.clone()
    }

    pub fn application(&self) -> Option<Application> {
        self.application.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    id: Option<String>,
    email: Option<String>,
}

impl User {
    pub fn new(display_name: Option<String>, id: Option<String>, email: Option<String>) -> Self {
        User {
            display_name,
            id,
            email,
        }
    }

    pub fn display_name(&self) -> Option<String> {
        self.display_name.clone()
    }

    pub fn id(&self) -> Option<String> {
        self.display_name.clone()
    }

    pub fn email(&self) -> Option<String> {
        self.email.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    id: String,
}

impl Application {
    pub fn new(display_name: Option<String>, id: String) -> Self {
        Application { display_name, id }
    }

    pub fn display_name(&self) -> Option<String> {
        self.display_name.clone()
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastModifiedBy {
    user: Option<User>,
    application: Option<Application>,
}

impl LastModifiedBy {
    pub fn new(user: Option<User>, application: Option<Application>) -> Self {
        LastModifiedBy { user, application }
    }

    pub fn user(&self) -> Option<User> {
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
    id: Option<String>,
    path: Option<String>,
}

impl ParentReference {
    pub fn new(
        drive_id: String,
        drive_type: String,
        id: Option<String>,
        path: Option<String>,
    ) -> ParentReference {
        ParentReference {
            drive_id,
            drive_type,
            id,
            path,
        }
    }

    pub fn drive_id(&self) -> String {
        self.drive_id.clone()
    }

    pub fn drive_type(&self) -> String {
        self.drive_type.clone()
    }

    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn path(&self) -> Option<String> {
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
    pub fn new(created_date_time: String, last_modified_date_time: String) -> Self {
        FileSystemInfo {
            created_date_time,
            last_modified_date_time,
        }
    }
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
    view: Option<View>,
}

impl Folder {
    pub fn new(child_count: i64, view: Option<View>) -> Self {
        Folder { child_count, view }
    }
}

impl Folder {
    pub fn child_count(&self) -> i64 {
        self.child_count.clone()
    }

    pub fn view(&self) -> Option<View> {
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
    pub fn new(view_type: String, sort_by: String, sort_order: String) -> Self {
        View {
            view_type,
            sort_by,
            sort_order,
        }
    }
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
    pub fn new(name: String) -> Self {
        SpecialFolder { name }
    }
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
    hashes: Option<Hashes>,
}

impl File {
    pub fn new(mime_type: String, hashes: Option<Hashes>) -> Self {
        File { mime_type, hashes }
    }
}

impl File {
    pub fn mime_type(&self) -> String {
        self.mime_type.clone()
    }

    pub fn hashes(&self) -> Option<Hashes> {
        self.hashes.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hashes {
    #[serde(rename = "sha1Hash")]
    sha1_hash: Option<String>,
    #[serde(rename = "quickXorHash")]
    quick_xor_hash: Option<String>,
    #[serde(rename = "crc32Hash")]
    crc32_hash: Option<String>,
}

impl Hashes {
    pub fn new(
        sha1_hash: Option<String>,
        quick_xor_hash: Option<String>,
        crc32_hash: Option<String>,
    ) -> Self {
        Hashes {
            sha1_hash,
            quick_xor_hash,
            crc32_hash,
        }
    }
}

impl Hashes {
    pub fn sha1_hash(&self) -> Option<String> {
        self.sha1_hash.clone()
    }

    pub fn quick_xor_hash(&self) -> Option<String> {
        self.quick_xor_hash.clone()
    }

    pub fn crc32_hash(&self) -> Option<String> {
        self.crc32_hash.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoteItem {
    #[serde(rename = "createdDateTime")]
    created_date_time: String,
    id: String,
    #[serde(rename = "lastModifiedDateTime")]
    last_modified_date_time: String,
    name: String,
    size: i64,
    #[serde(rename = "webDavUrl")]
    web_dav_url: String,
    #[serde(rename = "webUrl")]
    web_url: String,
    #[serde(rename = "createdBy")]
    created_by: Option<CreatedBy>,
    file: Option<File>,
    #[serde(rename = "fileSystemInfo")]
    file_system_info: Option<FileSystemInfo>,
    #[serde(rename = "lastModifiedBy")]
    last_modified_by: Option<LastModifiedBy>,
    #[serde(rename = "parentReference")]
    parent_reference: Option<ParentReference>,
    shared: Option<Shared>,
    #[serde(rename = "sharepointIds")]
    share_point_ids: Option<SharePointIds>,
}

impl RemoteItem {
    pub fn new(
        created_date_time: String,
        id: String,
        last_modified_date_time: String,
        name: String,
        size: i64,
        web_dav_url: String,
        web_url: String,
        created_by: Option<CreatedBy>,
        file: Option<File>,
        file_system_info: Option<FileSystemInfo>,
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
            last_modified_by,
            parent_reference,
            shared,
            share_point_ids,
        }
    }
}

impl RemoteItem {
    pub fn created_date_time(&self) -> String {
        self.created_date_time.clone()
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
        self.size
    }

    pub fn web_url(&self) -> String {
        self.web_url.clone()
    }

    pub fn web_dav_url(&self) -> String {
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

    pub fn shared(&self) -> Option<Shared> {
        self.shared.clone()
    }

    pub fn share_point_ids(&self) -> Option<SharePointIds> {
        self.share_point_ids.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shared {
    scope: Option<String>,
    #[serde(rename = "sharedDateTime")]
    shared_date_time: Option<String>,
    #[serde(rename = "sharedBy")]
    shared_by: Option<SharedBy>,
}

impl Shared {
    pub fn new(
        scope: Option<String>,
        shared_date_time: Option<String>,
        shared_by: Option<SharedBy>,
    ) -> Self {
        Shared {
            scope,
            shared_date_time,
            shared_by,
        }
    }
}

impl Shared {
    pub fn scope(&self) -> Option<String> {
        self.scope.clone()
    }

    pub fn shared_date_time(&self) -> Option<String> {
        self.shared_date_time.clone()
    }

    pub fn shared_by(&self) -> Option<SharedBy> {
        self.shared_by.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharePointIds {
    #[serde(rename = "listId")]
    list_id: Option<String>,
    #[serde(rename = "listItemId")]
    list_item_id: Option<String>,
    #[serde(rename = "listItemUniqueId")]
    list_item_unique_id: Option<String>,
    #[serde(rename = "siteId")]
    site_id: Option<String>,
    #[serde(rename = "siteUrl")]
    site_url: Option<String>,
    #[serde(rename = "webId")]
    web_id: Option<String>,
}

impl SharePointIds {
    pub fn new(
        list_id: Option<String>,
        list_item_id: Option<String>,
        list_item_unique_id: Option<String>,
        site_id: Option<String>,
        site_url: Option<String>,
        web_id: Option<String>,
    ) -> Self {
        SharePointIds {
            list_id,
            list_item_id,
            list_item_unique_id,
            site_id,
            site_url,
            web_id,
        }
    }
}

impl SharePointIds {
    pub fn list_id(&self) -> Option<String> {
        self.list_id.clone()
    }

    pub fn list_item_id(&self) -> Option<String> {
        self.list_item_id.clone()
    }

    pub fn list_item_unique_id(&self) -> Option<String> {
        self.list_item_unique_id.clone()
    }

    pub fn site_id(&self) -> Option<String> {
        self.site_id.clone()
    }

    pub fn site_url(&self) -> Option<String> {
        self.site_url.clone()
    }

    pub fn web_id(&self) -> Option<String> {
        self.web_id.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedBy {
    user: Option<User>,
}

impl SharedBy {
    pub fn new(user: Option<User>) -> Self {
        SharedBy { user }
    }
}

impl SharedBy {
    pub fn user(&self) -> Option<User> {
        self.user.clone()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    height: Option<i64>,
    width: Option<i64>,
}

impl Image {
    pub fn new(height: Option<i64>, width: Option<i64>) -> Self {
        Image { height, width }
    }
}

impl Image {
    pub fn height(&self) -> Option<i64> {
        self.height
    }

    pub fn width(&self) -> Option<i64> {
        self.width
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Photo {
    #[serde(rename = "takenDateTime")]
    taken_date_time: String,
}

impl Photo {
    pub fn new(taken_date_time: String) -> Self {
        Photo { taken_date_time }
    }
}

impl Photo {
    pub fn taken_date_time(&self) -> String {
        self.taken_date_time.clone()
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
            .expect("Could not deserialize DriveItem from file");
        Ok(drive_item)
    }

    fn deserialize_from_file(path: &str) -> io::Result<DriveItem> {
        let drive_item: DriveItem =
            JsonFile::from_file(path).expect("Could not deserialize DriveItem from file");
        Ok(drive_item)
    }
}
