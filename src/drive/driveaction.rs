/// Enum for describing what action to take in a drive request
/// such as uploading and item or downloading an item.
/// TODO: This will most likely change to use traits instead of an enum.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DriveEvent {
    CheckIn,
    CheckOut,
    Copy,
    CreateFolder,
    Delete,
    Download,
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
