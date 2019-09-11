use crate::client::Ident;
use crate::url::GraphUrl;
use crate::{GRAPH_URL, GRAPH_URL_BETA};
use url::Url;

/// Implements well known or special folder paths.
///
/// # See
/// [Special Folders](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/drive_get_specialfolder?view=odsp-graph-online)
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum DriveEndPoint {
    Drive,
    DriveRoot,
    DriveRootChild,
    Delta,
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
            DriveEndPoint::Drive => "drive",
            DriveEndPoint::DriveRoot => "root",
            DriveEndPoint::DriveRootChild => "root/children",
            DriveEndPoint::Delta => "root/delta",
            DriveEndPoint::SharedWithMe => "sharedWithMe",
            DriveEndPoint::DriveRecent => "recent",
            DriveEndPoint::DriveActivities => "activities",
            DriveEndPoint::SpecialFolder => "special",
            DriveEndPoint::SpecialDocuments => "special/documents",
            DriveEndPoint::SpecialDocumentsChild => "special/documents/children",
            DriveEndPoint::SpecialPhotos => "special/photos",
            DriveEndPoint::SpecialPhotosChild => "special/photos/children",
            DriveEndPoint::SpecialCameraRoll => "special/cameraroll",
            DriveEndPoint::SpecialCameraRollChild => "special/cameraroll/children",
            DriveEndPoint::SpecialAppRoot => "special/approot",
            DriveEndPoint::SpecialAppRootChild => "special/approot/children",
            DriveEndPoint::SpecialMusic => "special/music",
            DriveEndPoint::SpecialMusicChild => "special/music/children",
        }
    }

    pub fn v1_url(self) -> String {
        format!("{}/me/{}", GRAPH_URL, self.as_str())
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn v1_url_with_id(&self, id: &str) -> String {
        format!("{}/{}/{}", GRAPH_URL, id, self.as_str())
    }

    pub fn beta_url(self) -> String {
        format!("{}/me/{}", GRAPH_URL_BETA, self.as_str())
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn beta_url_with_id(&self, id: &str) -> String {
        format!("{}/{}/{}", GRAPH_URL_BETA, id, self.as_str())
    }

    pub fn url(self, host: &str) -> String {
        format!("{}/{}", host, self.as_str())
    }

    pub fn with_ident(self, host: &str, ident: Ident) -> String {
        format!("{}/{}/{}", host, ident.as_ref(), self.as_str())
    }
}

impl From<DriveEndPoint> for String {
    fn from(dep: DriveEndPoint) -> Self {
        dep.v1_url()
    }
}

impl From<DriveEndPoint> for GraphUrl {
    fn from(endpoint: DriveEndPoint) -> Self {
        let url = format!("{}/{}", GRAPH_URL, endpoint.as_str());
        GraphUrl::from(Url::parse(&url).unwrap())
    }
}

impl AsRef<str> for DriveEndPoint {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl ToString for DriveEndPoint {
    fn to_string(&self) -> String {
        String::from(self.as_ref())
    }
}
