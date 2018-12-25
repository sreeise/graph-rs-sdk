/*
Microsoft Graph and OneDrive API use OAuth 2.0 for authorization. By completing an OAuth flow,
your app receives an access token that provides access to the Microsoft Graph a particular
set of permissions for a user.

Your app provides the access token in each request, through an HTTP header:

Authorization: bearer {token}
*/

pub mod endpoint;

use crate::drive::endpoint::DriveEndPoint;
use crate::drive::endpoint::GRAPH_ENDPOINT;
use reqwest::*;
use std;

pub trait DriveRequest {
    fn request(
        &mut self,
        end_point: DriveEndPoint,
    ) -> std::result::Result<Response, reqwest::Error>;
    fn resource_request(
        &mut self,
        resource: DriveItems,
        resource_type: &mut DriveItemType,
        resource_id: &str,
        item_id: &str,
    ) -> std::result::Result<Response, reqwest::Error>;
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Drive {
    access_token: String,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DriveItems {
    Drives,
    Groups,
    Sites,
    Users,
    Me,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DriveItemType {
    CheckIn,
    CheckOut,
    Copy,
    CreateFolder,
    Delete,
    Download,
    GetItem,
    GetItemRoot,
    ListChildren,
    Move,
}

impl DriveItemType {
    fn as_str(&mut self) -> &str {
        match self {
            DriveItemType::CheckIn => "checkin",
            DriveItemType::CheckOut => "checkout",
            DriveItemType::Copy => "copy",
            DriveItemType::CreateFolder => "children",
            DriveItemType::Delete => "",
            DriveItemType::Download => "content",
            DriveItemType::GetItem => "",
            DriveItemType::GetItemRoot => "",
            DriveItemType::ListChildren => "children",
            DriveItemType::Move => "",
        }
    }
}

impl Drive {
    /// Construct a new Drive struct for accessing drive resources
    ///
    /// # Example
    /// ```
    /// use rust_onedrive::drive::Drive;
    ///
    ///  // The Drive struct requires the access token to make
    ///  // authenticated requests to microsoft graph.
    /// let mut drive = Drive::new("B32484FJL;ASFJ");
    /// ```
    pub fn new(access_code: &str) -> Drive {
        Drive {
            access_token: String::from(access_code),
        }
    }

    /// Change the access_token on a Drive struct.
    /// Useful when the current access_token has expired
    ///
    /// # Example
    /// ```
    /// use rust_onedrive::drive::Drive;
    ///
    /// let mut drive = Drive::new("B32484FJL;ASFJ");
    /// drive.reset_access_token("ALS484FJL;ASFJ");
    /// ```
    pub fn reset_access_token(&mut self, access_token: &str) {
        self.access_token = String::from(access_token);
    }

    /// Constructs the url endpoint for a drive item given the drive item resource
    /// name, drive item type, drive id, and item id
    ///
    /// DriveItemType must be a mutable reference.
    ///
    /// # Example
    /// ```
    /// use rust_onedrive::drive::{Drive, DriveItemType, DriveItems};
    ///
    ///     let mut drive = Drive::new("Dfsdf");
    ///     let drive_item_url = drive.resource_drive_item_url(
    ///            DriveItems::Groups,
    ///            &mut DriveItemType::CheckIn,
    ///            "323",
    ///            "222"
    ///        );
    ///     assert_eq!("https://graph.microsoft.com/groups/323/drive/items/222/checkin", drive_item_url);
    /// ```
    pub fn resource_drive_item_url(
        &self,
        resource: DriveItems,
        resource_type: &mut DriveItemType,
        resource_id: &str,
        item_id: &str,
    ) -> String {
        match resource_type {
            DriveItemType::GetItemRoot => self.match_root_resource(resource, resource_id, item_id),
            DriveItemType::GetItem => self.match_no_resource_type(resource, resource_id, item_id),
            DriveItemType::Delete => self.match_no_resource_type(resource, resource_id, item_id),
            DriveItemType::Move => self.match_no_resource_type(resource, resource_id, item_id),
            _ => self.match_with_resource_type(resource, resource_type, resource_id, item_id),
        }
    }

    fn match_root_resource(
        &self,
        resource: DriveItems,
        resource_id: &str,
        item_id: &str,
    ) -> String {
        match resource {
            DriveItems::Drives => format!(
                "{}/drives/{}/root:/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveItems::Users => format!(
                "{}/users/{}/drive/root:/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveItems::Sites => format!(
                "{}/sites/{}/drive/root:/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveItems::Groups => format!(
                "{}/groups/{}/drive/root:/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveItems::Me => format!("{}/me/drive/root:/{}", GRAPH_ENDPOINT, item_id,),
        }
    }

    fn match_no_resource_type(
        &self,
        resource: DriveItems,
        resource_id: &str,
        item_id: &str,
    ) -> String {
        match resource {
            DriveItems::Drives => format!(
                "{}/drives/{}/items/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveItems::Users => format!(
                "{}/users/{}/drive/items/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveItems::Sites => format!(
                "{}/sites/{}/drive/items/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveItems::Groups => format!(
                "{}/groups/{}/drive/items/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveItems::Me => format!("{}/me/drive/items/{}", GRAPH_ENDPOINT, item_id,),
        }
    }

    fn match_with_resource_type(
        &self,
        resource: DriveItems,
        resource_type: &mut DriveItemType,
        resource_id: &str,
        item_id: &str,
    ) -> String {
        match resource {
            DriveItems::Drives => format!(
                "{}/drives/{}/items/{}/{}",
                GRAPH_ENDPOINT,
                resource_id,
                item_id,
                resource_type.as_str()
            ),
            DriveItems::Users => format!(
                "{}/users/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                resource_id,
                item_id,
                resource_type.as_str()
            ),
            DriveItems::Sites => format!(
                "{}/sites/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                resource_id,
                item_id,
                resource_type.as_str()
            ),
            DriveItems::Groups => format!(
                "{}/groups/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                resource_id,
                item_id,
                resource_type.as_str()
            ),
            DriveItems::Me => {
                if resource_type == &mut DriveItemType::Download {
                    if item_id.ends_with(":") {
                        format!(
                            "{}/me/drive/root/{}/{}",
                            GRAPH_ENDPOINT,
                            item_id,
                            resource_type.as_str(),
                        )
                    } else {
                        format!(
                            "{}/me/drive/items/{}/{}",
                            GRAPH_ENDPOINT,
                            item_id,
                            resource_type.as_str(),
                        )
                    }
                } else {
                    format!(
                        "{}/me/drive/items/{}/{}",
                        GRAPH_ENDPOINT,
                        item_id,
                        resource_type.as_str(),
                    )
                }
            }
        }
    }
}

impl DriveRequest for Drive {
    /// A drive request can make a request to any of the end points on the DriveEndPoint enum
    fn request(
        &mut self,
        end_point: DriveEndPoint,
    ) -> std::result::Result<Response, reqwest::Error> {
        let client = reqwest::Client::builder().build()?;
        let res = client
            .get(end_point.as_str())
            .header(header::AUTHORIZATION, self.access_token.as_str())
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .expect("Error with request to microsoft graph");
        Ok(res)
    }

    fn resource_request(
        &mut self,
        resource: DriveItems,
        resource_type: &mut DriveItemType,
        resource_id: &str,
        item_id: &str,
    ) -> std::result::Result<Response, reqwest::Error> {
        let client = reqwest::Client::builder().build()?;
        let res = client
            .get(
                self.resource_drive_item_url(resource, resource_type, resource_id, item_id)
                    .as_str(),
            )
            .header(header::AUTHORIZATION, self.access_token.as_str())
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .expect("Error with request to microsoft graph");
        Ok(res)
    }
}

#[cfg(test)]
mod drive_tests {
    use super::*;

    #[test]
    fn drive_access_token() {
        let mut drive = Drive::new("232344");
        assert_eq!(drive.access_token, "232344");
        drive.reset_access_token("3232434");
        assert_eq!(drive.access_token, "3232434");
    }

    #[test]
    fn checkin_test() {
        /*
        POST /drives/{driveId}/items/{itemId}/checkin
        POST /groups/{groupId}/drive/items/{itemId}/checkin
        POST /me/drive/items/{item-id}/checkin
        POST /sites/{siteId}/drive/items/{itemId}/checkin
        POST /users/{userId}/drive/items/{itemId}/checkin
        */
        let mut drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let checkin_item_drive = drive.resource_drive_item_url(
            DriveItems::Drives,
            &mut DriveItemType::CheckIn,
            "driveId",
            "itemId",
        );
        let checkin_item_group = drive.resource_drive_item_url(
            DriveItems::Groups,
            &mut DriveItemType::CheckIn,
            "groupId",
            "itemId",
        );
        let checkin_item_users = drive.resource_drive_item_url(
            DriveItems::Users,
            &mut DriveItemType::CheckIn,
            "userId",
            "itemId",
        );
        let checkin_item_sites = drive.resource_drive_item_url(
            DriveItems::Sites,
            &mut DriveItemType::CheckIn,
            "siteId",
            "itemId",
        );
        let checkin_item_me = drive.resource_drive_item_url(
            DriveItems::Me,
            &mut DriveItemType::CheckIn,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            checkin_item_drive,
            "https://graph.microsoft.com/drives/driveId/items/itemId/checkin"
        );
        assert_eq!(
            checkin_item_group,
            "https://graph.microsoft.com/groups/groupId/drive/items/itemId/checkin"
        );
        assert_eq!(
            checkin_item_users,
            "https://graph.microsoft.com/users/userId/drive/items/itemId/checkin"
        );
        assert_eq!(
            checkin_item_sites,
            "https://graph.microsoft.com/sites/siteId/drive/items/itemId/checkin"
        );
        assert_eq!(
            checkin_item_me,
            "https://graph.microsoft.com/me/drive/items/item-Id/checkin"
        );
    }

    #[test]
    fn checkout_test() {
        /*
        POST /drives/{driveId}/items/{itemId}/checkout
        POST /groups/{groupId}/drive/items/{itemId}/checkout
        POST /me/drive/items/{item-id}/checkout
        POST /sites/{siteId}/drive/items/{itemId}/checkout
        POST /users/{userId}/drive/items/{itemId}/checkout
        */
        let mut drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let checkout_item_drive = drive.resource_drive_item_url(
            DriveItems::Drives,
            &mut DriveItemType::CheckOut,
            "driveId",
            "itemId",
        );
        let checkout_item_group = drive.resource_drive_item_url(
            DriveItems::Groups,
            &mut DriveItemType::CheckOut,
            "groupId",
            "itemId",
        );
        let checkout_item_users = drive.resource_drive_item_url(
            DriveItems::Users,
            &mut DriveItemType::CheckOut,
            "userId",
            "itemId",
        );
        let checkout_item_sites = drive.resource_drive_item_url(
            DriveItems::Sites,
            &mut DriveItemType::CheckOut,
            "siteId",
            "itemId",
        );
        let checkout_item_me = drive.resource_drive_item_url(
            DriveItems::Me,
            &mut DriveItemType::CheckOut,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            checkout_item_drive,
            "https://graph.microsoft.com/drives/driveId/items/itemId/checkout"
        );
        assert_eq!(
            checkout_item_group,
            "https://graph.microsoft.com/groups/groupId/drive/items/itemId/checkout"
        );
        assert_eq!(
            checkout_item_users,
            "https://graph.microsoft.com/users/userId/drive/items/itemId/checkout"
        );
        assert_eq!(
            checkout_item_sites,
            "https://graph.microsoft.com/sites/siteId/drive/items/itemId/checkout"
        );
        assert_eq!(
            checkout_item_me,
            "https://graph.microsoft.com/me/drive/items/item-Id/checkout"
        );
    }

    #[test]
    fn copy_test() {
        /*
        POST /drives/{driveId}/items/{itemId}/copy
        POST /groups/{groupId}/drive/items/{itemId}/copy
        POST /me/drive/items/{item-id}/copy
        POST /sites/{siteId}/drive/items/{itemId}/copy
        POST /users/{userId}/drive/items/{itemId}/copy
        */
        let mut drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let copy_item_drive = drive.resource_drive_item_url(
            DriveItems::Drives,
            &mut DriveItemType::Copy,
            "driveId",
            "itemId",
        );
        let copy_item_group = drive.resource_drive_item_url(
            DriveItems::Groups,
            &mut DriveItemType::Copy,
            "groupId",
            "itemId",
        );
        let copy_item_users = drive.resource_drive_item_url(
            DriveItems::Users,
            &mut DriveItemType::Copy,
            "userId",
            "itemId",
        );
        let copy_item_sites = drive.resource_drive_item_url(
            DriveItems::Sites,
            &mut DriveItemType::Copy,
            "siteId",
            "itemId",
        );
        let copy_item_me = drive.resource_drive_item_url(
            DriveItems::Me,
            &mut DriveItemType::Copy,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            copy_item_drive,
            "https://graph.microsoft.com/drives/driveId/items/itemId/copy"
        );
        assert_eq!(
            copy_item_group,
            "https://graph.microsoft.com/groups/groupId/drive/items/itemId/copy"
        );
        assert_eq!(
            copy_item_users,
            "https://graph.microsoft.com/users/userId/drive/items/itemId/copy"
        );
        assert_eq!(
            copy_item_sites,
            "https://graph.microsoft.com/sites/siteId/drive/items/itemId/copy"
        );
        assert_eq!(
            copy_item_me,
            "https://graph.microsoft.com/me/drive/items/item-Id/copy"
        );
    }

    #[test]
    fn create_folder_test() {
        /*
        POST /drives/{drive-id}/items/{parent-item-id}/children
        POST /groups/{group-id}/drive/items/{parent-item-id}/children
        POST /me/drive/items/{parent-item-id}/children
        POST /sites/{site-id}/drive/items/{parent-item-id}/children
        POST /users/{user-id}/drive/items/{parent-item-id}/children
        */
        let mut drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let create_folder_item_drive = drive.resource_drive_item_url(
            DriveItems::Drives,
            &mut DriveItemType::CreateFolder,
            "driveId",
            "itemId",
        );
        let create_folder_item_group = drive.resource_drive_item_url(
            DriveItems::Groups,
            &mut DriveItemType::CreateFolder,
            "groupId",
            "itemId",
        );
        let create_folder_item_users = drive.resource_drive_item_url(
            DriveItems::Users,
            &mut DriveItemType::CreateFolder,
            "userId",
            "itemId",
        );
        let create_folder_item_sites = drive.resource_drive_item_url(
            DriveItems::Sites,
            &mut DriveItemType::CreateFolder,
            "siteId",
            "itemId",
        );
        let create_folder_item_me = drive.resource_drive_item_url(
            DriveItems::Me,
            &mut DriveItemType::CreateFolder,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            create_folder_item_drive,
            "https://graph.microsoft.com/drives/driveId/items/itemId/children"
        );
        assert_eq!(
            create_folder_item_group,
            "https://graph.microsoft.com/groups/groupId/drive/items/itemId/children"
        );
        assert_eq!(
            create_folder_item_users,
            "https://graph.microsoft.com/users/userId/drive/items/itemId/children"
        );
        assert_eq!(
            create_folder_item_sites,
            "https://graph.microsoft.com/sites/siteId/drive/items/itemId/children"
        );
        assert_eq!(
            create_folder_item_me,
            "https://graph.microsoft.com/me/drive/items/item-Id/children"
        );
    }

    #[test]
    fn delete_item_test() {
        /*
        DELETE /drives/{drive-id}/items/{item-id}
        DELETE /groups/{group-id}/drive/items/{item-id}
        DELETE /me/drive/items/{item-id}
        DELETE /sites/{siteId}/drive/items/{itemId}
        DELETE /users/{userId}/drive/items/{itemId}
        */
        let mut drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let delete_item_drive = drive.resource_drive_item_url(
            DriveItems::Drives,
            &mut DriveItemType::Delete,
            "driveId",
            "itemId",
        );
        let delete_item_group = drive.resource_drive_item_url(
            DriveItems::Groups,
            &mut DriveItemType::Delete,
            "groupId",
            "itemId",
        );
        let delete_item_users = drive.resource_drive_item_url(
            DriveItems::Users,
            &mut DriveItemType::Delete,
            "userId",
            "itemId",
        );
        let delete_item_sites = drive.resource_drive_item_url(
            DriveItems::Sites,
            &mut DriveItemType::Delete,
            "siteId",
            "itemId",
        );
        let delete_item_me = drive.resource_drive_item_url(
            DriveItems::Me,
            &mut DriveItemType::Delete,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            delete_item_drive,
            "https://graph.microsoft.com/drives/driveId/items/itemId"
        );
        assert_eq!(
            delete_item_group,
            "https://graph.microsoft.com/groups/groupId/drive/items/itemId"
        );
        assert_eq!(
            delete_item_users,
            "https://graph.microsoft.com/users/userId/drive/items/itemId"
        );
        assert_eq!(
            delete_item_sites,
            "https://graph.microsoft.com/sites/siteId/drive/items/itemId"
        );
        assert_eq!(
            delete_item_me,
            "https://graph.microsoft.com/me/drive/items/item-Id"
        );
    }

    #[test]
    fn download_item_test() {
        /*
        DELETE /drives/{drive-id}/items/{item-id}
        DELETE /groups/{group-id}/drive/items/{item-id}
        DELETE /me/drive/items/{item-id}
        DELETE /sites/{siteId}/drive/items/{itemId}
        DELETE /users/{userId}/drive/items/{itemId}
        */
        let mut drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let download_item_drive = drive.resource_drive_item_url(
            DriveItems::Drives,
            &mut DriveItemType::Download,
            "driveId",
            "itemId",
        );
        let download_item_group = drive.resource_drive_item_url(
            DriveItems::Groups,
            &mut DriveItemType::Download,
            "groupId",
            "itemId",
        );
        let download_item_users = drive.resource_drive_item_url(
            DriveItems::Users,
            &mut DriveItemType::Download,
            "userId",
            "itemId",
        );
        let download_item_sites = drive.resource_drive_item_url(
            DriveItems::Sites,
            &mut DriveItemType::Download,
            "siteId",
            "itemId",
        );
        let download_item_me = drive.resource_drive_item_url(
            DriveItems::Me,
            &mut DriveItemType::Download,
            "meId_not_used",
            "item-Id",
        );
        let download_item_me_path = drive.resource_drive_item_url(
            DriveItems::Me,
            &mut DriveItemType::Download,
            "meId_not_used",
            "item-Id/itemid:",
        );

        assert_eq!(
            download_item_drive,
            "https://graph.microsoft.com/drives/driveId/items/itemId/content"
        );
        assert_eq!(
            download_item_group,
            "https://graph.microsoft.com/groups/groupId/drive/items/itemId/content"
        );
        assert_eq!(
            download_item_users,
            "https://graph.microsoft.com/users/userId/drive/items/itemId/content"
        );
        assert_eq!(
            download_item_sites,
            "https://graph.microsoft.com/sites/siteId/drive/items/itemId/content"
        );
        assert_eq!(
            download_item_me,
            "https://graph.microsoft.com/me/drive/items/item-Id/content"
        );
        assert_eq!(
            download_item_me_path,
            "https://graph.microsoft.com/me/drive/root/item-Id/itemid:/content"
        );
    }

    #[test]
    fn get_item_test() {
        /*
        GET /drives/{drive-id}/items/{item-id}
        GET /groups/{group-id}/drive/items/{item-id}
        GET /me/drive/items/{item-id}
        GET /sites/{siteId}/drive/items/{itemId}
        GET /users/{userId}/drive/items/{itemId}
        */
        let mut drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let get_item_drive = drive.resource_drive_item_url(
            DriveItems::Drives,
            &mut DriveItemType::GetItem,
            "driveId",
            "itemId",
        );
        let get_item_group = drive.resource_drive_item_url(
            DriveItems::Groups,
            &mut DriveItemType::GetItem,
            "groupId",
            "itemId",
        );
        let get_item_users = drive.resource_drive_item_url(
            DriveItems::Users,
            &mut DriveItemType::GetItem,
            "userId",
            "itemId",
        );
        let get_item_sites = drive.resource_drive_item_url(
            DriveItems::Sites,
            &mut DriveItemType::GetItem,
            "siteId",
            "itemId",
        );
        let get_item_me = drive.resource_drive_item_url(
            DriveItems::Me,
            &mut DriveItemType::GetItem,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            get_item_drive,
            "https://graph.microsoft.com/drives/driveId/items/itemId"
        );
        assert_eq!(
            get_item_group,
            "https://graph.microsoft.com/groups/groupId/drive/items/itemId"
        );
        assert_eq!(
            get_item_users,
            "https://graph.microsoft.com/users/userId/drive/items/itemId"
        );
        assert_eq!(
            get_item_sites,
            "https://graph.microsoft.com/sites/siteId/drive/items/itemId"
        );
        assert_eq!(
            get_item_me,
            "https://graph.microsoft.com/me/drive/items/item-Id"
        );
    }

    #[test]
    fn get_item_root_test() {
        /*
        GET /drives/{drive-id}/root:/{item-path}
        GET /groups/{group-id}/drive/root:/{item-path}
        GET /me/drive/root:/{item-path}
        GET /sites/{siteId}/drive/root:/{item-path}
        GET /users/{userId}/drive/root:/{item-path}
        */

        let mut drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let root_item_drive = drive.resource_drive_item_url(
            DriveItems::Drives,
            &mut DriveItemType::GetItemRoot,
            "driveId",
            "itemId",
        );
        let root_item_group = drive.resource_drive_item_url(
            DriveItems::Groups,
            &mut DriveItemType::GetItemRoot,
            "groupId",
            "itemId",
        );
        let root_item_users = drive.resource_drive_item_url(
            DriveItems::Users,
            &mut DriveItemType::GetItemRoot,
            "userId",
            "itemId",
        );
        let root_item_sites = drive.resource_drive_item_url(
            DriveItems::Sites,
            &mut DriveItemType::GetItemRoot,
            "siteId",
            "itemId",
        );
        let root_item_me = drive.resource_drive_item_url(
            DriveItems::Me,
            &mut DriveItemType::GetItemRoot,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            root_item_drive,
            "https://graph.microsoft.com/drives/driveId/root:/itemId"
        );
        assert_eq!(
            root_item_group,
            "https://graph.microsoft.com/groups/groupId/drive/root:/itemId"
        );
        assert_eq!(
            root_item_users,
            "https://graph.microsoft.com/users/userId/drive/root:/itemId"
        );
        assert_eq!(
            root_item_sites,
            "https://graph.microsoft.com/sites/siteId/drive/root:/itemId"
        );
        assert_eq!(
            root_item_me,
            "https://graph.microsoft.com/me/drive/root:/item-Id"
        );
    }

    #[test]
    fn list_children_test() {
        /*
        GET /drives/{drive-id}/items/{item-id}/children
        GET /groups/{group-id}/drive/items/{item-id}/children
        GET /me/drive/items/{item-id}/children
        GET /sites/{site-id}/drive/items/{item-id}/children
        GET /users/{user-id}/drive/items/{item-id}/children
        */
        let mut drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let list_children_drive = drive.resource_drive_item_url(
            DriveItems::Drives,
            &mut DriveItemType::ListChildren,
            "driveId",
            "itemId",
        );
        let list_children_group = drive.resource_drive_item_url(
            DriveItems::Groups,
            &mut DriveItemType::ListChildren,
            "groupId",
            "itemId",
        );
        let list_children_users = drive.resource_drive_item_url(
            DriveItems::Users,
            &mut DriveItemType::ListChildren,
            "userId",
            "itemId",
        );
        let list_children_sites = drive.resource_drive_item_url(
            DriveItems::Sites,
            &mut DriveItemType::ListChildren,
            "siteId",
            "itemId",
        );
        let list_children_me = drive.resource_drive_item_url(
            DriveItems::Me,
            &mut DriveItemType::ListChildren,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            list_children_drive,
            "https://graph.microsoft.com/drives/driveId/items/itemId/children"
        );
        assert_eq!(
            list_children_group,
            "https://graph.microsoft.com/groups/groupId/drive/items/itemId/children"
        );
        assert_eq!(
            list_children_users,
            "https://graph.microsoft.com/users/userId/drive/items/itemId/children"
        );
        assert_eq!(
            list_children_sites,
            "https://graph.microsoft.com/sites/siteId/drive/items/itemId/children"
        );
        assert_eq!(
            list_children_me,
            "https://graph.microsoft.com/me/drive/items/item-Id/children"
        );
    }

    #[test]
    fn move_test() {
        /*
        PATCH /drives/{drive-id}/items/{item-id}
        PATCH /groups/{group-id}/drive/{item-id}
        PATCH /me/drive/items/{item-id}
        PATCH /sites/{site-id}/drive/items/{item-id}
        PATCH /users/{user-id}/drive/items/{item-id}
        */
        let mut drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let move_item_drive = drive.resource_drive_item_url(
            DriveItems::Drives,
            &mut DriveItemType::Move,
            "driveId",
            "itemId",
        );
        let move_item_group = drive.resource_drive_item_url(
            DriveItems::Groups,
            &mut DriveItemType::Move,
            "groupId",
            "itemId",
        );
        let move_item_users = drive.resource_drive_item_url(
            DriveItems::Users,
            &mut DriveItemType::Move,
            "userId",
            "itemId",
        );
        let move_item_sites = drive.resource_drive_item_url(
            DriveItems::Sites,
            &mut DriveItemType::Move,
            "siteId",
            "itemId",
        );
        let move_item_me = drive.resource_drive_item_url(
            DriveItems::Me,
            &mut DriveItemType::Move,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            move_item_drive,
            "https://graph.microsoft.com/drives/driveId/items/itemId"
        );
        assert_eq!(
            move_item_group,
            "https://graph.microsoft.com/groups/groupId/drive/items/itemId"
        );
        assert_eq!(
            move_item_users,
            "https://graph.microsoft.com/users/userId/drive/items/itemId"
        );
        assert_eq!(
            move_item_sites,
            "https://graph.microsoft.com/sites/siteId/drive/items/itemId"
        );
        assert_eq!(
            move_item_me,
            "https://graph.microsoft.com/me/drive/items/item-Id"
        );
    }
}
