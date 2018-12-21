/*
GET DRIVE
    https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/drive_get?view=odsp-graph-online
    https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/drive_list?view=odsp-graph-online

    GET /me/drive
    GET /drive
    GET /users/{idOrUserPrincipalName}/drive
    GET /groups/{groupId}/drive
    GET /sites/{siteId}/drive
    GET /drives/{drive-id}


GET SPECIAL FOLDER
    https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/drive_get_specialfolder?view=odsp-graph-online

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

    GET /me/drive/special/{special-folder-name}
    GET /me/drive/special/{special-folder-name}/children


SHARED FILES
    https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/drive_sharedwithme?view=odsp-graph-online

    GET /me/drive/sharedWithMe
    GET /drives/{remoteItem-driveId}/items/{remoteItem-id}


RECENT ITEM
    https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/drive_recent?view=odsp-graph-online

    GET /me/drive/recent
    GET /drives/{remoteItem-driveId}/items/{remoteItem-id}
*/

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum DriveEndPoint {
    DriveMe,
    Drive,
    DriveRoot,
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
            DriveEndPoint::Drive => "https://graph.microsoft.com/v1.0/drive",
            DriveEndPoint::DriveMe => "https://graph.microsoft.com/v1.0/me/drive",
            DriveEndPoint::DriveRoot => "https://graph.microsoft.com/v1.0/me/drive/root",
            DriveEndPoint::SharedWithMe => "https://graph.microsoft.com/v1.0/me/drive/sharedWithMe",
            DriveEndPoint::DriveRecent => "https://graph.microsoft.com/v1.0/me/drive/recent",
            DriveEndPoint::SpecialDocuments => "https://graph.microsoft.com/v1.0/documents",
            DriveEndPoint::SpecialDocumentsChild => "https://graph.microsoft.com/v1.0/documents/children",
            DriveEndPoint::SpecialPhotos => "https://graph.microsoft.com/v1.0/photos",
            DriveEndPoint::SpecialPhotosChild => "https://graph.microsoft.com/v1.0/photos/children",
            DriveEndPoint::SpecialCameraRoll => "https://graph.microsoft.com/v1.0/cameraroll",
            DriveEndPoint::SpecialCameraRollChild => "https://graph.microsoft.com/v1.0/cameraroll/children",
            DriveEndPoint::SpecialAppRoot => "https://graph.microsoft.com/v1.0/approot",
            DriveEndPoint::SpecialAppRootChild => "https://graph.microsoft.com/v1.0/approot/children",
            DriveEndPoint::SpecialMusic => "https://graph.microsoft.com/v1.0/music",
            DriveEndPoint::SpecialMusicChild => "https://graph.microsoft.com/v1.0/music/children",
        }
    }
}

#[derive(Debug)]
pub enum SiteEndPoint {
    RootSiteTenant,
    SharePointTenant,
    JPNTenant,
}

impl SiteEndPoint {
    pub fn as_str(&mut self) -> &str {
        match *self {
            SiteEndPoint::RootSiteTenant => "https://graph.microsoft.com/v1.0/sites/root",
            SiteEndPoint::SharePointTenant => "https://graph.microsoft.com/v1.0/sites/contoso.sharepoint.com",
            SiteEndPoint::JPNTenant => "https://graph.microsoft.com/v1.0/sites/sites/JPN",
        }
    }
 }
