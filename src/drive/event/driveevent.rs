/// Enum for describing what action to take in a drive request
/// such as uploading and item or downloading an item.
///
/// These names are defined by their URL path for API calls.
///
/// # See Also:
/// [Documentation on Drive Items and API Events](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/driveitem?view=odsp-graph-online#methods)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum DriveEvent {
    CheckIn,
    CheckOut,
    Copy,
    CreateFolder,
    Delete,
    Download,
    DownloadAndFormat,
    GetItem,
    GetItemRoot,
    ListChildren,
    Move,
    Upload,
    CreateUploadSession,
    ListVersions,
    TrackChanges,
    Preview,
    Activities,
}

impl DriveEvent {
    pub fn as_str(&self) -> &str {
        match *self {
            DriveEvent::CheckIn => "checkin",
            DriveEvent::CheckOut => "checkout",
            DriveEvent::Copy => "copy",
            DriveEvent::ListVersions => "versions",
            DriveEvent::TrackChanges => "delta",
            DriveEvent::Download | DriveEvent::Upload => "content",
            DriveEvent::DownloadAndFormat => "content?format=",
            DriveEvent::CreateUploadSession => "createUploadSession",
            DriveEvent::ListChildren | DriveEvent::CreateFolder => "children",
            DriveEvent::Preview => "preview",
            DriveEvent::Activities => "activities",
            DriveEvent::Move |
            DriveEvent::GetItem |
            DriveEvent::GetItemRoot |
            DriveEvent::Delete => "",
        }
    }
}
