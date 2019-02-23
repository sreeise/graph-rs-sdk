/*
Microsoft Graph and OneDrive API use OAuth 2.0 for authorization. By completing an OAuth flow,
your app receives an access token that provides access to the Microsoft Graph a particular
set of permissions for a user.

Your app provides the access token in each request, through an HTTP header:

Authorization: bearer {token}
*/

pub mod baseitem;
pub mod endpoint;
pub mod error;
pub mod headers;
pub mod watchevent;
#[macro_use]
pub mod query_string;
pub mod base;
pub mod driveaction;
pub mod driveresource;
pub mod onedrive;

use crate::drive::base::driveinfo::DriveInfo;
use crate::drive::base::driveitem::DriveItem;
use crate::drive::base::value::Value;
use crate::drive::baseitem::BaseItem;
use crate::drive::driveaction::DriveAction;
use crate::drive::driveresource::DriveResource;
use crate::drive::endpoint::DriveEndPoint;
use crate::drive::endpoint::EP;
use crate::drive::error::DriveError;
use crate::drive::error::DriveErrorType;
use crate::drive::query_string::QueryString;

use reqwest::*;
use std;
use std::io;

pub static GRAPH_ENDPOINT: &str = "https://graph.microsoft.com/v1.0";

pub type DriveResponse = std::result::Result<Response, reqwest::Error>;
pub type UrlResult = std::io::Result<String>;

