use crate::drive::parentreference::ParentReference;
use crate::drive::DriveResource;
use crate::drive::ItemResult;

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

// Used for copy events.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct ParentReferenceCopy {
    parent_reference: ParentReference,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl ParentReferenceCopy {
    pub fn new(pr: ParentReference, name: Option<String>) -> ParentReferenceCopy {
        ParentReferenceCopy {
            parent_reference: pr,
            name,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DriveItemCopy {
    parent_reference: ParentReference,
    name: Option<String>,
    drive_resource: DriveResource,
}

impl DriveItemCopy {
    pub fn new(
        parent_reference: ParentReference,
        name: Option<String>,
        drive_resource: DriveResource,
    ) -> DriveItemCopy {
        println!("{:#?}", &parent_reference);
        DriveItemCopy {
            parent_reference,
            name,
            drive_resource,
        }
    }

    pub fn drive_resource(&self) -> DriveResource {
        self.drive_resource
    }

    pub fn as_json(&self) -> ItemResult<String> {
        if let Some(name) = &self.name {
            let prc = ParentReferenceCopy::new(self.parent_reference.clone(), Some(name.clone()));
            let s = serde_json::to_string_pretty(&prc)?;
            Ok(s)
        } else {
            let prc = ParentReferenceCopy::new(self.parent_reference.clone(), None);
            let s = serde_json::to_string_pretty(&prc)?;
            Ok(s)
        }
    }
}

/// The progress status of long running events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventProgress {
    pub operation: Option<String>,
    #[serde(rename = "percentageComplete")]
    pub percentage_complete: f64,
    #[serde(rename = "resourceId")]
    resource_id: Option<String>,
    pub status: String,
}

impl EventProgress {
    #[allow(dead_code)]
    pub fn resource_id(&self) -> Option<String> {
        self.resource_id.clone()
    }

    pub fn is_completed(&self) -> bool {
        self.status.as_str() == "completed"
    }
}
