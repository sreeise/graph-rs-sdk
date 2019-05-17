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

/// Represents the types of file formats that a file can be
/// converted using
///
/// # Example
/// ```rust,ignore
/// # use rust_onedrive::prelude::*;
/// # use std::path::PathBuf;
///
/// let drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
/// let path_buf: PathBuf = drive.download_format("directory", &mut drive::Value, DownloadFormat::PDF).unwrap();
/// println!("{:#?}", path_buf);
/// ```
pub enum DownloadFormat {
    GLB,
    HTML,
    JPG,
    PDF,
}

impl AsRef<str> for DownloadFormat {
    fn as_ref(&self) -> &str {
        match self {
            DownloadFormat::GLB => "glb",
            DownloadFormat::HTML => "html",
            DownloadFormat::JPG => "jpg",
            DownloadFormat::PDF => "pdf",
        }
    }
}
