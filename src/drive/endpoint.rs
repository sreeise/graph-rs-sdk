use crate::drive::drive_item::driveitem::DriveItem;
use crate::drive::driveinfo::DriveInfo;
use crate::drive::item::Item;
use crate::drive::value;
use crate::drive::{Drive, DriveVersion, ItemResult, GRAPH_ENDPOINT, GRAPH_ENDPOINT_BETA};
use graph_error::{GraphError, GraphFailure};
use reqwest::header;
use std::convert::TryFrom;

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
    fn special_folder<T>(&mut self, folder_name: &str) -> ItemResult<T>
    where
        for<'de> T: serde::Deserialize<'de>;
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
    /// Get the drive info of a drive.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.drive());
    /// ```
    fn drive(&mut self) -> ItemResult<DriveInfo> {
        self.get(DriveEndPoint::Drive.url(self.version.as_str()).as_str())
    }

    /// Get the drive me DriveItem for the currently signed in user. Automatically
    /// provisions an accounts OneDrive if they do not have one.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.drive_me());
    /// ```
    fn drive_me(&mut self) -> ItemResult<DriveItem> {
        let v: value::Value =
            self.get(DriveEndPoint::DriveMe.url(self.version.as_str()).as_str())?;
        Ok(DriveItem::from(v))
    }

    /// Get the drives root folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.drive_root());
    /// ```
    fn drive_root(&mut self) -> ItemResult<DriveItem> {
        self.get(DriveEndPoint::DriveRoot.url(self.version.as_str()).as_str())
    }

    /// Get the drives root folder for the me endpoint. The me endpoint
    /// is the currently signed in user.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.drive_root_me());
    /// ```
    fn drive_root_me(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::DriveRootMe
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// Get the children of the drives root folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.drive_root_child());
    /// ```
    fn drive_root_child(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::DriveRootChild
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// Get drive changes.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.drive_changes());
    /// ```
    fn drive_changes(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::DriveChanges
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// Get drive items that have been shared with an account.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.shared_with_me());
    /// ```
    fn shared_with_me(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SharedWithMe
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// Get recent items for a drive.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.drive_recent());
    /// ```
    fn drive_recent(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::DriveRecent
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// Get recent activities for a drive. This API may be limited
    /// to specific accounts.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.drive_activities());
    /// ```
    fn drive_activities(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::DriveActivities
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// Get the children of the special documents folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.special_folder("documents"));
    /// ```
    fn special_folder<T>(&mut self, folder_name: &str) -> ItemResult<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        // To deserialize a more manual approach is needed here because the response
        // could either be a single value::Value or multiple value::Value's. DriveItem's
        // TryFrom impl does the work of checking for a value::Value from a Response.
        let mut endpoint = DriveEndPoint::SpecialFolder.to_string();
        endpoint.push('/');
        endpoint.push_str(folder_name);
        let mut response = self
            .client()?
            .get(endpoint.as_str())
            .bearer_auth(self.token())
            .header(header::CONTENT_TYPE, "application/json")
            .send()?;

        let status = response.status().as_u16();
        if GraphError::is_error(status) {
            return Err(GraphFailure::from(
                GraphError::try_from(status).unwrap_or_default(),
            ));
        }

        let item: T = response.json()?;
        Ok(item)
    }

    /// Get the special documents folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.special_documents());
    /// ```
    fn special_documents(&mut self) -> ItemResult<DriveItem> {
        let v: value::Value = self.get(
            DriveEndPoint::SpecialDocuments
                .url(self.version.as_str())
                .as_str(),
        )?;
        Ok(DriveItem::from(v))
    }

    /// Get the children of the special documents folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.special_documents_child());
    /// ```
    fn special_documents_child(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SpecialDocumentsChild
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// Get the special photos folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.special_photos());
    /// ```
    fn special_photos(&mut self) -> ItemResult<DriveItem> {
        let v: value::Value = self.get(
            DriveEndPoint::SpecialPhotos
                .url(self.version.as_str())
                .as_str(),
        )?;
        Ok(DriveItem::from(v))
    }

    /// Get the children of the special photos folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.special_photos_child());
    /// ```
    fn special_photos_child(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SpecialPhotosChild
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// Get the special camera roll folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.special_cameraroll());
    /// ```
    fn special_cameraroll(&mut self) -> ItemResult<DriveItem> {
        let v: value::Value = self.get(
            DriveEndPoint::SpecialCameraRoll
                .url(self.version.as_str())
                .as_str(),
        )?;
        Ok(DriveItem::from(v))
    }

    /// Get the children of the special camera roll folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.special_cameraroll_child());
    /// ```
    fn special_cameraroll_child(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SpecialCameraRollChild
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// Get the special approot folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.special_approot());
    /// ```
    fn special_approot(&mut self) -> ItemResult<DriveItem> {
        let v: value::Value = self.get(
            DriveEndPoint::SpecialAppRoot
                .url(self.version.as_str())
                .as_str(),
        )?;
        Ok(DriveItem::from(v))
    }

    /// Get the children of the special approot folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.special_approot_child());
    /// ```
    fn special_approot_child(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SpecialAppRootChild
                .url(self.version.as_str())
                .as_str(),
        )
    }

    /// Get the special music folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.special_music());
    /// ```
    fn special_music(&mut self) -> ItemResult<DriveItem> {
        let v: value::Value = self.get(
            DriveEndPoint::SpecialMusic
                .url(self.version.as_str())
                .as_str(),
        )?;
        Ok(DriveItem::from(v))
    }

    /// Get the children of the special music folder.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use rust_onedrive::prelude::Drive;
    /// # use rust_onedrive::drive::DriveVersion;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// println!("{:#?}", drive.special_music_child());
    /// ```
    fn special_music_child(&mut self) -> ItemResult<DriveItem> {
        self.get(
            DriveEndPoint::SpecialMusicChild
                .url(self.version.as_str())
                .as_str(),
        )
    }
}
