/*
Microsoft Graph and OneDrive API use OAuth 2.0 for authorization. By completing an OAuth flow,
your app receives an access token that provides access to the Microsoft Graph a particular
set of permissions for a user.

Your app provides the access token in each request, through an HTTP header:

Authorization: bearer {token}
*/

mod drive_item;
pub mod driverequest;
pub mod driveurl;
mod endpoint;
pub mod event;
pub mod intoitem;
mod item;
pub mod pipeline;

pub use crate::drive::drive_item::*;
pub use crate::drive::endpoint::{DriveEndPoint, EP};
use crate::drive::item::SelectResource;
pub use crate::drive::item::{ItemCommon, ItemMe, ItemResponse, Request, SelectEventMe};
use crate::drive::pipeline::DataPipeline;
use driveurl::DriveUrl;
use from_to_file::*;
use graph_error::GraphFailure;
use graph_oauth::oauth::OAuth;
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

#[derive(Eq, PartialEq, Serialize, Deserialize, FromToFile)]
pub struct Drive {
    access_token: String,
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
    /// let mut drive = Drive::new("B32484FJL;ASFJ");
    /// ```
    pub fn new(access_token: &str) -> Drive {
        Drive {
            access_token: access_token.into(),
        }
    }

    pub fn v1(&self) -> SelectResource {
        SelectResource::new(DataPipeline::new(
            self.access_token.as_str(),
            DriveUrl::new(DriveVersion::V1),
        ))
    }

    pub fn v2(&self) -> SelectResource {
        SelectResource::new(DataPipeline::new(
            self.access_token.as_str(),
            DriveUrl::new(DriveVersion::V2),
        ))
    }
}

impl TryFrom<OAuth> for Drive {
    type Error = GraphFailure;

    fn try_from(value: OAuth) -> Result<Self, Self::Error> {
        let access_token = value.get_access_token();

        if let Some(token) = access_token {
            return Ok(Drive::new(token.get_access_token()));
        }

        Err(GraphFailure::none_err(
            "OAuth instance missing access token.",
        ))
    }
}

impl TryFrom<&OAuth> for Drive {
    type Error = GraphFailure;

    fn try_from(value: &OAuth) -> Result<Self, Self::Error> {
        let access_token = value.get_access_token();

        if let Some(token) = access_token {
            return Ok(Drive::new(token.get_access_token()));
        }

        Err(GraphFailure::none_err(
            "OAuth instance missing access token.",
        ))
    }
}
