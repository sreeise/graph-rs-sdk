/*
Microsoft Graph and OneDrive API use OAuth 2.0 for authorization. By completing an OAuth flow,
your app receives an access token that provides access to the Microsoft Graph a particular
set of permissions for a user.

Your app provides the access token in each request, through an HTTP header:

Authorization: bearer {token}
*/

use graph_oauth::oauth::OAuth;
use reqwest::*;
use std;
use transform_request::RequestError;

mod drive_item;
mod driveaction;
mod driveresource;
mod endpoint;
mod item;
mod pathbuilder;
#[macro_use]
pub mod query_string;

use crate::drive;
pub use crate::drive::drive_item::*;
pub use crate::drive::driveaction::{DownloadFormat, DriveEvent};
pub use crate::drive::driveresource::DriveEventPath;
pub use crate::drive::driveresource::DriveResource;
pub use crate::drive::endpoint::{DriveEndPoint, EP};
pub use crate::drive::item::{Download, Item};
pub use crate::drive::pathbuilder::PathBuilder;
use crate::fetch::Fetch;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use transform_request::prelude::*;

pub static GRAPH_ENDPOINT: &str = "https://graph.microsoft.com/v1.0";
pub static GRAPH_ENDPOINT_BETA: &str = "https://graph.microsoft.com/beta";

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum DriveVersion {
    V1,
    V2,
}

impl ToString for DriveVersion {
    fn to_string(&self) -> String {
        match self {
            DriveVersion::V1 => GRAPH_ENDPOINT.to_string(),
            DriveVersion::V2 => GRAPH_ENDPOINT_BETA.to_string(),
        }
    }
}

pub type ItemResult<T> = std::result::Result<T, RequestError>;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, FromFile, ToFile)]
pub struct Drive {
    access_token: String,
    version: String,
}

impl Drive {
    /// Construct a new Drive struct for accessing drive resources
    ///
    /// # Example
    /// ```
    /// use rust_onedrive::drive::{Drive, DriveVersion};
    ///
    ///  // The Drive struct requires the access token to make
    ///  // authenticated requests to microsoft graph.
    ///  // The DriveVersion specifies microsoft API version to use.
    /// let mut drive = Drive::new("B32484FJL;ASFJ", DriveVersion::V1);
    /// ```
    pub fn new(access_token: &str, graph_version: DriveVersion) -> Drive {
        Drive {
            access_token: String::from(access_token),
            version: graph_version.to_string(),
        }
    }

    pub fn version(&mut self, version: DriveVersion) {
        self.version = version.to_string();
    }

    pub fn host(&self) -> String {
        self.version.to_string()
    }

    fn token(&self) -> &str {
        self.access_token.as_str()
    }
}

/// Converts an OAuth instance to a Result<Drive> on success
/// or Result<Error> on failure. An Err is returned if there
/// is no access token in the OAuth instance.
impl Transform<OAuth> for Drive {
    type Err = RequestError;

    fn transform(rhs: OAuth) -> std::result::Result<Self, Self::Err>
    where
        Self: Serialize + for<'de> Deserialize<'de>,
    {
        let access_token = rhs.get_access_token();

        if let Some(token) = access_token {
            return Ok(Drive::new(token.get_access_token(), DriveVersion::V1));
        }

        Err(RequestError::error_kind(
            ErrorKind::InvalidData,
            "OAuth instance missing access token.",
        ))
    }
}

impl Fetch for Drive {}

impl TryFrom<OAuth> for Drive {
    type Error = RequestError;

    fn try_from(value: OAuth) -> std::result::Result<Self, Self::Error> {
        Drive::transform(value)
    }
}

impl Item for Drive {
    fn token(&self) -> &str {
        self.token()
    }
}

