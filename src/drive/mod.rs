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

use crate::drive::endpoint::DriveEndPoint;
use reqwest::*;
use std;
use std::io;
/*
Drive Resource: Top level Microsoft Graph resource.
*/

pub static GRAPH_ENDPOINT: &str = "https://graph.microsoft.com/v1.0";

pub type DriveResponse = std::result::Result<Response, reqwest::Error>;
pub type UrlResult = std::io::Result<String>;

pub trait DriveSerDe<T> {
    fn serialize(item: T) -> io::Result<String>;
    fn serialize_to_file(path: &str, item: T) -> io::Result<()>;
    fn deserialize_from_str(item_str: &str) -> io::Result<T>;
    fn deserialize_from_file(path: &str) -> io::Result<T>;
}

/// Query string trait for building graph requests with select and expand query strings
///
/// https://docs.microsoft.com/en-us/graph/query-parameters
/// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/concepts/optional-query-parameters?view=odsp-graph-online#selecting-properties
pub trait QueryString {
    fn select(&self, end_point: DriveEndPoint, query: &Vec<&str>) -> DriveResponse;

    fn expand(
        &self,
        end_point: DriveEndPoint,
        expand_item: &str,
        query: &Vec<&str>,
    ) -> DriveResponse;

    fn select_url(&self, end_point: DriveEndPoint, query: &Vec<&str>) -> String;

    fn expand_url(&self, end_point: DriveEndPoint, expand_item: &str, query: &Vec<&str>) -> String;

    // TODO: Add support for filter URL
    // https://docs.microsoft.com/en-us/graph/query-parameters#filter-parameter
    fn filter(&self) {
        unimplemented!();
    }

    // TODO: Add support for order_by URL
    // https://docs.microsoft.com/en-us/graph/query-parameters#orderby-parameter
    fn order_by(&self) {
        unimplemented!();
    }

    // TODO: Add support for search url
    // https://docs.microsoft.com/en-us/graph/query-parameters#search-parameter
    fn search(&self) {
        unimplemented!();
    }

    // TODO: Add Support for format URL
    // https://docs.microsoft.com/en-us/graph/query-parameters#format-parameter
    fn format(&self) {
        unimplemented!();
    }

    // TODO: Add support for skip URL
    // https://docs.microsoft.com/en-us/graph/query-parameters#skip-parameter
    fn skip(&self) {
        unimplemented!();
    }

    // TODO: Add support for top URL
    // https://docs.microsoft.com/en-us/graph/query-parameters#top-parameter
    fn top(&self) {
        unimplemented!();
    }
}

pub trait DriveRequest {
    fn request(&mut self, end_point: DriveEndPoint) -> DriveResponse;
    fn resource_request(
        &mut self,
        resource: DriveResource,
        drive_action: DriveAction,
        resource_id: &str,
        item_id: &str,
    ) -> std::result::Result<Response, reqwest::Error>;

    fn get_with_url(&self, url: String) -> DriveResponse;
    fn post_with_url(&self, url: String) -> DriveResponse;
}

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

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DriveAction {
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
    Upload,
    ListVersions,
    TrackChanges,
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

    #[allow(dead_code)]
    fn req_with_url(url: &str, access_token: &str) -> DriveResponse {
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
        drive_action: DriveAction,
        resource_id: &str,
        item_id: &str,
    ) -> String {
        match drive_action {
            DriveAction::GetItemRoot | DriveAction::TrackChanges => {
                Drive::root_resource_url(resource, drive_action, resource_id, item_id)
            }
            DriveAction::GetItem | DriveAction::Delete | DriveAction::Move => {
                Drive::no_drive_action_url(resource, resource_id, item_id)
            }
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
            DriveAction::ListChildren | DriveAction::CreateFolder => "children",
            DriveAction::Move
            | DriveAction::GetItem
            | DriveAction::GetItemRoot
            | DriveAction::Delete => "",
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
                    if item_id.ends_with(":") {
                        format!("{}/me/drive/root/{}/{}", GRAPH_ENDPOINT, item_id, rt,)
                    } else {
                        format!("{}/me/drive/items/{}/{}", GRAPH_ENDPOINT, item_id, rt,)
                    }
                } else {
                    format!("{}/me/drive/items/{}/{}", GRAPH_ENDPOINT, item_id, rt,)
                }
            }
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
                }
                DriveResource::Users => {
                    format!("{}/users/{}/drive/root/delta", GRAPH_ENDPOINT, resource_id)
                }
                DriveResource::Sites => {
                    format!("{}/sites/{}/drive/root/delta", GRAPH_ENDPOINT, resource_id)
                }
                DriveResource::Groups => {
                    format!("{}/groups/{}/drive/root/delta", GRAPH_ENDPOINT, resource_id)
                }
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
    /// drive.reset_access_token("ALS484FJL;ASFJ");
    /// ```
    pub fn reset_access_token(&mut self, access_token: &str) {
        self.access_token = String::from(access_token);
    }

    /// Constructs the url endpoint for a drive item given the drive item resource
    /// name, drive item type, drive id, and item id
    ///
    /// DriveAction must be a mutable reference.
    ///
    /// # Example
    /// ```
    /// use rust_onedrive::drive::{Drive, DriveAction, DriveResource};
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
            }
            DriveAction::GetItem | DriveAction::Delete | DriveAction::Move => {
                Drive::no_drive_action_url(resource, resource_id, item_id)
            }
            _ => Drive::with_drive_action_url(resource, drive_action, resource_id, item_id),
        }
    }

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
        drive_action: DriveAction,
        resource_id: &str,
        item_id: &str,
    ) -> DriveResponse {
        let client = reqwest::Client::builder().build()?;
        let res = client
            .get(
                self.resource_drive_item_url(resource, drive_action, resource_id, item_id)
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
    fn select(&self, end_point: DriveEndPoint, query: &Vec<&str>) -> DriveResponse {
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
        query: &Vec<&str>,
    ) -> DriveResponse {
        let url = self.expand_url(end_point, expand_item, query);
        self.get_with_url(url)
    }

    /// Get the URL string a select query string
    fn select_url(&self, end_point: DriveEndPoint, query: &Vec<&str>) -> String {
        let query_str = query.join(",").clone();
        let url_vec = vec![
            DriveEndPoint::build(end_point).expect("could not build drive end point"),
            "?select=".to_string(),
            query_str,
        ];
        url_vec.join("")
    }

    /// Get the URL string for a expand query string
    fn expand_url(&self, end_point: DriveEndPoint, expand_item: &str, query: &Vec<&str>) -> String {
        let query_str = query.join(",").clone();
        let url_vec = vec![
            DriveEndPoint::build(end_point).expect("could not build drive end point"),
            "?expand=".to_string(),
            expand_item.to_string(),
            "(select=".to_string(),
            query_str,
            String::from(")"),
        ];
        url_vec.join("")
    }
}

// Tests for private field in Drive - must stay here
#[test]
fn drive_access_token() {
    let mut drive = Drive::new("232344");
    assert_eq!(drive.access_token, "232344");
    drive.reset_access_token("3232434");
    assert_eq!(drive.access_token, "3232434");
}
