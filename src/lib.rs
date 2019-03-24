//! Rust-OneDrive is an API for Microsoft OneDrive V1.0 and Graph Beta.
//! Callers can utilize the oauth module to authenticate users
//! and receive an access token. This access token can then be used
//! in the drive module to make OneDrive resource requests.
//!
//! # Example
//! ```
//! // For better understanding see the examples directory.
//! use rust_onedrive::oauth::OAuth;
//! use rust_onedrive::oauth::ClientCredentialsGrant;
//!
//! let mut oauth = OAuth::new();
//! oauth.client_id("client_id")
//!     .client_secret("client_secret")
//!     .redirect_url("http://localhost:8000")
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
//! oauth.browser_sign_in().unwrap();
//! After the user has authenticated they will be redirected to the redirect url
//! given above. Per the client credentials grant a temporary access code will be
//! appended onto the end of redirect url. This code can then be used to request
//! an access token.
//!
//! Currently there is no other way to authenticate and get long term access tokens
//! for OneDrive except through the browser.
//!
//!
//!
//! # Exmaple
//! ```rust,ignore
//! // This will use the url given for the authorize_url() method above
//! // and open it in the users default browser. For a good exmaple
//! // see examples/rocket_example.rs
//!
//! oauth.browser_sign_in().unwrap();
//! ```
//!
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
//! request_access_token(),
//!
//! Once the access token has been retrieved the drive module
//! can be used to make authenticated requests to the OneDrive v1.0
//! or Graph Beta APIs
//!
//!
//! # Example
//! ```rust,ignore
//! use rust_onedrive::Drive
//! oauth.request_access_token().unwrap();
//!
//! let mut drive = Drive::from(oauth).unwrap();
//! let drive_item: DriveItem = drive.recent().unwrap();
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
#[macro_use]
pub extern crate derive_from_to_file;

pub mod drive;
pub mod process;

pub mod oauth {
    pub use graph_error::ErrorType;
    pub use graph_error::GraphError;
    pub use graph_oauth::jwt;
    pub use graph_oauth::oauth::*;
    pub use graph_oauth::op;
}

pub mod transform {
    pub use transform_request::prelude::*;
}
