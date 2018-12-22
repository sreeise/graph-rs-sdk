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
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DriveItemType {
    CheckIn,
    CheckOut,
    Copy,
    CreateFolder,
}

impl DriveItemType {
    fn as_str(&mut self) -> &str {
        match *self {
            DriveItemType::CheckIn => "checkin",
            DriveItemType::CheckOut => "checkout",
            DriveItemType::Copy => "copy",
            DriveItemType::CreateFolder => "children",
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
        &mut self,
        resource: DriveItems,
        resource_type: &mut DriveItemType,
        drive_id: &str,
        item_id: &str,
    ) -> String {
        match resource {
            DriveItems::Drives => format!(
                "{}/drives/{}/items/{}/{}",
                GRAPH_ENDPOINT,
                drive_id,
                item_id,
                resource_type.as_str()
            ),
            DriveItems::Users => format!(
                "{}/users/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                drive_id,
                item_id,
                resource_type.as_str()
            ),
            DriveItems::Sites => format!(
                "{}/sites{}/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                drive_id,
                item_id,
                resource_type.as_str()
            ),
            DriveItems::Groups => format!(
                "{}/groups/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                drive_id,
                item_id,
                resource_type.as_str()
            ),
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
    fn drive_item() {
        let mut drive = Drive::new("Dfsdf");
        let drive_item_url = drive.resource_drive_item_url(
            DriveItems::Groups,
            &mut DriveItemType::CheckIn,
            "323",
            "222",
        );

        assert_eq!(
            "https://graph.microsoft.com/groups/323/drive/items/222/checkin",
            drive_item_url
        );
        let mut drive = Drive::new("Dfsdf");
        let drive_item_url = drive.resource_drive_item_url(
            DriveItems::Groups,
            &mut DriveItemType::CheckOut,
            "323",
            "222",
        );
        assert_eq!(
            "https://graph.microsoft.com/groups/323/drive/items/222/checkout",
            drive_item_url
        );

        let mut drive = Drive::new("Dfsdf");
        let drive_item_url = drive.resource_drive_item_url(
            DriveItems::Groups,
            &mut DriveItemType::Copy,
            "323",
            "222",
        );
        assert_eq!(
            "https://graph.microsoft.com/groups/323/drive/items/222/copy",
            drive_item_url
        );

        let mut drive = Drive::new("Dfsdf");
        let drive_item_url = drive.resource_drive_item_url(
            DriveItems::Groups,
            &mut DriveItemType::CreateFolder,
            "323",
            "222",
        );
        assert_eq!(
            "https://graph.microsoft.com/groups/323/drive/items/222/children",
            drive_item_url
        );
    }
}
