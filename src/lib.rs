//! Rust-OneDrive is an API for Microsoft OneDrive V1.0 and Graph Beta.
//! Callers can utilize the oauth module to authenticate users
//! and receive an access token. This access token can then be used
//! in the drive module to make OneDrive resource requests.
//!
//! As an example, pass an access token to request a resource from the OneDrive API and
//! the version of OneDrive that you want to use. V1 is the OneDrive version 1 API and
//! V2 is the Graph API.
//! # Example
//! ```
//! use rust_onedrive::drive::{Drive, DriveVersion};
//! let drive = Drive::new("access_token");
//! ```
//!
//! The graph-oauth crate was created for use with rust-onedrive to
//! authorize users and get access tokens.
//! # Example
//! ```
//! // For better understanding see the examples directory and the graph-oauth crate.
//! use rust_onedrive::oauth::OAuth;
//!
//! let mut oauth = OAuth::new();
//! oauth.client_id("client_id")
//!     .client_secret("client_secret")
//!     .redirect_uri("http://localhost:8000")
//!     .add_scope("Files.Read")
//!     .add_scope("Files.ReadWrite")
//!     .add_scope("Files.Read.All")
//!     .add_scope("Files.ReadWrite.All")
//!     .add_scope("wl.offline_access")
//!     .authorize_url("https://login.live.com/oauth20_authorize.srf?")
//!     .access_token_url("https://login.live.com/oauth20_token.srf");
//! ```
//!
//!
//! Users can then authenticate in the browser using:
//! ```rust,ignore
//! oauth.request_authorization();
//! ```
//! After the user has authenticated they will be redirected to the redirect url
//! given above. Per the client credentials grant a temporary access code will be
//! appended onto the end of redirect url. This code can then be used to request
//! an access token.
//!
//! Currently there is no other way to authenticate and get long term access tokens
//! for OneDrive except through the browser.
//!
//! # Example
//! ```
//! # use rust_onedrive::oauth::OAuth;
//! # let mut oauth = OAuth::new();
//! oauth.access_code("temporary access code");
//!
//! ```
//!
//!
//! The access token can then be requested by calling:
//! ```rust,ignore
//! oauth.request_access_token(),
//! ```
//!
//! Once the access token has been retrieved the drive module
//! can be used to make authenticated requests to the OneDrive v1.0
//! or Graph Beta APIs
//!
//!
//! # Example
//! ```rust,ignore
//! let mut drive: Drive = Drive::try_from(oauth).unwrap();
//! let drive_item: DriveItemCollection = drive.drive_recent().unwrap();
//! println!("{:#?}", drive_item);
//! ```

#![feature(try_trait)]
#![feature(associated_type_defaults)]
pub extern crate strum;
pub extern crate strum_macros;
#[macro_use]
pub extern crate serde_derive;
pub extern crate base64;
pub extern crate encoding;
pub extern crate graph_error;
pub extern crate graph_oauth;
pub extern crate reqwest;
pub extern crate serde;
pub extern crate serde_json;
pub extern crate serde_yaml;
#[macro_use]
pub extern crate derive_from_to_file;
#[macro_use]
extern crate getset;

pub mod drive;
mod io;

pub mod prelude {
    pub use crate::drive::driveitem::DriveItem;
    pub use crate::drive::driveitemcollection::DriveItemCollection;
    pub use crate::drive::driveurl::{DriveUrl, MutateUrl};
    pub use crate::drive::intoitem::IntoItem;
    pub use crate::drive::Drive;
    pub use crate::drive::DriveEndPoint;
    pub use crate::drive::DriveVersion;
    pub use crate::drive::ItemCommon;
    pub use crate::drive::ItemMe;
    pub use crate::drive::ItemResponse;
    pub use crate::drive::ItemResult;
    pub use crate::drive::Request;
    pub use crate::drive::SelectEventMe;
    pub use crate::drive::EP;
    pub use crate::from_to::*;
}

pub mod oauth {
    pub use graph_error::ErrorType;
    pub use graph_error::GraphError;
    pub use graph_oauth::jwt;
    pub use graph_oauth::oauth::*;
    pub use graph_oauth::op;
}

pub mod from_to {
    pub use from_to_file::*;
}

pub mod fetch {
    pub use crate::io::fetch::FetchBuilder;
    pub use crate::io::iotools::IoTools;
}
