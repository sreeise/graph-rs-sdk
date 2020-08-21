//! Graph-rs is an API client for Microsoft Graph V1.0 and Graph Beta.
//!
//! If you run into issues related to graph-rs specifically please
//! file an issue on github: https://github.com/sreeise/graph-rs
//!
//! # Example
//! ```rust,ignore
//! use graph_rs::prelude::*;
//!
//! let client = Graph::new("ACCESS_TOKEN");
//!
//! // Use the V1.0 endpoint:
//! let collection = client.v1()
//!     .me()
//!     .drive()
//!     .root_children()
//!     .send()?;
//! pirntln!("{:#?}", collection.value());
//!
//! // Use the Graph beta endpoint.
//! let collection = client.beta()
//!     .me()
//!     .drive()
//!     .root_children()
//!     .send()?;
//! pirntln!("{:#?}", collection.value());
//! ```
//!
//! Choose between me, drives, users, groups, and sites.
//! # Example
//! ```rust,ignore
//! use graph_rs::prelude::*;
//!
//! let client = Graph::new("TOKEN");
//!
//! // Users
//! let response = client.v1()
//!     .users("ID")
//!     .mail()
//!     .messages()
//!     .list()
//!     .send()?;
//! // Collection of messages.
//! println!("{:#?}", response.value());
//!
//! // Groups
//! let response = client.v1()
//!     .groups("ID")
//!     .list_members()
//!     .send()?;
//! // Group members.
//! println!("{:#?}", response.value());
//!
//! ```

#![feature(type_alias_impl_trait)]
#![feature(async_closure)]

extern crate graph_error;
extern crate graph_oauth;
extern crate log;
extern crate pretty_env_logger;
extern crate reqwest;
pub extern crate serde;
pub extern crate serde_derive;
pub extern crate serde_json;
pub extern crate serde_yaml;
extern crate strum;
extern crate strum_macros;
#[macro_use]
extern crate url_serde;
#[macro_use]
extern crate getset;
extern crate handlebars;

// mod client needs to stay on type
// for macro use.
/// Main Graph client.
#[macro_use]
pub mod client;
// Activities request client.
pub mod activities;
/// Calendar request client.
pub mod calendar;
/// Contacts request client.
pub mod contacts;
/// OneDrive request client.
pub mod drive;
/// Groups request client.
pub mod groups;
/// Various internal http helpers.
pub mod http;
/// Mail request client.
pub mod mail;
/// OneNote request client.
pub mod onenote;
// Attachment request client.
pub mod attachments;
/// Types used crate wide.
pub mod types;
/// Url type for graph-rs.
pub mod url;

pub static GRAPH_URL: &str = "https://graph.microsoft.com/v1.0";
pub static GRAPH_URL_BETA: &str = "https://graph.microsoft.com/beta";

/// Common structs and traits.
pub mod prelude {
    pub use crate::client::*;
    pub use crate::http::GraphResponse;
    pub use crate::types::{collection::Collection, delta::*};
}

/// Reexport of graph-oauth crate.
pub mod oauth {
    pub use graph_oauth::jwt;
    pub use graph_oauth::oauth::*;
}

/// Reexport of graph-error crate.
pub mod error {
    pub use graph_error::*;
}

/// Reexport of reqwest headers for use with API requests.
pub mod header {
    pub use reqwest::header::*;
}