pub trait DriveSerDe<T> {
    fn serialize(item: T) -> io::Result<String>;
    fn serialize_to_file(path: &str, item: T) -> io::Result<()>;
    fn deserialize_from_str(item_str: &str) -> io::Result<T>;
    fn deserialize_from_file(path: &str) -> io::Result<T>;
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
                    .expect("Error with request to microsoft graph");
                Ok(res)
            }
            "POST" => {
                let res = client
                    .post(url)
                    .header(header::AUTHORIZATION, access_token)
                    .header(header::CONTENT_TYPE, content_type)
                    .send()
                    .expect("Error with request to microsoft graph");
                Ok(res)
            }
            _ => unimplemented!(),
        }
    }

    /// A drive request can make a request to any of the end points on the DriveEndPoint enum
    fn request(&mut self, end_point: DriveEndPoint) -> DriveResponse {
        let client = reqwest::Client::builder().build()?;
        client
            .get(DriveEndPoint::build(end_point).as_str())
            .header(header::AUTHORIZATION, self.access_token.as_str())
            .header(header::CONTENT_TYPE, "application/json")
            .send()
    }

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

    fn get_with_url(&self, url: String) -> DriveResponse {
        self.build_request(
            url.as_str(),
            "GET",
            "application/json",
            self.access_token.as_str(),
        )
    }

    fn post_with_url(&self, url: String) -> DriveResponse {
        self.build_request(
            url.as_str(),
            "POST",
            "application/json",
            self.access_token.as_str(),
        )
    }

    #[allow(dead_code)]
    fn req_with_url(url: &str, access_token: &str) -> DriveResponse {
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
            DriveAction::CreateUploadSession => "createUploadSession",
            DriveAction::ListChildren | DriveAction::CreateFolder => "children",
            DriveAction::Preview => "preview",
            DriveAction::Activities => "activities",
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
                    if item_id.ends_with(':') {
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
            }
            DriveAction::GetItem | DriveAction::Delete | DriveAction::Move => {
                Drive::no_drive_action_url(resource, resource_id, item_id)
            }
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

    /// This method takes a response from a call to the drive/graph api returning
    /// a parsed BaseItem<T> with the values returned or a BaseItem<DriveError>
    /// representing the an error response from the api call.
    fn base_item_response<T>(&mut self, response: DriveResponse) -> BaseItem<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        match response {
            Ok(mut t) => self.parse_base_item(&mut t),
            Err(_) => BaseItem::new(
                None,
                Some(DriveError::new(
                    DriveErrorType::BadRequest.as_str(),
                    DriveErrorType::BadRequest,
                    400,
                )),
            ),
        }
    }

    /// Sends a request for the given DriveEndPoint and if possible constructs
    /// and returns BaseItem<T>
    pub fn base_item<T>(&mut self, end_point: DriveEndPoint) -> BaseItem<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let response = self.request(end_point);
        self.base_item_response(response)
    }

    /// Sends a request for the given url and if possible constructs
    /// and returns BaseItem<T>
    pub fn base_item_from_url<T>(&mut self, url: &str) -> BaseItem<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let response = self.get_with_url(url.to_string());
        self.base_item_response(response)
    }

    /// Parses a successful api response into BaseItem<T> where T: Value, DriveItem, or DriveInfo.
    /// Errors return BaseItem<DriveError> representing the an error response from the api call.
    fn parse_base_item<T>(&mut self, response: &mut Response) -> BaseItem<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        if DriveErrorType::is_error(response.status().as_u16()) {
            let drive_error = match DriveErrorType::drive_error(response.status().as_u16()) {
                Some(t) => Some(t),
                None => None,
            };

            BaseItem::new(None, drive_error)
        } else {
            let req = match response.text() {
                Ok(t) => t,
                Err(_) => {
                    return BaseItem::new(
                        None,
                        Some(DriveError::new(
                            DriveErrorType::BadRequest.as_str(),
                            DriveErrorType::BadRequest,
                            400,
                        )),
                    );
                }
            };

            let parsed_req = match json::parse(req.as_str()) {
                Ok(t) => t,
                Err(_) => {
                    return BaseItem::new(
                        None,
                        Some(DriveError::new(
                            DriveErrorType::BadRequest.as_str(),
                            DriveErrorType::BadRequest,
                            400,
                        )),
                    );
                }
            };

            let pretty_str = parsed_req.pretty(1);
            match serde_json::from_str(pretty_str.as_str()) {
                Ok(t) => BaseItem::new(Some(t), None),
                Err(_) => BaseItem::new(
                    None,
                    Some(DriveError::new(
                        DriveErrorType::BadRequest.as_str(),
                        DriveErrorType::BadRequest,
                        400,
                    )),
                ),
            }
        }
    }

    /// Parses calls to the drive/graph api into a prettified JSON string.
    fn req_to_string_pretty(&mut self, endpoint: DriveEndPoint) -> Option<String> {
        let mut drive_req = self
            .request(endpoint)
            .unwrap_or_else(|_| panic!(DriveErrorType::BadRequest.as_str().to_string()));

        if DriveErrorType::is_error(drive_req.status().as_u16()) {
            let error_type = DriveErrorType::from_u16(drive_req.status().as_u16());
            let error_t = error_type.unwrap();
            let drive_error = DriveError {
                error_info: String::from(error_t.as_str()),
                error_type: error_t,
                code: drive_req.status().as_u16(),
            };

            let serialized = match serde_json::to_string(&drive_error) {
                Ok(s) => Some(s),
                Err(_) => None,
            };

            if let Some(drive_error) = serialized {
                match json::parse(&drive_error) {
                    Ok(t) => return Some(t.pretty(1)),
                    Err(_) => return None,
                }
            }

            return None;
        }

        let req = match drive_req.text() {
            Ok(t) => t,
            Err(_) => return None,
        };

        let parsed_req = match json::parse(req.as_str()) {
            Ok(t) => t,
            Err(_) => return None,
        };

        Some(parsed_req.pretty(1))
    }
}

