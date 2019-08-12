use crate::drive::drive_item::collection::Collection;
use crate::drive::drive_item::driveitem::DriveItem;
use crate::drive::drive_item::itemactivity::ItemActivity;
use crate::drive::driveinfo::DriveInfo;
use crate::drive::driveurl::MutateUrl;
use crate::drive::item::SelectResource;
use crate::drive::{DriveVersion, Request, GRAPH_ENDPOINT, GRAPH_ENDPOINT_BETA};

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
    DriveRootMeChild,
    DriveRootChild,
    DriveChanges,
    SharedWithMe,
    DriveRecent,
    DriveActivities,
    DriveActivitiesMe,
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
            DriveEndPoint::DriveRootMeChild => "me/drive/root/children",
            DriveEndPoint::DriveRootChild => "/drive/root/children",
            DriveEndPoint::DriveChanges => "/drive/root/delta",
            DriveEndPoint::SharedWithMe => "/me/drive/sharedWithMe",
            DriveEndPoint::DriveRecent => "/me/drive/recent",
            DriveEndPoint::DriveActivities => "/drive/activities",
            DriveEndPoint::DriveActivitiesMe => "/me/drive/activities",
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
    fn drive(&mut self) -> Request<DriveInfo>;
    fn drive_me(&mut self) -> Request<DriveInfo>;
    fn drive_root(&mut self) -> Request<DriveItem>;
    fn drive_root_me(&mut self) -> Request<DriveItem>;
    fn drive_root_child(&mut self) -> Request<Collection<DriveItem>>;
    fn drive_root_me_child(&mut self) -> Request<Collection<DriveItem>>;
    fn delta(&mut self) -> Request<Collection<DriveItem>>;
    fn shared_with_me(&mut self) -> Request<Collection<DriveItem>>;
    fn drive_recent(&mut self) -> Request<Collection<DriveItem>>;
    fn drive_activities(&mut self, drive_id: &str) -> Request<Collection<ItemActivity>>;
    fn drive_activities_me(&mut self) -> Request<Collection<ItemActivity>>;
    fn special_folder<T>(&mut self, folder_name: &str) -> Request<T>
    where
        for<'de> T: serde::Deserialize<'de>;
    fn special_documents(&mut self) -> Request<Collection<DriveItem>>;
    fn special_documents_child(&mut self) -> Request<Collection<DriveItem>>;
    fn special_photos(&mut self) -> Request<Collection<DriveItem>>;
    fn special_photos_child(&mut self) -> Request<Collection<DriveItem>>;
    fn special_cameraroll(&mut self) -> Request<Collection<DriveItem>>;
    fn special_cameraroll_child(&mut self) -> Request<Collection<DriveItem>>;
    fn special_approot(&mut self) -> Request<Collection<DriveItem>>;
    fn special_approot_child(&mut self) -> Request<Collection<DriveItem>>;
    fn special_music(&mut self) -> Request<Collection<DriveItem>>;
    fn special_music_child(&mut self) -> Request<Collection<DriveItem>>;
}

