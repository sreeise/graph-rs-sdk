/*
Microsoft Graph and OneDrive API use OAuth 2.0 for authorization. By completing an OAuth flow,
your app receives an access token that provides access to the Microsoft Graph a particular
set of permissions for a user.

Your app provides the access token in each request, through an HTTP header:

Authorization: bearer {token}
*/

pub mod drive_builder;
pub mod driveitem;
pub mod endpoint;
pub mod identity;

use crate::drive::endpoint::DriveEndPoint;
pub static GRAPH_ENDPOINT: &str = "https://graph.microsoft.com/v1.0";
use reqwest::*;
use std;
/*
Drive Resource: Top level Microsoft Graph resource.
*/

pub static AND_SELECT: &str = "?select=";
pub static AND_EXPAND: &str = "?expand=";
pub static AND_ORDER_BY: &str = "?orderby=";

pub type DriveResponse = std::result::Result<Response, reqwest::Error>;
pub type UrlResult = std::io::Result<String>;

/// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/concepts/addressing-driveitems?view=odsp-graph-online
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DriveResource {
    Drives,
    Groups,
    Sites,
    Users,
    Me,
}

impl DriveResource {
    pub fn as_str(&self) -> String {
        match self {
            DriveResource::Drives => String::from("/drives"),
            DriveResource::Groups => String::from("/groups"),
            DriveResource::Sites => String::from("/sites"),
            DriveResource::Users => String::from("/users"),
            DriveResource::Me => String::from("/me"),
        }
    }
}

#[allow(dead_code)]
enum CustomEndPoint {
    DriveItem,
    DriveItemWithType,
    MeDriveItem,
    MeDriveItemWithType,
    DriveRoot,
}

/// Query string trait for building graph requests with select and expand query strings
///
/// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/concepts/optional-query-parameters?view=odsp-graph-online#selecting-properties
pub trait QueryString {
    fn select(&self, end_point: DriveEndPoint, query: Vec<&str>) -> DriveResponse;
    fn expand(
        &self,
        end_point: DriveEndPoint,
        expand_item: &str,
        query: Vec<&str>,
    ) -> DriveResponse;
    fn select_url(&self, end_point: DriveEndPoint, query: Vec<&str>) -> String;
    fn expand_url(&self, end_point: DriveEndPoint, expand_item: &str, query: Vec<&str>) -> String;
}

pub trait DriveRequest {
    fn request(&mut self, end_point: DriveEndPoint) -> DriveResponse;
    fn resource_request(
        &mut self,
        resource: DriveResource,
        resource_type: &mut DriveItemType,
        resource_id: &str,
        item_id: &str,
    ) -> std::result::Result<Response, reqwest::Error>;

