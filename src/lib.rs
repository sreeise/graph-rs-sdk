//! graph-rs is an API client for Microsoft Graph V1.0 and Graph Beta.
//!
//! - For more information and examples please see the repository on
//! [GitHub](https://github.com/sreeise/graph-rs)
//! - If you run into issues related to graph-rs specifically please
//! file an issue on [GitHub](https://github.com/sreeise/graph-rs)
//!
//! # Basic Use:
//! ```rust,ignore
//! use graph_rs::prelude::*;
//!
//! let client = Graph::new("ACCESS_TOKEN");
//!
//! // Use the V1.0 endpoint:
//! let response = client.v1()
//!     .me()
//!     .drive()
//!     .root_children()
//!     .send()?;
//! println!("{:#?}", response.body());
//!
//! // Use the Graph beta endpoint.
//! let response = client.beta()
//!     .me()
//!     .drive()
//!     .get_drive()
//!     .send()?;
//! println!("{:#?}", response.body());
//! ```
//!
//! # Using the Async Client
//! ```rust,ignore
//! use graph_rs::prelude::*;
//!
//! let client =  Graph::new_async("ACCESS_TOKEN");
//!
//! // Returns GraphResponse<Collection<serde_json::Value>>
//! let response = client
//!     .v1()
//!     .drives("{drive-id}")
//!     .root()?;
//!
// println!("{:#?}", response.body());
//!
//! ```
//!

#![feature(type_alias_impl_trait)]
#![feature(async_closure)]

extern crate graph_error;
extern crate graph_oauth;
extern crate handlebars;
extern crate log;
extern crate pretty_env_logger;
extern crate reqwest;
pub extern crate serde;
pub extern crate serde_json;
pub extern crate serde_yaml;
extern crate strum;
extern crate strum_macros;

// mod client needs to stay on top of all other
// client mod declarations for macro use.
/// Main Graph client.
#[macro_use]
pub mod client;
/// Activities request client.
pub mod activities;
/// App catalogs request client.
pub mod app_catalogs;
/// Audit logs request client.
pub mod audit_logs;
/// Planner buckets request client.
pub mod buckets;
// Applications request client.
pub mod applications;
/// Calendar request client.
pub mod calendar;
/// Calendar groups client.
pub mod calendar_groups;
/// Calendar view client.
pub mod calendar_view;
/// Communication call records request client.
pub mod call_records;
/// Communication calls request client.
pub mod calls;
/// Certificate based auth configuration client.
pub mod certificate_based_auth_configuration;
/// Communications client.
pub mod communications;
/// Contact folders client (me, users, etc.).
pub mod contact_folders;
/// Contacts request client.
pub mod contacts;
/// Content types request client.
pub mod content_types;
/// Contracts request client.
pub mod contracts;
/// Groups conversations request client.
pub mod conversations;
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
/// Events request client (Calendars).
pub mod events;
/// Extended properties request client.
pub mod extended_properties;
/// Group lifecycle policies request client.
pub mod group_lifecycle_policies;
/// Groups request client.
pub mod groups;
/// Identity request client.
pub mod identity;
// Inference classification client (me, users, etc.).
pub mod inference_classification;
// Insights client (me, users, etc.).
pub mod insights;
/// Instances request client (events and calendarView).
pub mod instances;
/// Invitations request client.
pub mod invitations;
/// Items request client.
pub mod items;
/// Lists request client.
pub mod lists;
/// Mail folders request client.
pub mod mail_folders;
/// Managed devices client.
pub mod managed_devices;
/// Me request client.
pub mod me;
/// Messages request client.
pub mod messages;
/// Notebooks request client.
pub mod notebooks;
/// OneNote request client.
pub mod onenote;
/// Outlook request client.
pub mod outlook;
/// Onenote pages request client.
pub mod pages;
/// Onenote parent notebook request client.
pub mod parent_notebook;
/// Onenote parent section request client.
pub mod parent_section;
/// Onenote parent section group request client.
pub mod parent_section_group;
/// Places request client.
pub mod places;
/// Planner request client.
pub mod planner;
/// Planner plans request client.
pub mod plans;
/// Policies request client.
pub mod policies;
/// Groups threads posts request client.
pub mod posts;
/// Schema extensions request client.
pub mod schema_extensions;
/// Onenote section group request client.
pub mod section_groups;
/// Onenote section request client.
pub mod sections;
/// Service principles request client.
pub mod service_principals;
/// Communications call record sessions request client.
pub mod sessions;
/// Settings request client (me, users, etc.).
pub mod settings;
/// Sites request client.
pub mod sites;
/// Subscribed skus request client.
pub mod subscribed_skus;
/// Subscriptions request client.
pub mod subscriptions;
/// Planner tasks request client.
pub mod tasks;
/// Teams request client.
pub mod teams;
/// Teamwork request client.
pub mod teamwork;
/// Groups thread request client.
pub mod threads;
/// Users request client.
pub mod users;

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

pub mod core {
    pub use graph_core::resource::*;
}
