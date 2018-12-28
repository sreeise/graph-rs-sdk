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

use std::io;

pub static GRAPH_ENDPOINT: &str = "https://graph.microsoft.com/v1.0";

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum DriveEndPoint {
    Drive,
    DriveMe,
    DriveRoot,
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

impl DriveEndPoint {
    pub fn as_str(&self) -> &str {
        match *self {
            DriveEndPoint::Drive => "/drive",
            DriveEndPoint::DriveMe => "/me/drive",
            DriveEndPoint::DriveRoot => "/me/drive/root",
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

#[cfg(test)]
mod endpoint_tests {
    use super::*;

    #[test]
    fn build_endpoint_test() {
        assert_eq!(
            DriveEndPoint::build(DriveEndPoint::Drive).expect("could not build drive endpoint"),
            "https://graph.microsoft.com/v1.0/drive"
        );
        assert_eq!(
            DriveEndPoint::build(DriveEndPoint::DriveMe).expect("could not build drive endpoint"),
            "https://graph.microsoft.com/v1.0/me/drive"
        );
        assert_eq!(
            DriveEndPoint::build(DriveEndPoint::DriveRoot).expect("could not build drive endpoint"),
            "https://graph.microsoft.com/v1.0/me/drive/root"
        );
        assert_eq!(
            DriveEndPoint::build(DriveEndPoint::DriveRootChild)
                .expect("could not build drive endpoint"),
            "https://graph.microsoft.com/v1.0/drive/root/children"
        );
        assert_eq!(
            DriveEndPoint::build(DriveEndPoint::SharedWithMe)
                .expect("could not build drive endpoint"),
            "https://graph.microsoft.com/v1.0/me/drive/sharedWithMe"
        );
        assert_eq!(
            DriveEndPoint::build(DriveEndPoint::DriveRecent)
                .expect("could not build drive endpoint"),
            "https://graph.microsoft.com/v1.0/me/drive/recent"
        );
        assert_eq!(
            DriveEndPoint::build(DriveEndPoint::SpecialDocuments)
                .expect("could not build drive endpoint"),
            "https://graph.microsoft.com/v1.0/me/drive/documents"
        );
        assert_eq!(
            DriveEndPoint::build(DriveEndPoint::SpecialDocumentsChild)
                .expect("could not build drive endpoint"),
            "https://graph.microsoft.com/v1.0/me/drive/special/documents/children"
        );
        assert_eq!(
            DriveEndPoint::build(DriveEndPoint::SpecialPhotos)
                .expect("could not build drive endpoint"),
            "https://graph.microsoft.com/v1.0/me/drive/special/photos"
        );
        assert_eq!(
            DriveEndPoint::build(DriveEndPoint::SpecialPhotosChild)
                .expect("could not build drive endpoint"),
            "https://graph.microsoft.com/v1.0/me/special/photos/children"
        );
        assert_eq!(
            DriveEndPoint::build(DriveEndPoint::SpecialCameraRoll)
                .expect("could not build drive endpoint"),
            "https://graph.microsoft.com/v1.0/me/drive/special/cameraroll"
        );
        assert_eq!(
            DriveEndPoint::build(DriveEndPoint::SpecialCameraRollChild)
                .expect("could not build drive endpoint"),
            "https://graph.microsoft.com/v1.0/me/drive/special/cameraroll/children"
        );
        assert_eq!(
            DriveEndPoint::build(DriveEndPoint::SpecialAppRoot)
                .expect("could not build drive endpoint"),
            "https://graph.microsoft.com/v1.0/me/drive/special/approot"
        );
        assert_eq!(
            DriveEndPoint::build(DriveEndPoint::SpecialAppRootChild)
                .expect("could not build drive endpoint"),
            "https://graph.microsoft.com/v1.0/me/drive/special/approot/children"
        );
        assert_eq!(
            DriveEndPoint::build(DriveEndPoint::SpecialMusic)
                .expect("could not build drive endpoint"),
            "https://graph.microsoft.com/v1.0/me/drive/special/music"
        );
        assert_eq!(
            DriveEndPoint::build(DriveEndPoint::SpecialMusicChild)
                .expect("could not build drive endpoint"),
            "https://graph.microsoft.com/v1.0/me/drive/special/music/children"
        );
    }
}
