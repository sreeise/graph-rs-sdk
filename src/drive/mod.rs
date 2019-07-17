/*
Microsoft Graph and OneDrive API use OAuth 2.0 for authorization. By completing an OAuth flow,
your app receives an access token that provides access to the Microsoft Graph a particular
set of permissions for a user.

Your app provides the access token in each request, through an HTTP header:

Authorization: bearer {token}
*/

use graph_error::GraphFailure;
use graph_oauth::oauth::OAuth;
use std;

mod drive_item;
mod driveresource;
mod driveurl;
mod endpoint;
mod item;
#[macro_use]
pub mod query_string;
pub mod event;

pub use crate::drive::drive_item::*;
pub use crate::drive::driveresource::{DriveResource, ResourceBuilder};
pub use crate::drive::driveurl::DriveUrl;
pub use crate::drive::endpoint::{DriveEndPoint, EP};
pub use crate::drive::item::{Item, ItemResponse};
use from_to_file::*;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Formatter};

pub static GRAPH_ENDPOINT: &str = "https://graph.microsoft.com/v1.0";
pub static GRAPH_ENDPOINT_BETA: &str = "https://graph.microsoft.com/beta";

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum DriveVersion {
    V1,
    V2,
}

impl AsRef<str> for DriveVersion {
    fn as_ref(&self) -> &str {
        match self {
            DriveVersion::V1 => GRAPH_ENDPOINT,
            DriveVersion::V2 => GRAPH_ENDPOINT_BETA,
        }
    }
}

impl Display for DriveVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Default for DriveVersion {
    fn default() -> Self {
        DriveVersion::V1
    }
}

pub type ItemResult<T> = std::result::Result<T, GraphFailure>;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, FromToFile)]
pub struct Drive {
    access_token: String,
    version: String,
    drive_version: DriveVersion,
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
    pub fn new(access_token: &str, version: DriveVersion) -> Drive {
        Drive {
            access_token: String::from(access_token),
            version: version.to_string(),
            drive_version: version,
        }
    }

    pub fn set_drive_version(&mut self, version: DriveVersion) {
        self.version = version.to_string();
        self.drive_version = version;
    }
}

impl TryFrom<OAuth> for Drive {
    type Error = GraphFailure;

    fn try_from(value: OAuth) -> std::result::Result<Self, Self::Error> {
        let access_token = value.get_access_token();

        if let Some(token) = access_token {
            return Ok(Drive::new(token.get_access_token(), DriveVersion::V1));
        }

        Err(GraphFailure::none_err(
            "OAuth instance missing access token.",
        ))
    }
}

impl Item for Drive {
    fn token(&self) -> &str {
        self.access_token.as_str()
    }

    fn drive_version(&self) -> DriveVersion {
        self.drive_version
    }
}
