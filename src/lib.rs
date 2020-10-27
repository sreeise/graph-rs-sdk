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
#[macro_use]
pub extern crate serde_derive;
extern crate handlebars;
pub extern crate serde_json;
pub extern crate serde_yaml;
extern crate strum;
extern crate strum_macros;

// mod client needs to stay on type
// for macro use.
/// Main Graph client.
#[macro_use]
pub mod client;
/// Activities request client.
pub mod activities;
/// App catalogs request client.
pub mod app_catalogs;
/// Audit logs request client.
pub mod audit_logs;
// Applications request client.
pub mod applications;
// Attachment request client.
pub mod attachments;
/// Calendar request client.
pub mod calendar;
/// Certificate based auth configuration client.
mod certificate_based_auth_configuration;
/// Communications client.
mod communications;
/// Contacts request client.
pub mod contacts;
/// Contracts request client.
pub mod contracts;
/// Data policy operations request client.
pub mod data_policy_operations;
/// Device app management request client.
pub mod device_app_management;
/// Device management request client.
pub mod device_management;
/// Directory request client.
pub mod directory;
/// Domain dns records request client.
pub mod domain_dns_records;
/// Domains request client.
pub mod domains;
/// OneDrive request client.
pub mod drive;
/// Education request client.
pub mod education;
/// Group lifecycle policies request client.
pub mod group_lifecycle_policies;
/// Groups request client.
pub mod groups;
/// Identity request client.
pub mod identity;
/// Invitations request client.
pub mod invitations;
/// Mail request client.
pub mod mail;
/// OneNote request client.
pub mod onenote;
/// Places request client.
pub mod places;
/// Planner request client.
pub mod planner;
/// Policies request client.
pub mod policies;
/// Schema extensions request client.
pub mod schema_extensions;
/// Service principles request client.
pub mod service_principals;
/// Sites request client.
pub mod sites;
/// Subscribed skus request client.
pub mod subscribed_skus;
/// Subscriptions request client.
pub mod subscriptions;
/// Teamwork request client.
pub mod teamwork;

pub static GRAPH_URL: &str = "https://graph.microsoft.com/v1.0";
pub static GRAPH_URL_BETA: &str = "https://graph.microsoft.com/beta";

/// Common structs and traits.
pub mod prelude {
    pub use crate::client::*;
    pub use graph_http::types::{Collection, Delta};
    pub use graph_http::GraphResponse;
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
