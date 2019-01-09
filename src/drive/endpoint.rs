/*
Implements the base drive endpoints including special folders shown below.

Special folders are one of:
    1. Documents:
        Name: documents
        Description: The Documents folder.
    2. Photos
        Name: photos
        Description: The Photos folder.
    3. Camera Roll:
        Name: cameraroll
        Description: The Camera Roll Backup folder.
    4. App Root:
        Name: approot
        Description: The application's personal folder. Usually in /Apps/{Application Name}
    5. Music:
        Name: music
        Description: The Music folder.
*/

use crate::drive::driveitem::Value;
use crate::drive::driveitem::{DriveInfo, DriveItem};
use crate::drive::GRAPH_ENDPOINT;
use std::io;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum DriveEndPoint {
    Drive,
    DriveMe,
    DriveRoot,
    DriveRootMe,
    DriveRootChild,
    DriveChanges,
    SharedWithMe,
    DriveRecent,
    SpecialDocuments,
    SpecialDocumentsChild,
    SpecialPhotos,
    SpecialPhotosChild,
    SpecialCameraRoll,
    SpecialCameraRollChild,
    SpecialAppRoot,
    SpecialAppRootChild,
    SpecialMusic,
    SpecialMusicChild,
}

// TODO: Implement the rest of DriveEndPoint for trait EP here and in drive/mod.rs
// Tests will be needed as well.
pub trait EP {
    fn req_to_string(&mut self, endpoint: DriveEndPoint) -> String;
    fn drive(&mut self) -> DriveInfo;
    fn drive_me(&mut self) -> DriveInfo;
    fn drive_root(&mut self) -> Value;
    fn drive_root_me(&mut self) -> Value;
    fn drive_root_child(&mut self) -> DriveItem;
    fn drive_changes(&mut self) -> DriveItem;
    fn shared_with_me(&mut self) -> DriveItem;
}

impl DriveEndPoint {
    pub fn as_str(&self) -> &str {
        match *self {
            DriveEndPoint::Drive => "/drive",
            DriveEndPoint::DriveMe => "/me/drive",
            DriveEndPoint::DriveRoot => "/drive/root",
            DriveEndPoint::DriveRootMe => "/me/drive/root",
            DriveEndPoint::DriveRootChild => "/drive/root/children",
            DriveEndPoint::DriveChanges => "/drive/root/delta",
            DriveEndPoint::SharedWithMe => "/me/drive/sharedWithMe",
            DriveEndPoint::DriveRecent => "/me/drive/recent",
            DriveEndPoint::SpecialDocuments => "/me/drive/documents",
            DriveEndPoint::SpecialDocumentsChild => "/me/drive/special/documents/children",
            DriveEndPoint::SpecialPhotos => "/me/drive/special/photos",
            DriveEndPoint::SpecialPhotosChild => "/me/special/photos/children",
            DriveEndPoint::SpecialCameraRoll => "/me/drive/special/cameraroll",
            DriveEndPoint::SpecialCameraRollChild => "/me/drive/special/cameraroll/children",
            DriveEndPoint::SpecialAppRoot => "/me/drive/special/approot",
            DriveEndPoint::SpecialAppRootChild => "/me/drive/special/approot/children",
            DriveEndPoint::SpecialMusic => "/me/drive/special/music",
            DriveEndPoint::SpecialMusicChild => "/me/drive/special/music/children",
        }
    }

    pub fn build(endpoint: DriveEndPoint) -> io::Result<String> {
        let mut url = GRAPH_ENDPOINT.to_string();
        url.push_str(endpoint.as_str());
        Ok(url)
    }
}

// TODO: Implement methods for request status in header or
// possibly return header information on a request that fails?
pub enum ReqError {
    BadRequest,
}

impl ReqError {
    pub fn as_str(&self) -> &str {
        match *self {
            ReqError::BadRequest => {
                "Error: either the request did not succeed or the request could not be parsed"
            }
        }
    }
}
