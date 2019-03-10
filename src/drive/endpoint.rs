/*
Implements the drive_item drive endpoints including special folders shown below.

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

use crate::drive::drive_item::driveitem::DriveItem;

use crate::drive::item::Item;
use crate::drive::{Drive, ItemResult, GRAPH_ENDPOINT};

/*
pub struct DI<U: Item<U>>(Result<U, RequestError>);
pub type ItemResult<T> = DI<T>;
*/

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

pub trait EP: Item {
    fn drive(&mut self) -> ItemResult<DriveItem>;
    fn drive_me(&mut self) -> ItemResult<DriveItem>;
    fn drive_root(&mut self) -> ItemResult<DriveItem>;
    fn drive_root_me(&mut self) -> ItemResult<DriveItem>;
    fn drive_root_child(&mut self) -> ItemResult<DriveItem>;
    fn drive_changes(&mut self) -> ItemResult<DriveItem>;
    fn shared_with_me(&mut self) -> ItemResult<DriveItem>;
    fn drive_recent(&mut self) -> ItemResult<DriveItem>;
    fn special_documents(&mut self) -> ItemResult<DriveItem>;
    fn special_documents_child(&mut self) -> ItemResult<DriveItem>;
    fn special_photos(&mut self) -> ItemResult<DriveItem>;
    fn special_photos_child(&mut self) -> ItemResult<DriveItem>;
    fn special_cameraroll(&mut self) -> ItemResult<DriveItem>;
    fn special_cameraroll_child(&mut self) -> ItemResult<DriveItem>;
    fn special_approot(&mut self) -> ItemResult<DriveItem>;
    fn special_approot_child(&mut self) -> ItemResult<DriveItem>;
    fn special_music(&mut self) -> ItemResult<DriveItem>;
    fn special_music_child(&mut self) -> ItemResult<DriveItem>;
}

/// Automatically requests the DriveEndPoint given in the function name and returns the struct
/// of that request. The structs may be of different types listed here by function name:
impl EP for Drive {
    /// # Example
    /// ```rust,ignore
    ///    fn drive_me(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn drive(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::Drive)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn drive_me(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::DriveMe)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive_root(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn drive_root(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::DriveRoot)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive_root_me(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn drive_root_me(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::DriveRootMe)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive_root_child(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn drive_root_child(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::DriveRootChild)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn shared_with_me(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn drive_changes(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::DriveChanges)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive_recent(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn shared_with_me(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::SharedWithMe)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive_recent(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn drive_recent(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::DriveRecent)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_documents(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_documents(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::SpecialDocuments)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_documents_child(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_documents_child(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::SpecialDocumentsChild)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_photos(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_photos(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::SpecialPhotos)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_photos_child(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_photos_child(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::SpecialPhotosChild)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_cameraroll(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_cameraroll(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::SpecialCameraRoll)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_cameraroll_child(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_cameraroll_child(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::SpecialCameraRollChild)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_approot(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_approot(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::SpecialAppRoot)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_approot_child(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_approot_child(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::SpecialAppRootChild)
    }

    /// # Example
    /// ```rust,ignore
    ///     fn special_music(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_music(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::SpecialMusic)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_music_child(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_music_child(&mut self) -> ItemResult<DriveItem> {
        self.drive_item(DriveEndPoint::SpecialMusicChild)
    }
}
