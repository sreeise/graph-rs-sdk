use crate::drive::drive_item::driveitem::DriveItem;
use crate::drive::driveinfo::DriveInfo;
use crate::drive::item::Item;
use crate::drive::{Drive, DriveVersion, ItemResult, GRAPH_ENDPOINT, GRAPH_ENDPOINT_BETA};

/// Implements well known or special folder paths.
///
/// # See
/// [Special Folders](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/drive_get_specialfolder?view=odsp-graph-online)
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
    DriveActivities,
    SpecialFolder,
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
            DriveEndPoint::DriveActivities => "/drive/activities",
            DriveEndPoint::SpecialFolder => "/me/drive/special",
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

    pub fn v1_url(self) -> String {
        let endpoint = self.as_str();
        let mut url = GRAPH_ENDPOINT.to_string();
        url.push_str(endpoint);
        url
    }

    pub fn url(self, host: &str) -> String {
        let endpoint = self.as_str();
        let mut url = String::from(host);
        url.push_str(endpoint);
        url
    }

    pub fn use_version(self, version: DriveVersion) -> String {
        match version {
            DriveVersion::V1 => self.v1_url(),
            DriveVersion::V2 => self.beta_url(),
        }
    }

    pub fn beta_url(self) -> String {
        let mut url = GRAPH_ENDPOINT_BETA.to_string();
        url.push_str(self.as_str());
        url
    }
}

impl From<DriveEndPoint> for String {
    fn from(dep: DriveEndPoint) -> Self {
        dep.v1_url()
    }
}

impl ToString for DriveEndPoint {
    fn to_string(&self) -> String {
        self.v1_url()
    }
}

pub trait EP {
    fn drive(&mut self) -> ItemResult<DriveInfo>;
    fn drive_me(&mut self) -> ItemResult<DriveItem>;
    fn drive_root(&mut self) -> ItemResult<DriveItem>;
    fn drive_root_me(&mut self) -> ItemResult<DriveItem>;
    fn drive_root_child(&mut self) -> ItemResult<DriveItem>;
    fn drive_changes(&mut self) -> ItemResult<DriveItem>;
    fn shared_with_me(&mut self) -> ItemResult<DriveItem>;
    fn drive_recent(&mut self) -> ItemResult<DriveItem>;
    fn drive_activities(&mut self) -> ItemResult<DriveItem>;
    fn special_folder(&mut self, folder_name: &str) -> ItemResult<DriveItem>;
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
    fn drive(&mut self) -> ItemResult<DriveInfo> {
        self.get(DriveEndPoint::Drive.url(self.version.as_str()).as_str())
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn drive_me(&mut self) -> ItemResult<DriveItem> {
        self.get(DriveEndPoint::DriveMe.url(self.version.as_str()).as_str())
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive_root(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn drive_root(&mut self) -> ItemResult<DriveItem> {
        self.get(DriveEndPoint::DriveRoot.url(self.version.as_str()).as_str())
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive_root_me(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn drive_root_me(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::DriveRootMe
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive_root_child(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn drive_root_child(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::DriveRootChild
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// # Example
    /// ```rust,ignore
    ///    fn shared_with_me(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn drive_changes(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::DriveChanges
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive_recent(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn shared_with_me(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SharedWithMe
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive_recent(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn drive_recent(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::DriveRecent
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive_activities(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn drive_activities(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::DriveActivities
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_folder(&mut self, folder_name: &str) -> ItemResult<DriveItem>
    ///    let drive_item: DriveItem = drive.special_folder("my_folder").unwrap();
    /// ```
    fn special_folder(&mut self, folder_name: &str) -> ItemResult<DriveItem> {
        let mut endpoint = DriveEndPoint::SpecialFolder.to_string();
        endpoint.push('/');
        endpoint.push_str(folder_name);
        self.get(endpoint.as_str())
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_documents(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_documents(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SpecialDocuments
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_documents_child(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_documents_child(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SpecialDocumentsChild
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_photos(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_photos(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SpecialPhotos
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_photos_child(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_photos_child(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SpecialPhotosChild
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_cameraroll(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_cameraroll(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SpecialCameraRoll
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_cameraroll_child(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_cameraroll_child(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SpecialCameraRollChild
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_approot(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_approot(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SpecialAppRoot
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_approot_child(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_approot_child(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SpecialAppRootChild
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// # Example
    /// ```rust,ignore
    ///     fn special_music(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_music(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SpecialMusic
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_music_child(&mut self) -> ItemResult<DriveItem>
    /// ```
    fn special_music_child(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SpecialMusicChild
                .url(self.version.as_str())
                .as_str(),
        )
    }
}