    fn get_with_url(&self, url: String) -> DriveResponse;
    fn post_with_url(&self, url: String) -> DriveResponse;
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

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Drive {
    access_token: String,
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
    pub fn new(access_token: &str) -> Drive {
        Drive {
            access_token: String::from(access_token),
        }
    }

    fn req_url(url: &str, access_token: &str) -> DriveResponse {
        let client = reqwest::Client::builder().build()?;
        let res = client
            .get(url)
            .header(header::AUTHORIZATION, access_token)
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .expect("Error with request to microsoft graph");
        Ok(res)
    }

    pub fn url(
        resource: DriveResource,
        resource_type: &mut DriveItemType,
        resource_id: &str,
        item_id: &str,
    ) -> String {
        match resource_type {
            DriveItemType::GetItemRoot => Drive::match_root_url(resource, resource_id, item_id),
            DriveItemType::GetItem | DriveItemType::Delete | DriveItemType::Move => {
                Drive::match_root_url(resource, resource_id, item_id)
            }
            _ => Drive::match_url(resource, resource_type, resource_id, item_id),
        }
    }

    fn match_url(
        resource: DriveResource,
        resource_type: &mut DriveItemType,
        resource_id: &str,
        item_id: &str,
    ) -> String {
        match resource {
            DriveResource::Drives => format!(
                "{}/drives/{}/items/{}/{}",
                GRAPH_ENDPOINT,
                resource_id,
                item_id,
                resource_type.as_str()
            ),
            DriveResource::Users => format!(
                "{}/users/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                resource_id,
                item_id,
                resource_type.as_str()
            ),
            DriveResource::Sites => format!(
                "{}/sites/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                resource_id,
                item_id,
                resource_type.as_str()
            ),
            DriveResource::Groups => format!(
                "{}/groups/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                resource_id,
                item_id,
                resource_type.as_str()
            ),
            DriveResource::Me => {
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

    fn match_root_url(resource: DriveResource, resource_id: &str, item_id: &str) -> String {
        match resource {
            DriveResource::Drives => format!(
                "{}/drives/{}/root:/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveResource::Users => format!(
                "{}/users/{}/drive/root:/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveResource::Sites => format!(
                "{}/sites/{}/drive/root:/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveResource::Groups => format!(
                "{}/groups/{}/drive/root:/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveResource::Me => format!("{}/me/drive/root:/{}", GRAPH_ENDPOINT, item_id,),
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
    /// use rust_onedrive::drive::{Drive, DriveItemType, DriveResource};
    ///
    ///     let mut drive = Drive::new("Dfsdf");
    ///     let drive_item_url = drive.resource_drive_item_url(
    ///            DriveResource::Groups,
    ///            &mut DriveItemType::CheckIn,
    ///            "323",
    ///            "222"
    ///        );
    ///     assert_eq!("https://graph.microsoft.com/v1.0/groups/323/drive/items/222/checkin", drive_item_url);
    /// ```
    pub fn resource_drive_item_url(
        &self,
        resource: DriveResource,
        resource_type: &mut DriveItemType,
        resource_id: &str,
        item_id: &str,
    ) -> String {
        match resource_type {
            DriveItemType::GetItemRoot => self.match_root_resource(resource, resource_id, item_id),
            DriveItemType::GetItem | DriveItemType::Delete | DriveItemType::Move => {
                self.match_no_resource_type(resource, resource_id, item_id)
            }
            _ => self.match_with_resource_type(resource, resource_type, resource_id, item_id),
        }
    }

    fn match_root_resource(
        &self,
        resource: DriveResource,
        resource_id: &str,
        item_id: &str,
    ) -> String {
        match resource {
            DriveResource::Drives => format!(
                "{}/drives/{}/root:/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveResource::Users => format!(
                "{}/users/{}/drive/root:/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveResource::Sites => format!(
                "{}/sites/{}/drive/root:/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveResource::Groups => format!(
                "{}/groups/{}/drive/root:/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveResource::Me => format!("{}/me/drive/root:/{}", GRAPH_ENDPOINT, item_id,),
        }
    }

    fn match_no_resource_type(
        &self,
        resource: DriveResource,
        resource_id: &str,
        item_id: &str,
    ) -> String {
        match resource {
            DriveResource::Drives => format!(
                "{}/drives/{}/items/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveResource::Users => format!(
                "{}/users/{}/drive/items/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveResource::Sites => format!(
                "{}/sites/{}/drive/items/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveResource::Groups => format!(
                "{}/groups/{}/drive/items/{}",
                GRAPH_ENDPOINT, resource_id, item_id,
            ),
            DriveResource::Me => format!("{}/me/drive/items/{}", GRAPH_ENDPOINT, item_id,),
        }
    }

    fn match_with_resource_type(
        &self,
        resource: DriveResource,
        resource_type: &mut DriveItemType,
        resource_id: &str,
        item_id: &str,
    ) -> String {
        match resource {
            DriveResource::Drives => format!(
                "{}/drives/{}/items/{}/{}",
                GRAPH_ENDPOINT,
                resource_id,
                item_id,
                resource_type.as_str()
            ),
            DriveResource::Users => format!(
                "{}/users/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                resource_id,
                item_id,
                resource_type.as_str()
            ),
            DriveResource::Sites => format!(
                "{}/sites/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                resource_id,
                item_id,
                resource_type.as_str()
            ),
            DriveResource::Groups => format!(
                "{}/groups/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                resource_id,
                item_id,
                resource_type.as_str()
            ),
            DriveResource::Me => {
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
    fn request(&mut self, end_point: DriveEndPoint) -> DriveResponse {
        let client = reqwest::Client::builder().build()?;
        let res = client
            .get(DriveEndPoint::build(end_point).unwrap().as_str())
            .header(header::AUTHORIZATION, self.access_token.as_str())
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .expect("Error with request to microsoft graph");
        Ok(res)
    }

    fn resource_request(
        &mut self,
        resource: DriveResource,
        resource_type: &mut DriveItemType,
        resource_id: &str,
        item_id: &str,
    ) -> DriveResponse {
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

    fn get_with_url(&self, url: String) -> DriveResponse {
        let client = reqwest::Client::builder().build().unwrap();
        let res = client
            .get(url.as_str())
            .header(header::AUTHORIZATION, self.access_token.as_str())
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .expect("Error with request to microsoft graph");

        Ok(res)
    }

    fn post_with_url(&self, url: String) -> DriveResponse {
        let client = reqwest::Client::builder().build().unwrap();
        let res = client
            .post(url.as_str())
            .header(header::AUTHORIZATION, self.access_token.as_str())
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .expect("Error with request to microsoft graph");

        Ok(res)
    }
}

impl QueryString for Drive {
    /// Query String Select
    ///
    /// An expand request url includes an item to expand and the items to select:
    ///     select=name,size
    ///
    /// # Example
    /// ```rust,ignore
    ///    let mut query = Vec::new();
    ///    query.push("name");
    ///    query.push("size");
    ///
    ///   // Forms the request url: "https://graph.microsoft.com/v1.0/drive/root/children?select=name,size"
    ///   // sending the request and returning the response
    ///   let mut req = drive.select(DriveEndPoint::DriveRootChild, query)?;
    ///
    ///   println!("{:#?}", req); // -> Head of response
    ///   println!("{:#?}", req.text()); // -> Body of response
    /// ```
    fn select(&self, end_point: DriveEndPoint, query: Vec<&str>) -> DriveResponse {
        let url = self.select_url(end_point, query);
        self.get_with_url(url)
    }

    /// Query String Expand
    ///
    /// An expand request url includes an item to expand and the items to select:
    ///     expand=children(select=id,name,folder)
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut query = vec!["id", "name", "size"];
    ///
    /// // Forms the request url: "https://graph.microsoft.com/v1.0/me/drive/root?expand=children(select=id,name,size)"
    /// // sending the request and returning the response
    /// let req = drive.expand(DriveEndPoint::DriveRoot, "children", query);
    ///
    ///   println!("{:#?}", req); // -> Head of response
    ///   println!("{:#?}", req.text()); // -> Body of response
    ///```
    /// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/concepts/optional-query-parameters?view=odsp-graph-online#selecting-properties
    fn expand(
        &self,
        end_point: DriveEndPoint,
        expand_item: &str,
        query: Vec<&str>,
    ) -> DriveResponse {
        let url = self.expand_url(end_point, expand_item, query);
        self.get_with_url(url)
    }

    /// Get the URL string a select query string
    fn select_url(&self, end_point: DriveEndPoint, query: Vec<&str>) -> String {
        let query_str = query.join(",");
        let url_vec = vec![
            DriveEndPoint::build(end_point).expect("could not build drive end point"),
            AND_SELECT.to_string(),
            query_str,
        ];
        url_vec.join("")
    }

    /// Get the URL string for a expand query string
    fn expand_url(&self, end_point: DriveEndPoint, expand_item: &str, query: Vec<&str>) -> String {
        let query_str = query.join(",");
        let url_vec = vec![
            DriveEndPoint::build(end_point).expect("could not build drive end point"),
            AND_EXPAND.to_string(),
            expand_item.to_string(),
            "(select=".to_string(),
            query_str,
            String::from(")"),
        ];
        url_vec.join("")
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
        let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let checkin_item_drive = drive.resource_drive_item_url(
            DriveResource::Drives,
            &mut DriveItemType::CheckIn,
            "driveId",
            "itemId",
        );
        let checkin_item_group = drive.resource_drive_item_url(
            DriveResource::Groups,
            &mut DriveItemType::CheckIn,
            "groupId",
            "itemId",
        );
        let checkin_item_users = drive.resource_drive_item_url(
            DriveResource::Users,
            &mut DriveItemType::CheckIn,
            "userId",
            "itemId",
        );
        let checkin_item_sites = drive.resource_drive_item_url(
            DriveResource::Sites,
            &mut DriveItemType::CheckIn,
            "siteId",
            "itemId",
        );
        let checkin_item_me = drive.resource_drive_item_url(
            DriveResource::Me,
            &mut DriveItemType::CheckIn,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            checkin_item_drive,
            "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId/checkin"
        );
        assert_eq!(
            checkin_item_group,
            "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId/checkin"
        );
        assert_eq!(
            checkin_item_users,
            "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId/checkin"
        );
        assert_eq!(
            checkin_item_sites,
            "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId/checkin"
        );
        assert_eq!(
            checkin_item_me,
            "https://graph.microsoft.com/v1.0/me/drive/items/item-Id/checkin"
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
        let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let checkout_item_drive = drive.resource_drive_item_url(
            DriveResource::Drives,
            &mut DriveItemType::CheckOut,
            "driveId",
            "itemId",
        );
        let checkout_item_group = drive.resource_drive_item_url(
            DriveResource::Groups,
            &mut DriveItemType::CheckOut,
            "groupId",
            "itemId",
        );
        let checkout_item_users = drive.resource_drive_item_url(
            DriveResource::Users,
            &mut DriveItemType::CheckOut,
            "userId",
            "itemId",
        );
        let checkout_item_sites = drive.resource_drive_item_url(
            DriveResource::Sites,
            &mut DriveItemType::CheckOut,
            "siteId",
            "itemId",
        );
        let checkout_item_me = drive.resource_drive_item_url(
            DriveResource::Me,
            &mut DriveItemType::CheckOut,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            checkout_item_drive,
            "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId/checkout"
        );
        assert_eq!(
            checkout_item_group,
            "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId/checkout"
        );
        assert_eq!(
            checkout_item_users,
            "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId/checkout"
        );
        assert_eq!(
            checkout_item_sites,
            "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId/checkout"
        );
        assert_eq!(
            checkout_item_me,
            "https://graph.microsoft.com/v1.0/me/drive/items/item-Id/checkout"
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
        let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let copy_item_drive = drive.resource_drive_item_url(
            DriveResource::Drives,
            &mut DriveItemType::Copy,
            "driveId",
            "itemId",
        );
        let copy_item_group = drive.resource_drive_item_url(
            DriveResource::Groups,
            &mut DriveItemType::Copy,
            "groupId",
            "itemId",
        );
        let copy_item_users = drive.resource_drive_item_url(
            DriveResource::Users,
            &mut DriveItemType::Copy,
            "userId",
            "itemId",
        );
        let copy_item_sites = drive.resource_drive_item_url(
            DriveResource::Sites,
            &mut DriveItemType::Copy,
            "siteId",
            "itemId",
        );
        let copy_item_me = drive.resource_drive_item_url(
            DriveResource::Me,
            &mut DriveItemType::Copy,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            copy_item_drive,
            "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId/copy"
        );
        assert_eq!(
            copy_item_group,
            "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId/copy"
        );
        assert_eq!(
            copy_item_users,
            "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId/copy"
        );
        assert_eq!(
            copy_item_sites,
            "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId/copy"
        );
        assert_eq!(
            copy_item_me,
            "https://graph.microsoft.com/v1.0/me/drive/items/item-Id/copy"
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
        let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let create_folder_item_drive = drive.resource_drive_item_url(
            DriveResource::Drives,
            &mut DriveItemType::CreateFolder,
            "driveId",
            "itemId",
        );
        let create_folder_item_group = drive.resource_drive_item_url(
            DriveResource::Groups,
            &mut DriveItemType::CreateFolder,
            "groupId",
            "itemId",
        );
        let create_folder_item_users = drive.resource_drive_item_url(
            DriveResource::Users,
            &mut DriveItemType::CreateFolder,
            "userId",
            "itemId",
        );
        let create_folder_item_sites = drive.resource_drive_item_url(
            DriveResource::Sites,
            &mut DriveItemType::CreateFolder,
            "siteId",
            "itemId",
        );
        let create_folder_item_me = drive.resource_drive_item_url(
            DriveResource::Me,
            &mut DriveItemType::CreateFolder,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            create_folder_item_drive,
            "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId/children"
        );
        assert_eq!(
            create_folder_item_group,
            "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId/children"
        );
        assert_eq!(
            create_folder_item_users,
            "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId/children"
        );
        assert_eq!(
            create_folder_item_sites,
            "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId/children"
        );
        assert_eq!(
            create_folder_item_me,
            "https://graph.microsoft.com/v1.0/me/drive/items/item-Id/children"
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
        let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let delete_item_drive = drive.resource_drive_item_url(
            DriveResource::Drives,
            &mut DriveItemType::Delete,
            "driveId",
            "itemId",
        );
        let delete_item_group = drive.resource_drive_item_url(
            DriveResource::Groups,
            &mut DriveItemType::Delete,
            "groupId",
            "itemId",
        );
        let delete_item_users = drive.resource_drive_item_url(
            DriveResource::Users,
            &mut DriveItemType::Delete,
            "userId",
            "itemId",
        );
        let delete_item_sites = drive.resource_drive_item_url(
            DriveResource::Sites,
            &mut DriveItemType::Delete,
            "siteId",
            "itemId",
        );
        let delete_item_me = drive.resource_drive_item_url(
            DriveResource::Me,
            &mut DriveItemType::Delete,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            delete_item_drive,
            "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId"
        );
        assert_eq!(
            delete_item_group,
            "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId"
        );
        assert_eq!(
            delete_item_users,
            "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId"
        );
        assert_eq!(
            delete_item_sites,
            "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId"
        );
        assert_eq!(
            delete_item_me,
            "https://graph.microsoft.com/v1.0/me/drive/items/item-Id"
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
        let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let download_item_drive = drive.resource_drive_item_url(
            DriveResource::Drives,
            &mut DriveItemType::Download,
            "driveId",
            "itemId",
        );
        let download_item_group = drive.resource_drive_item_url(
            DriveResource::Groups,
            &mut DriveItemType::Download,
            "groupId",
            "itemId",
        );
        let download_item_users = drive.resource_drive_item_url(
            DriveResource::Users,
            &mut DriveItemType::Download,
            "userId",
            "itemId",
        );
        let download_item_sites = drive.resource_drive_item_url(
            DriveResource::Sites,
            &mut DriveItemType::Download,
            "siteId",
            "itemId",
        );
        let download_item_me = drive.resource_drive_item_url(
            DriveResource::Me,
            &mut DriveItemType::Download,
            "meId_not_used",
            "item-Id",
        );
        let download_item_me_path = drive.resource_drive_item_url(
            DriveResource::Me,
            &mut DriveItemType::Download,
            "meId_not_used",
            "item-Id/itemid:",
        );

        assert_eq!(
            download_item_drive,
            "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId/content"
        );
        assert_eq!(
            download_item_group,
            "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId/content"
        );
        assert_eq!(
            download_item_users,
            "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId/content"
        );
        assert_eq!(
            download_item_sites,
            "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId/content"
        );
        assert_eq!(
            download_item_me,
            "https://graph.microsoft.com/v1.0/me/drive/items/item-Id/content"
        );
        assert_eq!(
            download_item_me_path,
            "https://graph.microsoft.com/v1.0/me/drive/root/item-Id/itemid:/content"
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
        let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let get_item_drive = drive.resource_drive_item_url(
            DriveResource::Drives,
            &mut DriveItemType::GetItem,
            "driveId",
            "itemId",
        );
        let get_item_group = drive.resource_drive_item_url(
            DriveResource::Groups,
            &mut DriveItemType::GetItem,
            "groupId",
            "itemId",
        );
        let get_item_users = drive.resource_drive_item_url(
            DriveResource::Users,
            &mut DriveItemType::GetItem,
            "userId",
            "itemId",
        );
        let get_item_sites = drive.resource_drive_item_url(
            DriveResource::Sites,
            &mut DriveItemType::GetItem,
            "siteId",
            "itemId",
        );
        let get_item_me = drive.resource_drive_item_url(
            DriveResource::Me,
            &mut DriveItemType::GetItem,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            get_item_drive,
            "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId"
        );
        assert_eq!(
            get_item_group,
            "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId"
        );
        assert_eq!(
            get_item_users,
            "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId"
        );
        assert_eq!(
            get_item_sites,
            "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId"
        );
        assert_eq!(
            get_item_me,
            "https://graph.microsoft.com/v1.0/me/drive/items/item-Id"
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

        let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let root_item_drive = drive.resource_drive_item_url(
            DriveResource::Drives,
            &mut DriveItemType::GetItemRoot,
            "driveId",
            "itemId",
        );
        let root_item_group = drive.resource_drive_item_url(
            DriveResource::Groups,
            &mut DriveItemType::GetItemRoot,
            "groupId",
            "itemId",
        );
        let root_item_users = drive.resource_drive_item_url(
            DriveResource::Users,
            &mut DriveItemType::GetItemRoot,
            "userId",
            "itemId",
        );
        let root_item_sites = drive.resource_drive_item_url(
            DriveResource::Sites,
            &mut DriveItemType::GetItemRoot,
            "siteId",
            "itemId",
        );
        let root_item_me = drive.resource_drive_item_url(
            DriveResource::Me,
            &mut DriveItemType::GetItemRoot,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            root_item_drive,
            "https://graph.microsoft.com/v1.0/drives/driveId/root:/itemId"
        );
        assert_eq!(
            root_item_group,
            "https://graph.microsoft.com/v1.0/groups/groupId/drive/root:/itemId"
        );
        assert_eq!(
            root_item_users,
            "https://graph.microsoft.com/v1.0/users/userId/drive/root:/itemId"
        );
        assert_eq!(
            root_item_sites,
            "https://graph.microsoft.com/v1.0/sites/siteId/drive/root:/itemId"
        );
        assert_eq!(
            root_item_me,
            "https://graph.microsoft.com/v1.0/me/drive/root:/item-Id"
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
        let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let list_children_drive = drive.resource_drive_item_url(
            DriveResource::Drives,
            &mut DriveItemType::ListChildren,
            "driveId",
            "itemId",
        );
        let list_children_group = drive.resource_drive_item_url(
            DriveResource::Groups,
            &mut DriveItemType::ListChildren,
            "groupId",
            "itemId",
        );
        let list_children_users = drive.resource_drive_item_url(
            DriveResource::Users,
            &mut DriveItemType::ListChildren,
            "userId",
            "itemId",
        );
        let list_children_sites = drive.resource_drive_item_url(
            DriveResource::Sites,
            &mut DriveItemType::ListChildren,
            "siteId",
            "itemId",
        );
        let list_children_me = drive.resource_drive_item_url(
            DriveResource::Me,
            &mut DriveItemType::ListChildren,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            list_children_drive,
            "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId/children"
        );
        assert_eq!(
            list_children_group,
            "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId/children"
        );
        assert_eq!(
            list_children_users,
            "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId/children"
        );
        assert_eq!(
            list_children_sites,
            "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId/children"
        );
        assert_eq!(
            list_children_me,
            "https://graph.microsoft.com/v1.0/me/drive/items/item-Id/children"
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
        let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
        let move_item_drive = drive.resource_drive_item_url(
            DriveResource::Drives,
            &mut DriveItemType::Move,
            "driveId",
            "itemId",
        );
        let move_item_group = drive.resource_drive_item_url(
            DriveResource::Groups,
            &mut DriveItemType::Move,
            "groupId",
            "itemId",
        );
        let move_item_users = drive.resource_drive_item_url(
            DriveResource::Users,
            &mut DriveItemType::Move,
            "userId",
            "itemId",
        );
        let move_item_sites = drive.resource_drive_item_url(
            DriveResource::Sites,
            &mut DriveItemType::Move,
            "siteId",
            "itemId",
        );
        let move_item_me = drive.resource_drive_item_url(
            DriveResource::Me,
            &mut DriveItemType::Move,
            "meId_not_used",
            "item-Id",
        );

        assert_eq!(
            move_item_drive,
            "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId"
        );
        assert_eq!(
            move_item_group,
            "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId"
        );
        assert_eq!(
            move_item_users,
            "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId"
        );
        assert_eq!(
            move_item_sites,
            "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId"
        );
        assert_eq!(
            move_item_me,
            "https://graph.microsoft.com/v1.0/me/drive/items/item-Id"
        );
    }

    #[test]
    fn query_string_test() {
        let expand_query = vec!["id", "name", "size"];
        let select_query = vec!["name", "size"];
        let drive = Drive::new("access_token");

        let expand_string = drive.expand_url(DriveEndPoint::DriveRoot, "children", expand_query);
        assert_eq!(
            expand_string,
            "https://graph.microsoft.com/v1.0/drive/root?expand=children(select=id,name,size)"
        );

        let select_string = drive.select_url(DriveEndPoint::DriveRootChild, select_query);
        assert_eq!(
            select_string,
            "https://graph.microsoft.com/v1.0/drive/root/children?select=name,size"
        );
    }
}
