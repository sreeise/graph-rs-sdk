//! Rust-OneDrive is an API for Microsoft OneDrive V1.0 and Graph Beta.
//! Callers can utilize the oauth module to authenticate users
//! and receive an access token. This access token can then be used
//! in the drive module to make OneDrive resource requests.
//!
//! The Drive struct is the main OneDrive impl and provides the v1()
//! and v2() methods used for selecting the specific API endpoint
//! to use for the next request. The v1() method uses the stable Onedrive
//! V1.0 API while the v2() method can be used for making requests using the
//! graph beta API endpoint.
//!
//! # Example
//! ```rust,ignore
//! // To use the V1.0 endpoint:
//! let mut req = drive.v1().drive_recent();
//! let collection: Collection<DriveItem> = req.send().unwrap();
//! pirntln!("{:#?}", collection);
//!
//! // Use the Graph beta endpoint.
//! let mut req = drive.v2().drive_recent();
//! let collection: Collection<DriveItem> = req.send().unwrap();
//! pirntln!("{:#?}", collection);
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
//! oauth.client_id("<CLIENT ID>")
//!     .client_secret("<CLIENT SECRET>")
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
//! Users can then authenticate in the browser using a selected
//! grant type.
//! ```rust,ignore
//! let mut oauth_code_grant = oauth.build().authorization_code_grant();
//!
//! // Opens the users default browser to the Microsoft login page.
//! let mut req = oauth_code_grant.browser_authorization();
//! ```
//! After the user has authenticated they will be redirected to the redirect url
//! given above. In this instance  a temporary access code will be appended onto
//! the end of redirect url. This code can then be used to request
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
//! ```
//!
//! The access token can then be requested by selecting an OAuth
//! grant type. The authorization code grant is shown here.
//! ```rust,ignore
//! let mut oauth_code_grant = oauth.build().authorization_code_grant();
//! let req = oauth_code_grant.access_token();
//! let access_token = oauth.send().unwrap();
//! println!("{:#?}", access_token);
//! ```
//!
//! Once the access token has been retrieved the drive module
//! can be used to make authenticated requests to the OneDrive v1.0
//! or Graph Beta APIs
//!
//! # Example
//! ```rust,ignore
//! let mut drive = Drive::try_from(oauth).unwrap();
//! let req = drive.v1().drive_recent();
//! let drive_item: DriveItemCollection = req.send().unwrap();
//! println!("{:#?}", drive_item);
//! ```
//!
//! You can also select a specific resource: me, drives, sites, or
//! users.
//! # Example
//! ```rust,ignore
//! let mut req = drive.v1().me().get_item("ITEM ID");
//! let drive_item = req.send()?;
//! println!("{:#?}", drive_item);
//!
//! // Or get the item by path
//! let mut req = drive.v1().me().get_item_path("Documents/file.txt");
//! let drive_item = req.send()?;
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

/// The main drive module used for making requests
/// to the OneDrive V1.0 and Graph Beta endpoints.
pub mod drive;
pub mod io;

/// Common structs and traits.
pub mod prelude {
    pub use crate::drive::drive_item::collection::Collection;
    pub use crate::drive::drive_item::driveitem::DriveItem;
    pub use crate::drive::driveurl::{DriveUrl, MutateUrl};
    pub use crate::drive::endpoint::{DriveEndPoint, EP};
    pub use crate::drive::item::{AsEvent, IntoItem, SelectEventMe};
    pub use crate::drive::pipelines::downloadpipeline::MutateDownload;
    pub use crate::drive::pipelines::request::{ReqBuilder, Request};
    pub use crate::drive::{Drive, ItemResult};
    pub use crate::from_to::*;
}

/// Reexport of graph-oauth crate.
pub mod oauth {
    pub use graph_oauth::jwt;
    pub use graph_oauth::oauth::*;
    pub use graph_oauth::op;
}

/// The FromToFile trait provides writing JSON and Yaml to
/// and from files. The mod exposes the traits needed for
/// the FromToFile trait.
pub mod from_to {
    pub use from_to_file::*;
}

/// Tools for working with and downloading files.
pub mod fetch {
    pub use crate::io::fetch::FetchBuilder;
    pub use crate::io::iotools::IoTools;
}
