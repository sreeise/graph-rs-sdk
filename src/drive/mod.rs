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

#[macro_use]
pub mod query_string;

use crate::drive;
pub use crate::drive::drive_item::*;
pub use crate::drive::driveaction::DriveEvent;
pub use crate::drive::driveresource::DriveResource;
pub use crate::drive::endpoint::{DriveEndPoint, EP};
pub use crate::drive::item::{Download, Item};
use crate::process::fileretriever::FileRetriever;
use serde::{Deserialize, Serialize};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use transform_request::prelude::*;

pub static GRAPH_ENDPOINT: &str = "https://graph.microsoft.com/v1.0";
pub static GRAPH_ENDPOINT_BETA: &str = "https://graph.microsoft.com/beta";

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

/// Converts an OAuth instance to a drive.
/// Panics if OAuth does not contain an access token.
///
/// # Better to use Transform over From here.
/// To avoid unnecessary panics use Drive::transform(OAuth)
/// which returns Result<Drive, Error>.
impl From<OAuth> for Drive {
    fn from(oauth: OAuth) -> Self {
        match oauth.get_access_token() {
            Some(t) => Drive::new(t.get_access_token(), DriveVersion::V1),
            None => panic!("Missing Access Token"),
        }
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

impl Item for Drive {
    fn token(&self) -> &str {
        self.token()
    }
}

impl Download for Drive {
    fn download<P: AsRef<Path>>(
        &self,
        directory: P,
        value: &mut drive::value::Value,
    ) -> ItemResult<PathBuf> {
        match value.microsoft_graph_download_url() {
            Some(download_url) => {
                let path_buf = FileRetriever::download(directory, download_url.as_str()).unwrap();
                Ok(path_buf)
            },
            None => {
                let client = reqwest::Client::builder()
                    .redirect(RedirectPolicy::custom(|attempt| {
                        // There should be only 1 redirect to download a drive item.
                        if attempt.previous().len() > 1 {
                            return attempt.too_many_redirects();
                        }

                        if let Some(url) = attempt.url().host_str() {
                            if url.ends_with("1drv.com") {
                                return attempt.follow();
                            }
                        }
                        attempt.stop()
                    }))
                    .build()
                    .map_err(RequestError::from)?;

                let item_id = match value.id() {
                    Some(t) => t,
                    None => {
                        return Err(RequestError::error_kind(
                            ErrorKind::InvalidData,
                            "Drive value is missing item_id or a microsoft_graph_download_url.",
                        ))
                    },
                };

                let url =
                    DriveResource::Me.action_url(None, item_id.as_str(), DriveEvent::Download);
                let res = client
                    .get(url.as_str())
                    .header(header::AUTHORIZATION, self.token())
                    .send()?;

                let path_buf = FileRetriever::download(directory, res.url().as_str())?;
                Ok(path_buf)
            },
        }
    }
}
