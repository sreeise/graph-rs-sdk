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

use crate::drive::base::driveinfo::DriveInfo;
use crate::drive::base::driveitem::DriveItem;
use crate::drive::base::value::Value;
use crate::drive::baseitem::BaseItem;
use crate::drive::GRAPH_ENDPOINT;

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

pub trait EP {
    fn drive(&mut self) -> BaseItem<DriveInfo>;
    fn drive_me(&mut self) -> BaseItem<DriveInfo>;
    fn drive_root(&mut self) -> BaseItem<Value>;
    fn drive_root_me(&mut self) -> BaseItem<Value>;
    fn drive_root_child(&mut self) -> BaseItem<DriveItem>;
    fn drive_changes(&mut self) -> BaseItem<DriveItem>;
    fn shared_with_me(&mut self) -> BaseItem<DriveItem>;
    fn drive_recent(&mut self) -> BaseItem<DriveItem>;
    fn special_documents(&mut self) -> BaseItem<Value>;
    fn special_documents_child(&mut self) -> BaseItem<DriveItem>;
    fn special_photos(&mut self) -> BaseItem<Value>;
    fn special_photos_child(&mut self) -> BaseItem<DriveItem>;
    fn special_cameraroll(&mut self) -> BaseItem<Value>;
    fn special_cameraroll_child(&mut self) -> BaseItem<DriveItem>;
    fn special_approot(&mut self) -> BaseItem<Value>;
    fn special_approot_child(&mut self) -> BaseItem<DriveItem>;
    fn special_music(&mut self) -> BaseItem<Value>;
    fn special_music_child(&mut self) -> BaseItem<DriveItem>;
}

impl DriveEndPoint {
    pub fn as_str(self) -> &'static str {
        match self {
            DriveEndPoint::Drive => "/drive",
            DriveEndPoint::DriveMe => "/me/drive",
            DriveEndPoint::DriveRoot => "/drive/root",
            DriveEndPoint::DriveRootMe => "/me/drive/root",
            DriveEndPoint::DriveRootChild => "/drive/root/children",
            DriveEndPoint::DriveChanges => "/drive/root/delta",
            DriveEndPoint::SharedWithMe => "/me/drive/sharedWithMe",
            DriveEndPoint::DriveRecent => "/me/drive/recent",
            DriveEndPoint::SpecialDocuments => "/me/drive/special/documents",
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

    pub fn as_url(self) -> String {
        let endpoint = self.as_str();
        let mut url = GRAPH_ENDPOINT.to_string();
        url.push_str(endpoint);
        url
    }

    pub fn build(endpoint: DriveEndPoint) -> String {
        let mut url = GRAPH_ENDPOINT.to_string();
        url.push_str(endpoint.as_str());
        url
    }
}
