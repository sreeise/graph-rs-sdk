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
mod driveaction;
mod driveresource;
mod endpoint;
mod item;
mod pathbuilder;
#[macro_use]
pub mod query_string;

pub use crate::drive::drive_item::*;
pub use crate::drive::driveaction::{DownloadFormat, DriveEvent, DriveItemCopy, EventProgress};
pub use crate::drive::driveresource::{DriveResource, ResourceBuilder};
pub use crate::drive::endpoint::{DriveEndPoint, EP};
pub use crate::drive::item::{Item, ItemResponse};
pub use crate::drive::pathbuilder::PathBuilder;
use from_to_file::*;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::ErrorKind;

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

pub type ItemResult<T> = std::result::Result<T, GraphFailure>;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, FromToFile)]
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

    pub fn get_version(&self) -> &String {
        &self.version
    }

    fn token(&self) -> &str {
        self.access_token.as_str()
    }
}

impl TryFrom<OAuth> for Drive {
    type Error = GraphFailure;

    fn try_from(value: OAuth) -> std::result::Result<Self, Self::Error> {
        let access_token = value.get_access_token();

        if let Some(token) = access_token {
            return Ok(Drive::new(token.get_access_token(), DriveVersion::V1));
        }

        Err(GraphFailure::error_kind(
            ErrorKind::InvalidData,
            "OAuth instance missing access token.",
        ))
    }
}

impl Item for Drive {
    fn token(&self) -> &str {
        self.token()
    }

    fn drive_version(&self) -> DriveVersion {
        if self.version.eq(GRAPH_ENDPOINT) {
            DriveVersion::V1
        } else {
            DriveVersion::V2
        }
    }
}
