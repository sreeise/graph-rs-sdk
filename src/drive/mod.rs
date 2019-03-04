/*
Microsoft Graph and OneDrive API use OAuth 2.0 for authorization. By completing an OAuth flow,
your app receives an access token that provides access to the Microsoft Graph a particular
set of permissions for a user.

Your app provides the access token in each request, through an HTTP header:

Authorization: bearer {token}
*/

use crate::drive::driveaction::DriveAction;
use crate::drive::driveresource::DriveResource;
use graph_error::RequestError;
use graph_oauth::oauth::OAuth;
use reqwest::*;
use std;

pub mod endpoint;
#[macro_use]
pub mod query_string;
pub mod base;
pub mod discovery;
pub mod driveaction;
pub mod driveresource;
pub mod item;

pub static GRAPH_ENDPOINT: &str = "https://graph.microsoft.com/v1.0";

pub type DriveResponse = std::result::Result<Response, RequestError>;
pub type UrlResult = std::io::Result<String>;
pub type ItemResult<T> = std::result::Result<T, RequestError>;

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

    fn build_request(
        &self,
        url: &str,
        request_type: &str,
        content_type: &str,
        access_token: &str,
    ) -> DriveResponse {
        let client = reqwest::Client::builder().build()?;
        match request_type {
            "GET" => {
                let res = client
                    .get(url)
                    .header(header::AUTHORIZATION, access_token)
                    .header(header::CONTENT_TYPE, content_type)
                    .send()
                    .expect("Error with request to microsoft graph-error");
                Ok(res)
            },
            "POST" => {
                let res = client
                    .post(url)
                    .header(header::AUTHORIZATION, access_token)
                    .header(header::CONTENT_TYPE, content_type)
                    .send()
                    .expect("Error with request to microsoft graph-error");
                Ok(res)
            },
            _ => unimplemented!(),
        }
    }

    #[allow(dead_code)]
    fn resource_request(
        &mut self,
        resource: DriveResource,
        drive_action: DriveAction,
        resource_id: &str,
        item_id: &str,
    ) -> DriveResponse {
        let url = self.resource_drive_item_url(resource, drive_action, resource_id, item_id);
        self.build_request(
            url.as_str(),
            "GET",
            "application/json",
            self.access_token.as_str(),
        )
    }

    #[allow(dead_code)]
    fn get_with_url(&self, url: String) -> DriveResponse {
        self.build_request(
            url.as_str(),
            "GET",
            "application/json",
            self.access_token.as_str(),
        )
    }

    #[allow(dead_code)]
    fn post_with_url(&self, url: String) -> DriveResponse {
        self.build_request(
            url.as_str(),
            "POST",
            "application/json",
            self.access_token.as_str(),
        )
    }

    #[allow(dead_code)]
    fn req_with_url(url: &str, access_token: &str) -> Result<Response> {
        let client = reqwest::Client::builder().build()?;
        client
            .get(url)
            .header(header::AUTHORIZATION, access_token)
            .header(header::CONTENT_TYPE, "application/json")
            .send()
    }

    pub fn url(
        resource: DriveResource,
        drive_action: DriveAction,
        resource_id: &str,
        item_id: &str,
    ) -> String {
        match drive_action {
            DriveAction::GetItemRoot | DriveAction::TrackChanges => {
                Drive::root_resource_url(resource, drive_action, resource_id, item_id)
            },
            DriveAction::GetItem | DriveAction::Delete | DriveAction::Move => {
                Drive::no_drive_action_url(resource, resource_id, item_id)
            },
            _ => Drive::with_drive_action_url(resource, drive_action, resource_id, item_id),
        }
    }

    pub fn with_drive_action_url(
        resource: DriveResource,
        drive_action: DriveAction,
        resource_id: &str,
        item_id: &str,
    ) -> String {
        let rt = match drive_action {
            DriveAction::CheckIn => "checkin",
            DriveAction::CheckOut => "checkout",
            DriveAction::Copy => "copy",
            DriveAction::ListVersions => "versions",
            DriveAction::TrackChanges => "delta",
            DriveAction::Download | DriveAction::Upload => "content",
            DriveAction::CreateUploadSession => "createUploadSession",
            DriveAction::ListChildren | DriveAction::CreateFolder => "children",
            DriveAction::Preview => "preview",
            DriveAction::Activities => "activities",
            DriveAction::Move |
            DriveAction::GetItem |
            DriveAction::GetItemRoot |
            DriveAction::Delete => "",
        };

        match resource {
            DriveResource::Drives => format!(
                "{}/drives/{}/items/{}/{}",
                GRAPH_ENDPOINT, resource_id, item_id, rt,
            ),
            DriveResource::Users => format!(
                "{}/users/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT, resource_id, item_id, rt,
            ),
            DriveResource::Sites => format!(
                "{}/sites/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT, resource_id, item_id, rt,
            ),
            DriveResource::Groups => format!(
                "{}/groups/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT, resource_id, item_id, rt,
            ),
            DriveResource::Me => {
                if drive_action == DriveAction::Download {
                    if item_id.ends_with(':') {
                        format!("{}/me/drive/root/{}/{}", GRAPH_ENDPOINT, item_id, rt,)
                    } else {
                        format!("{}/me/drive/items/{}/{}", GRAPH_ENDPOINT, item_id, rt,)
                    }
                } else {
                    format!("{}/me/drive/items/{}/{}", GRAPH_ENDPOINT, item_id, rt,)
                }
            },
        }
    }

    pub fn root_resource_url(
        resource: DriveResource,
        drive_action: DriveAction,
        resource_id: &str,
        item_id: &str,
    ) -> String {
        if drive_action == DriveAction::TrackChanges {
            match resource {
                DriveResource::Drives => {
                    format!("{}/drives/{}/drive/root/delta", GRAPH_ENDPOINT, resource_id)
                },
                DriveResource::Users => {
                    format!("{}/users/{}/drive/root/delta", GRAPH_ENDPOINT, resource_id)
                },
                DriveResource::Sites => {
                    format!("{}/sites/{}/drive/root/delta", GRAPH_ENDPOINT, resource_id)
                },
                DriveResource::Groups => {
                    format!("{}/groups/{}/drive/root/delta", GRAPH_ENDPOINT, resource_id)
                },
                DriveResource::Me => format!("{}/me/drive/root/delta", GRAPH_ENDPOINT,),
            }
        } else {
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
    }

    /// Change the access_token on a Drive struct.
    /// Useful when the current access_token has expired
    ///
    /// # Example
    /// ```
    /// use rust_onedrive::drive::Drive;
    ///
    /// let mut drive = Drive::new("B32484FJL;ASFJ");
    /// drive.token("ALS484FJL;ASFJ");
    /// ```
    pub fn token(&mut self, access_token: &str) {
        self.access_token = String::from(access_token);
    }

    /// Constructs the url endpoint for a drive item given the drive item resource
    /// name, drive item type, drive id, and item id
    ///
    /// DriveAction must be a mutable reference.
    ///
    /// # Example
    /// ```
    /// use rust_onedrive::drive::Drive;
    /// use rust_onedrive::drive::driveaction::DriveAction;
    /// use rust_onedrive::drive::driveresource::DriveResource;
    ///
    ///     let mut drive = Drive::new("Dfsdf");
    ///     let drive_item_url = drive.resource_drive_item_url(
    ///            DriveResource::Groups,
    ///            DriveAction::CheckIn,
    ///            "323",
    ///            "222"
    ///        );
    ///     assert_eq!("https://graph.microsoft.com/v1.0/groups/323/drive/items/222/checkin", drive_item_url);
    /// ```
    pub fn resource_drive_item_url(
        &self,
        resource: DriveResource,
        drive_action: DriveAction,
        resource_id: &str,
        item_id: &str,
    ) -> String {
        match drive_action {
            DriveAction::GetItemRoot | DriveAction::TrackChanges => {
                Drive::root_resource_url(resource, drive_action, resource_id, item_id)
            },
            DriveAction::GetItem | DriveAction::Delete | DriveAction::Move => {
                Drive::no_drive_action_url(resource, resource_id, item_id)
            },
            _ => Drive::with_drive_action_url(resource, drive_action, resource_id, item_id),
        }
    }

    /// Formats URLs where the DriveAction is an empty string or in other words
    /// a URL doesn't have or end with a DriveAction
    fn no_drive_action_url(resource: DriveResource, resource_id: &str, item_id: &str) -> String {
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
}

impl From<OAuth> for Drive {
    fn from(oauth: OAuth) -> Self {
        match oauth.get_access_token() {
            Some(t) => {
                let ac = t.try_borrow_mut();
                if let Ok(rt) = ac {
                    let token = rt.clone();
                    return Drive::new(token.get_access_token());
                }
            },
            None => panic!("Missing Access Token"),
        }
        panic!("Missing Access Token");
    }
}