impl EP for SelectResource {
    /// Get the drive info of a drive.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().drive();
    /// let collection: DriveInfo = req.send().unwrap();
    /// ```
    fn drive(&mut self) -> Request<DriveInfo> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::Drive);
        Request::from(&req)
    }

    /// Get the drive me DriveItem for the currently signed in user. Automatically
    /// provisions an accounts OneDrive if they do not have one.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().drive_me();
    /// let collection: DriveInfo = req.send().unwrap();
    /// ```
    fn drive_me(&mut self) -> Request<DriveInfo> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::DriveMe);
        Request::from(&req)
    }

    /// Get the drives root folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().drive_root();
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn drive_root(&mut self) -> Request<DriveItem> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::DriveRoot);
        Request::from(&req)
    }

    /// Get the drives root folder for the me endpoint. The me endpoint
    /// is the currently signed in user.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().drive_root_me();
    /// let drive_item: DriveItem = req.send().unwrap();
    /// ```
    fn drive_root_me(&mut self) -> Request<DriveItem> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::DriveRootMe);
        Request::from(&req)
    }

    /// Get the children of the drives root folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().drive_root_child();
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn drive_root_child(&mut self) -> Request<Collection<DriveItem>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::DriveRootChild);
        Request::from(&req)
    }

    /// Get the children of the drives root folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().drive_root_me_child();
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn drive_root_me_child(&mut self) -> Request<Collection<DriveItem>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::DriveRootMeChild);
        Request::from(&req)
    }

    /// Get drive changes.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().delta();
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn delta(&mut self) -> Request<Collection<DriveItem>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::DriveChanges);
        Request::from(&req)
    }

    /// Get drive items that have been shared with an account.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().shared_with_me();
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn shared_with_me(&mut self) -> Request<Collection<DriveItem>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::SharedWithMe);
        Request::from(&req)
    }

    /// Get recent items for a drive.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().drive_recent();
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn drive_recent(&mut self) -> Request<Collection<DriveItem>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::DriveRecent);
        Request::from(&req)
    }

    /// Get recent activities for a drive. This API may be limited
    /// to specific accounts.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().drive_activities();
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn drive_activities(&mut self, drive_id: &str) -> Request<Collection<ItemActivity>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.as_mut()
            .extend_path(&["drives", drive_id, "activities"]);
        Request::from(&req)
    }

    fn drive_activities_me(&mut self) -> Request<Collection<ItemActivity>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::DriveActivitiesMe);
        Request::from(&req)
    }

    /// Get the children of the special documents folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req: BoxItem<DriveItemCollection> = drive.v1().special_folder("folder_name");
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn special_folder<T>(&mut self, folder_name: &str) -> Request<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::SpecialFolder);
        req.as_mut().extend_path(&[folder_name]);
        Request::from(&req)
    }

    /// Get the special documents folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().special_documents();
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn special_documents(&mut self) -> Request<Collection<DriveItem>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::SpecialDocuments);
        Request::from(&req)
    }

    /// Get the children of the special documents folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().special_documents_child();
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn special_documents_child(&mut self) -> Request<Collection<DriveItem>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::SpecialDocumentsChild);
        Request::from(&req)
    }

    /// Get the special photos folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().special_photos();
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn special_photos(&mut self) -> Request<Collection<DriveItem>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::SpecialPhotos);
        Request::from(&req)
    }

    /// Get the children of the special photos folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().special_photos_child();
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn special_photos_child(&mut self) -> Request<Collection<DriveItem>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::SpecialPhotosChild);
        Request::from(&req)
    }

    /// Get the special camera roll folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().special_cameraroll();
    /// let collection: DCollection<DriveItem> = req.send().unwrap();
    /// ```
    fn special_cameraroll(&mut self) -> Request<Collection<DriveItem>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::SpecialCameraRoll);
        Request::from(&req)
    }

    /// Get the children of the special camera roll folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().special_cameraroll_child();
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn special_cameraroll_child(&mut self) -> Request<Collection<DriveItem>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::SpecialCameraRollChild);
        Request::from(&req)
    }

    /// Get the special approot folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().special_approot();
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn special_approot(&mut self) -> Request<Collection<DriveItem>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::SpecialAppRoot);
        Request::from(&req)
    }

    /// Get the children of the special approot folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().special_approot_child();
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn special_approot_child(&mut self) -> Request<Collection<DriveItem>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::SpecialAppRoot);
        Request::from(&req)
    }

    /// Get the special music folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().special_music();
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn special_music(&mut self) -> Request<Collection<DriveItem>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::SpecialMusic);
        Request::from(&req)
    }

    /// Get the children of the special music folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN");
    /// let mut req = drive.v1().special_music_child();
    /// let collection: Collection<DriveItem> = req.send().unwrap();
    /// ```
    fn special_music_child(&mut self) -> Request<Collection<DriveItem>> {
        let mut req = self.get();
        req.content_type("application/json");
        req.endpoint(DriveEndPoint::SpecialMusicChild);
        Request::from(&req)
    }
}
