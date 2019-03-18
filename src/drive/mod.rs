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
pub mod discovery;

pub use crate::drive::drive_item::*;
pub use crate::drive::driveaction::DriveEvent;
use crate::drive::driveitem::DriveItem;
pub use crate::drive::driveresource::DriveResource;
pub use crate::drive::endpoint::{DriveEndPoint, EP};
pub use crate::drive::item::{Download, Item};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::ErrorKind;
use transform_request::prelude::*;

pub static GRAPH_ENDPOINT: &str = "https://graph.microsoft.com/v1.0";
pub static GRAPH_ENDPOINT_BETA: &str = "https://graph.microsoft.com/beta";
pub type ItemResult<T> = std::result::Result<T, RequestError>;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, FromFile, ToFile)]
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
        if let Some(at) = access_token {
            let at = at.try_borrow_mut();
            if let Ok(rt) = at {
                let token = rt.clone();
                return Ok(Drive::new(token.get_access_token()));
            }
        }

        Err(RequestError::error_kind(
            ErrorKind::InvalidData,
            "OAuth instance missing access token.",
        ))
    }
}

impl Item<DriveItem> for Drive {
    fn token(&self) -> &str {
        self.access_token.as_str()
    }

    fn item(&self, r: &mut Response) -> ItemResult<DriveItem> {
        let drive_item = DriveItem::transform(r)?;
        Ok(drive_item)
    }
}

impl Item<Value> for Drive {
    fn token(&self) -> &str {
        self.access_token.as_str()
    }

    fn item(&self, r: &mut Response) -> ItemResult<Value> {
        let value: Value = r.json()?;
        Ok(value)
    }
}
