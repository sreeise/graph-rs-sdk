//! ### Microsoft Graph API Client in Rust
//! graph-rs-sdk is an API client for Microsoft Graph V1.0 and Graph Beta.
//!
//! Installation and basic usage can be found below and there are extensive examples in the example's directory
//! on GitHub](https://github.com/sreeise/graph-rs).
//!
//! ### What APIs are available
//!
//! The APIs available are generated from Microsoft's msgraph-metadata repository which stores OpenApi configs for the
//! Graph API. There may be some requests and/or API not yet included in this project but in general most of them are
//! implemented.
//!
//! ### Feature requests or Bug reports.
//!
//! For bug reports please file an issue on [GitHub](https://github.com/sreeise/graph-rs)
//! and a response or fix will be given as soon as possible.
//!
//! The [Discussions](https://github.com/sreeise/graph-rs/discussions) tab on
//! [GitHub](https://github.com/sreeise/graph-rs/discussions) is enabled so feel free to stop by
//! there with any questions or feature requests as well. For bugs, please file an issue first. Other
//! than that feel free to ask questions, provide tips to others, and talk about the project in general.
//!
//! ### Use
//! The client is async by default and it is recommended to use
//! tokio as the runtime.
//!
//! ```rust
//! use graph_rs_sdk::prelude::*;
//!
//! let client =  Graph::new("ACCESS_TOKEN");
//! ```
//!
//! #### The send method and Graph types
//! The send() method is the main method for sending a request and returns
//! a Result<reqwest::Response, GraphFailure>
//!
//! # Examples:
//! ```rust,ignore
//! use graph_rs_sdk::prelude::*;
//!
//! let client =  Graph::new("ACCESS_TOKEN");
//!
//! let response = client.me()
//!     .drive()
//!     .get_drive()
//!     .send()
//!     .await
//!     .unwrap();
//!
//! // Print the value returned in the body of the response
//! println!("{:#?}", response.body());
//! ```
//!
//! # Using the Async Client
//!
//! ```rust,ignore
//! use graph_rs_sdk::prelude::*;
//!
//! let client =  Graph::new("ACCESS_TOKEN");
//!
//! let response = client.me()
//!     .drive()
//!     .get_drive()
//!     .send()
//!     .await
//!     .unwrap();
//!
//! println!("{:#?}", response);
//! ```
//!
//! ### Use the Graph version one or beta Api
//! v1() refers to the endpoint for version 1 of the Microsoft graph API. You can also
//! use the beta() method which uses the Microsoft graph beta Api endpoint or use
//! custom_endpoint() for those graph APIs in countries or governments with their own endpoint.
//!
//!
//! ```rust,ignore
//! use graph_rs_sdk::prelude::*;
//!
//! let client =  Graph::new("ACCESS_TOKEN").beta();
//!
//! let _response = client.me()
//!     .get_user()
//!     .send()
//!     .await?;
//! ```
//!
//! - For more information and examples please see the repository on
//! [GitHub](https://github.com/sreeise/graph-rs)
//! - If you run into issues related to graph-rs specifically please
//! file an issue on [GitHub](https://github.com/sreeise/graph-rs)

extern crate handlebars;
extern crate reqwest;
pub extern crate serde;
pub extern crate serde_json;
pub extern crate serde_yaml;
extern crate strum;

// mod client needs to stay on top of all other
// client mod declarations for macro use.
/// Main Graph client.
#[macro_use]
pub(crate) mod client;
pub mod admin;
pub mod agreement_acceptances;
pub mod agreements;
pub mod app_catalogs;
pub mod applications;
pub mod audit_logs;
pub mod authentication_method_configurations;
pub mod authentication_methods_policy;
pub mod batch;
pub mod branding;
pub mod certificate_based_auth_configuration;
pub mod chats;
pub mod communications;
pub mod contracts;
pub mod data_policy_operations;
pub mod default_drive;
pub mod device_app_management;
pub mod device_management;
pub mod directory;
pub mod directory_objects;
pub mod directory_role_templates;
pub mod directory_roles;
pub mod domain_dns_records;
pub mod domains;
pub mod drives;
pub mod education;
pub mod extended_properties;
pub mod group_lifecycle_policies;
pub mod groups;
pub mod identity;
pub mod identity_governance;
pub mod identity_providers;
pub mod invitations;
pub mod me;
pub mod oauth2_permission_grants;
pub mod organization;
pub mod permission_grants;
pub mod places;
pub mod planner;
pub mod policies;
pub mod reports;
pub mod schema_extensions;
pub mod service_principals;
pub mod sites;
pub mod subscribed_skus;
pub mod subscriptions;
pub mod teams;
pub mod teams_templates;
pub mod teamwork;
pub mod users;

pub static GRAPH_URL: &str = "https://graph.microsoft.com/v1.0";
pub static GRAPH_URL_BETA: &str = "https://graph.microsoft.com/beta";

/// Common structs and traits.
pub mod prelude {
    pub use crate::client::*;
    pub use graph_error::{GraphError, GraphFailure, GraphResult};
    pub use graph_http::api_impl::ODataQuery;
}

/// Reexport of graph-oauth crate.
pub mod oauth {
    pub use graph_oauth::jwt;
    pub use graph_oauth::oauth::*;
}

pub mod http {
    pub use graph_http::traits::{
        AsyncIterator, BodyRead, HttpResponseBuilderExt, HttpResponseExt, ODataDeltaLink,
        ODataDownloadLink, ODataMetadataLink, ODataNextLink, ODataQuery, ResponseExt,
        UploadSessionLink,
    };
    pub use graph_http::{ChannelResponse, FileConfig, UploadSession};
}

/// Reexport of graph-error crate.
pub mod error {
    pub use graph_error::*;
}

/// Reexport of reqwest headers for use with API requests.
pub mod header {
    pub use reqwest::header::*;
}

/// Types used across multiple crates.
pub mod core {
    pub use graph_core::resource::*;
}

pub(crate) mod api_default_imports {
    pub(crate) use handlebars::*;
    pub use reqwest::Method;

    pub use graph_error::*;
    pub(crate) use graph_http::api_impl::*;

    pub use crate::client::Graph;
    pub(crate) use crate::client::{map_parameters, ResourceProvisioner};
    pub use crate::core::ResourceIdentity;
}
