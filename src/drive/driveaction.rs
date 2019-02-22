/// Enum for describing what action to take in a drive request
/// such as uploading and item or downloading an item.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DriveAction {
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

impl DriveAction {
    pub fn as_str(&self) -> &str {
        match *self {
            DriveAction::CheckIn => "checkin",
            DriveAction::CheckOut => "checkout",
            DriveAction::Copy => "copy",
            DriveAction::ListVersions => "versions",
            DriveAction::TrackChanges => "delta",
            DriveAction::Download | DriveAction::Upload => "content",
            DriveAction::CreateUploadSession => "createUploadSession",
            DriveAction::ListChildren | DriveAction::CreateFolder => "children",
            DriveAction::Preview => "preview",
            DriveAction::Activities => "activities",
            DriveAction::Move
            | DriveAction::GetItem
            | DriveAction::GetItemRoot
            | DriveAction::Delete => "",
        }
    }
}