/// Automatically requests the DriveEndPoint given in the function name and returns the struct
/// of that request. The structs may be of different types listed here by function name:
impl EP for Drive {
    /// # Example
    /// ```rust,ignore
    ///    fn drive_me(&mut self) -> BaseItem<DriveInfo>;
    /// ```
    fn drive(&mut self) -> BaseItem<DriveInfo> {
        self.base_item(DriveEndPoint::Drive)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive(&mut self) -> BaseItem<DriveInfo>
    /// ```
    fn drive_me(&mut self) -> BaseItem<DriveInfo> {
        self.base_item(DriveEndPoint::DriveMe)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive_root(&mut self) -> BaseItem<Value>;
    /// ```
    fn drive_root(&mut self) -> BaseItem<Value> {
        self.base_item(DriveEndPoint::DriveRoot)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive_root_me(&mut self) -> BaseItem<Value>;
    /// ```
    fn drive_root_me(&mut self) -> BaseItem<Value> {
        self.base_item(DriveEndPoint::DriveRootMe)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive_root_child(&mut self) -> BaseItem<DriveItem>;
    /// ```
    fn drive_root_child(&mut self) -> BaseItem<DriveItem> {
        self.base_item(DriveEndPoint::DriveRootChild)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn shared_with_me(&mut self) -> BaseItem<DriveItem>
    /// ```
    fn drive_changes(&mut self) -> BaseItem<DriveItem> {
        self.base_item(DriveEndPoint::DriveChanges)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive_recent(&mut self) -> BaseItem<DriveItem>
    /// ```
    fn shared_with_me(&mut self) -> BaseItem<DriveItem> {
        self.base_item(DriveEndPoint::SharedWithMe)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn drive_recent(&mut self) -> BaseItem<DriveItem>
    /// ```
    fn drive_recent(&mut self) -> BaseItem<DriveItem> {
        self.base_item(DriveEndPoint::DriveRecent)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_documents(&mut self) -> BaseItem<Value>
    /// ```
    fn special_documents(&mut self) -> BaseItem<Value> {
        self.base_item(DriveEndPoint::SpecialDocuments)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_documents_child(&mut self) -> BaseItem<DriveItem>
    /// ```
    fn special_documents_child(&mut self) -> BaseItem<DriveItem> {
        self.base_item(DriveEndPoint::SpecialDocumentsChild)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_photos(&mut self) -> BaseItem<Value>
    /// ```
    fn special_photos(&mut self) -> BaseItem<Value> {
        self.base_item(DriveEndPoint::SpecialPhotos)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_photos_child(&mut self) -> BaseItem<DriveItem>
    /// ```
    fn special_photos_child(&mut self) -> BaseItem<DriveItem> {
        self.base_item(DriveEndPoint::SpecialPhotosChild)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_cameraroll(&mut self) -> BaseItem<Value>
    /// ```
    fn special_cameraroll(&mut self) -> BaseItem<Value> {
        self.base_item(DriveEndPoint::SpecialCameraRoll)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_cameraroll_child(&mut self) -> BaseItem<DriveItem>
    /// ```
    fn special_cameraroll_child(&mut self) -> BaseItem<DriveItem> {
        self.base_item(DriveEndPoint::SpecialCameraRollChild)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_approot(&mut self) -> BaseItem<Value>
    /// ```
    fn special_approot(&mut self) -> BaseItem<Value> {
        self.base_item(DriveEndPoint::SpecialAppRoot)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_approot_child(&mut self) -> BaseItem<DriveItem>
    /// ```
    fn special_approot_child(&mut self) -> BaseItem<DriveItem> {
        self.base_item(DriveEndPoint::SpecialAppRootChild)
    }

    /// # Example
    /// ```rust,ignore
    ///     fn special_music(&mut self) -> BaseItem<Value>
    /// ```
    fn special_music(&mut self) -> BaseItem<Value> {
        self.base_item(DriveEndPoint::SpecialMusic)
    }

    /// # Example
    /// ```rust,ignore
    ///    fn special_music_child(&mut self) -> BaseItem<DriveItem>
    /// ```
    fn special_music_child(&mut self) -> BaseItem<DriveItem> {
        self.base_item(DriveEndPoint::SpecialMusicChild)
    }
}

impl QueryString for Drive {
    /// Query String Select
    ///
    /// Calls the drive/graph api with a select Odata query such as:
    ///     "https://graph.microsoft.com/v1.0/drive/root/children?select=name,size"
    ///
    /// The query should be a &Vec<&str> that holds the query parameters the caller
    /// wants to select: &vec!["name", "size"]
    ///
    ///
    /// # Example
    /// ```rust,ignore
    ///
    /// let mut drive = new Drive("ACCESS_TOKEN");
    ///
    /// let base_item = drive.select(DriveEndPoint::Drive, &vec!["name", "size"]);
    /// if !base_item.error.is_some() {
    ///     println!("{:#?}", &base_item); // BaseItem<DriveItem>
    /// } else {
    ///     println!("{:#?}", &base_item.error); // DriveError
    /// }
    /// ```
    fn select(&mut self, end_point: DriveEndPoint, query: &[&str]) -> BaseItem<DriveItem> {
        let url = self.select_url(end_point, query);
        self.base_item_from_url(url.as_str())
    }

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
    fn select_url(&self, end_point: DriveEndPoint, query: &[&str]) -> String {
        let query_str = query.join(",").clone();
        odata_query!(
            DriveEndPoint::build(end_point),
            "?$select=".to_string(),
            query_str.to_string()
        )
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
    /// let base_item: BaseItem<Value> = drive.expand(DriveEndPoint::DriveRoot, "children", query);
    ///
    /// if !base_item.error.is_some() {
    ///     println!("{:#?}", &base_item); // BaseItem<Value>
    /// } else {
    ///     println!("{:#?}", &base_item.error); // DriveError
    /// }
    ///```
    /// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/concepts/optional-query-parameters?view=odsp-graph-online#selecting-properties
    fn expand(
        &mut self,
        end_point: DriveEndPoint,
        expand_item: &str,
        query: &[&str],
    ) -> BaseItem<Value> {
        let url = self.expand_url(end_point, expand_item, query);
        self.base_item_from_url(url.as_str())
    }

    /// Get the URL string for a expand query string
    fn expand_url(&self, end_point: DriveEndPoint, expand_item: &str, query: &[&str]) -> String {
        let query_str = query.join(",").clone();
        odata_query!(
            DriveEndPoint::build(end_point),
            "?expand=".to_string(),
            expand_item.to_string(),
            "(select=".to_string(),
            query_str,
            String::from(")")
        )
    }

    fn filter(&mut self, end_point: DriveEndPoint, query: &[&str]) -> BaseItem<DriveItem> {
        let url = self.filter_url(end_point, query);
        self.base_item_from_url(url.as_str())
    }

    fn filter_url(&self, end_point: DriveEndPoint, query: &[&str]) -> String {
        let query_str = query.join(" ").clone();
        odata_query!(
            DriveEndPoint::build(end_point),
            "?$filter=".to_string(),
            query_str.to_string()
        )
    }

    fn order_by(&mut self, end_point: DriveEndPoint, query_str: &str) -> BaseItem<DriveItem> {
        let url = self.order_by_url(end_point, query_str);
        self.base_item_from_url(url.as_str())
    }

    fn order_by_url(&self, end_point: DriveEndPoint, query_str: &str) -> String {
        odata_query!(
            DriveEndPoint::build(end_point),
            "?$orderby=".to_string(),
            query_str.to_string()
        )
    }

    fn search(&mut self, end_point: DriveEndPoint, query_str: &str) -> BaseItem<DriveItem> {
        let url = self.search_url(end_point, query_str);
        self.base_item_from_url(url.as_str())
    }

    fn search_url(&self, end_point: DriveEndPoint, query_str: &str) -> String {
        odata_query!(
            DriveEndPoint::build(end_point),
            "?$search=".to_string(),
            query_str.to_string()
        )
    }

    fn format_url(&self, end_point: DriveEndPoint, query_str: &str) -> String {
        odata_query!(
            DriveEndPoint::build(end_point),
            "?$format=".to_string(),
            query_str.to_string()
        )
    }

    fn format(&mut self, end_point: DriveEndPoint, query_str: &str) -> BaseItem<DriveItem> {
        let url = self.format_url(end_point, query_str);
        self.base_item_from_url(url.as_str())
    }

    #[allow(unused_variables)]
    fn skip(&self, end_point: DriveEndPoint, query_str: &str) -> DriveResponse {
        unimplemented!();
    }

    #[allow(unused_variables)]
    fn top(&self, end_point: DriveEndPoint, query_str: &str) -> DriveResponse {
        unimplemented!();
    }
}