impl Download for Drive {
    /// Download files from the OneDrive API.
    ///
    /// # Example
    /// ```rust,ignore
    /// use rust_onedrive::prelude::*;
    ///
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// let drive_item: DriveItem = drive.drive_root_child().unwrap();
    /// let vec: Vec<Value> = drive_item.value().unwrap();
    ///
    /// let mut value = vec
    ///     .iter()
    ///     .find(|s| s.name() == Some("rust.docx"))
    ///     .unwrap()
    ///     .clone();
    ///
    /// drive.download("/home/drive", &mut value).unwrap();
    /// ```
    ///
    /// Requires the directory to download to and the drive::Value to download. The Value
    /// struct stores a download URL that can be used. If the download url is None, then
    /// the item's id (also in the Value struct) is used to download the item.
    ///
    /// # See
    /// [Downloading Drive Items](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_get_content?view=odsp-graph-online)
    fn download<P: AsRef<Path>>(
        &self,
        directory: P,
        value: &mut drive::value::Value,
    ) -> ItemResult<PathBuf> {
        match value.microsoft_graph_download_url() {
            // First check for a download URL in the drive::Value itself, If found use this
            // to download the file.
            Some(download_url) => Ok(self.fetch(directory, download_url.as_str())?),
            // If there is no download URL, then a request to get a download URL
            // will be made. If successful, the request will be redirected to the URL
            // of the download.
            None => {
                let client = reqwest::Client::builder()
                    .redirect(RedirectPolicy::custom(|attempt| {
                        // There should be only 1 redirect to download a drive item.
                        if attempt.previous().len() > 1 {
                            return attempt.too_many_redirects();
                        }
                        attempt.stop()
                    }))
                    .build()
                    .map_err(RequestError::from)?;

                // In order for the OneDrive API to know which item we need, the request
                // must include the id for the item being downloaded: /{item-id}/content
                let item_id = match value.id() {
                    Some(t) => t,
                    None => return Err(RequestError::none_err("Missing item id or download URL")),
                };

                let url = DriveResource::Me.item_resource(item_id.as_str(), DriveEvent::Download);
                let res = client.get(url.as_str()).bearer_auth(self.token()).send()?;
                Ok(self.fetch(directory, res.url().as_str())?)
            },
        }
    }

    /// Download files from the OneDrive API in the format given. The format given
    /// must be one of:
    /// # Example
    /// ```rust,ignore
    /// DownloadFormat::GLB => "glb",
    /// DownloadFormat::HTML => "html",
    /// DownloadFormat::JPG => "jpg",
    /// DownloadFormat::PDF => "pdf",
    /// ```
    ///
    /// # Example
    /// ```rust,ignore
    /// use rust_onedrive::prelude::*;
    ///
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// let drive_item: DriveItem = drive.drive_root_child().unwrap();
    /// let vec: Vec<Value> = drive_item.value().unwrap();
    ///
    /// let mut value = vec
    ///     .iter()
    ///     .find(|s| s.name() == Some("rust.docx"))
    ///     .unwrap()
    ///     .clone();
    ///
    /// drive.download_format("/home/drive", &mut value, DownloadFormat::PDF).unwrap();
    /// ```
    ///
    /// Requires the directory to download, the drive::Value, and the format. The Value
    /// struct stores a download URL that can be used. If the download url is None, then
    /// the item's id (also in the Value struct) is used to download the item.
    ///
    /// # See
    /// [Download Formats](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_get_content_format?view=odsp-graph-online)
    fn download_format<P: AsRef<Path>>(
        &self,
        directory: P,
        value: &mut drive::value::Value,
        format: DownloadFormat,
    ) -> ItemResult<PathBuf> {
        // A formatted download always uses a redirect to get the item.
        let client = reqwest::Client::builder()
            .redirect(RedirectPolicy::custom(|attempt| {
                // There should be only 1 redirect to download a drive item.
                if attempt.previous().len() > 1 {
                    return attempt.too_many_redirects();
                }
                attempt.stop()
            }))
            .build()
            .map_err(RequestError::from)?;

        // In order for the OneDrive API to know which item we need, the request
        // must include the id for the item being downloaded: /{item-id}/content
        let item_id = match value.id() {
            Some(t) => t,
            None => return Err(RequestError::none_err("Missing item id or download URL")),
        };

        let mut url =
            DriveResource::Drives.item_resource(item_id.as_str(), DriveEvent::DownloadAndFormat);
        url.push_str(format.as_ref());
        let res = client.get(url.as_str()).bearer_auth(self.token()).send()?;
        Ok(self.fetch(directory, res.url().as_str())?)
    }
}
