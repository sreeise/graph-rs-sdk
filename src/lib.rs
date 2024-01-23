//! # Microsoft Graph API Client in Rust
//! graph-rs-sdk is an API client for Microsoft Graph V1.0 and Graph Beta.
//!
//! Installation and basic usage can be found below and there are extensive examples in the example's directory
//! on [GitHub](https://github.com/sreeise/graph-rs-sdk).
//!
//! ## What APIs are available
//!
//! The APIs available are generated from Microsoft's msgraph-metadata repository which stores OpenApi configs for the
//! Graph API. There may be some requests and/or API not yet included in this project but in general most of them are
//! implemented.
//!
//! ## Feature requests or Bug reports.
//!
//! For bug reports please file an issue on [GitHub](https://github.com/sreeise/graph-rs-sdk)
//! and a response or fix will be given as soon as possible.
//!
//! The [Discussions](https://github.com/sreeise/graph-rs-sdk/discussions) tab on
//! [GitHub](https://github.com/sreeise/graph-rs-sdk/discussions) is enabled so feel free to stop by
//! there with any questions or feature requests as well. For bugs, please file an issue first. Other
//! than that feel free to ask questions, provide tips to others, and talk about the project in general.
//!
//! ## Use
//! The client is async by default and it is recommended to use
//! tokio as the runtime. Tokio is what is used internally and what the project
//! is tested with.
//!
//! ```rust,ignore
//! use graph_rs_sdk::*;
//!
//! #[tokio::main]
//! async fn main() -> GraphResult<()> {
//!     let client =  Graph::new("ACCESS_TOKEN");
//!
//!     let response = client.users()
//!         .list_user()
//!         .send()
//!         .await?;
//!
//!     println!("{response:#?}");
//!
//!     let body: serde_json::Value = response.json().await?;
//!     println!("{body:#?}");
//!
//!     Ok(())
//! }
//! ```
//! ### Using the blocking client
//!
//! The blocking client can be used by calling `into_blocking()` on a request.
//!
//! ```rust,ignore
//! use graph_rs_sdk::*;
//!
//! fn main() -> GraphResult<()> {
//!     let client =  Graph::new("ACCESS_TOKEN");
//!
//!     let response = client.users()
//!         .list_user()
//!         .into_blocking()
//!         .send()?;
//!
//!     println!("{response:#?}");
//!
//!     let body: serde_json::Value = response.json()?;
//!     println!("{body:#?}");
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Use the Graph version one or beta Api
//! v1() refers to the endpoint for version 1 of the Microsoft graph API. You can also
//! use the beta() method which uses the Microsoft graph beta API endpoint or use
//! custom_endpoint() for those graph APIs that have custom endpoints such as in
//! countries or governments with their own endpoint.
//!
//! The Graph client must be mutable in order to change from v1 to beta or a custom endpoint.
//!
//! #### Beta
//! ```rust,ignore
//! use graph_rs_sdk::*;
//!
//! #[tokio::main]
//! async fn main() -> GraphResult<()> {
//!     let mut client =  Graph::new("ACCESS_TOKEN");
//!
//!     let response = client.beta()
//!         .users()
//!         .list_user()
//!         .send()
//!         .await?;
//!
//!     println!("{response:#?}");
//!
//!     let body: serde_json::Value = response.json().await?;
//!     println!("{body:#?}");
//!
//!     Ok(())
//! }
//! ```
//!
//! #### Custom Endpoint
//! ```rust,ignore
//! use graph_rs_sdk::*;
//!
//! #[tokio::main]
//! async fn main() -> GraphResult<()> {
//!     let mut client =  Graph::new("ACCESS_TOKEN");
//!
//!     let response = client.custom_endpoint("https://api.microsoft.com/api")
//!         .users()
//!         .list_user()
//!         .send()
//!         .await?;
//!
//!     println!("{response:#?}");
//!
//!     let body: serde_json::Value = response.json().await?;
//!     println!("{body:#?}");
//!
//!     Ok(())
//! }
//! ```
//!
//! #### Custom endpoint using `use_endpoint()`
//! ```rust,ignore
//! use graph_rs_sdk::*;
//!
//! #[tokio::main]
//! async fn main() -> GraphResult<()> {
//!     let mut client =  Graph::new("ACCESS_TOKEN");
//!     client.use_endpoint("https://graph.microsoft.com");
//!
//!     let response = client
//!         .users()
//!         .list_user()
//!         .send()
//!         .await?;
//!
//!     println!("{response:#?}");
//!
//!     let body: serde_json::Value = response.json().await?;
//!     println!("{body:#?}");
//!
//!     Ok(())
//! }
//! ```
//!
//! - For more information and examples please see the repository on
//! [GitHub](https://github.com/sreeise/graph-rs-sdk)
//! - If you run into issues related to graph-rs specifically please
//! file an issue on [GitHub](https://github.com/sreeise/graph-rs-sdk)
//!
//! # OAuth
//!
//! OAuth client implementing the OAuth 2.0 and OpenID Connect protocols on Microsoft identity platform
//!
//! Purpose built as OAuth client for Microsoft Graph and the [graph-rs-sdk](https://crates.io/crates/graph-rs-sdk) project.
//! This project can however be used outside [graph-rs-sdk](https://crates.io/crates/graph-rs-sdk) as an OAuth client
//! for Microsoft Identity Platform.
//!
//! ### Supported Authorization Flows
//!
//! #### Microsoft OneDrive and SharePoint
//!
//! - [Token Flow](https://learn.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/graph-oauth?view=odsp-graph-online#token-flow)
//! - [Code Flow](https://learn.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/graph-oauth?view=odsp-graph-online#code-flow)
//!
//! #### Microsoft Identity Platform
//!
//! - [Authorization Code Grant](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow)
//! - [Authorization Code Grant PKCE](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow)
//! - [Open ID Connect](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc)
//! - [Implicit Grant](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-implicit-grant-flow)
//! - [Device Code Flow](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code)
//! - [Client Credentials](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow)
//! - [Resource Owner Password Credentials](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth-ropc)
//!
//! # Example
//! ```
//! use graph_rs_sdk::oauth::OAuth;
//! let mut oauth = OAuth::new();
//! oauth
//!     .client_id("<YOUR_CLIENT_ID>")
//!     .client_secret("<YOUR_CLIENT_SECRET>")
//!     .add_scope("files.read")
//!     .add_scope("files.readwrite")
//!     .add_scope("files.read.all")
//!     .add_scope("files.readwrite.all")
//!     .add_scope("offline_access")
//!     .redirect_uri("http://localhost:8000/redirect")
//!     .authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize")
//!     .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
//!     .refresh_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
//!     .response_type("code")
//!     .logout_url("https://login.microsoftonline.com/common/oauth2/v2.0/logout")
//!     .post_logout_redirect_uri("http://localhost:8000/redirect");
//! ```
//! Get the access code for the authorization code grant by sending the user to
//! log in using their browser.
//! ```rust,ignore
//! # use graph_rs_sdk::oauth::OAuth;
//! # let mut oauth = OAuth::new();
//! let mut request = oauth.build().authorization_code_grant();
//! let _ = request.browser_authorization().open();
//! ```
//!
//! The access code will be appended to the url on redirect. Pass
//! this code to the OAuth instance:
//! ```rust,ignore
//! # use graph_rs_sdk::oauth::OAuth;
//! # let mut oauth = OAuth::new();
//! oauth.access_code("<ACCESS CODE>");
//! ```
//!
//! Perform an authorization code grant request for an access token:
//! ```rust,ignore
//! # use graph_rs_sdk::oauth::{AccessToken, OAuth};
//! # let mut oauth = OAuth::new();
//! let mut request = oauth.build().authorization_code_grant();
//!
//! let response = request.access_token().send()?;
//! println!("{:#?}", access_token);
//!
//! if response.status().is_success() {
//!     let mut access_token: AccessToken = response.json()?;
//!
//!     let jwt = access_token.jwt();
//!     println!("{jwt:#?}");
//!
//!     // Store in OAuth to make requests for refresh tokens.
//!     oauth.access_token(access_token);
//! } else {
//!     // See if Microsoft Graph returned an error in the Response body
//!     let result: reqwest::Result<serde_json::Value> = response.json();
//!     println!("{:#?}", result);
//! }
//!
//! ```

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

pub use crate::client::Graph;
pub use graph_error::{GraphFailure, GraphResult};
pub use graph_http::api_impl::{GraphClientConfiguration, ODataQuery};

/// Reexport of graph-oauth crate.
pub mod oauth {
    pub use graph_oauth::jwt;
    pub use graph_oauth::oauth::*;
}

pub mod http {
    pub use graph_http::api_impl::{
        BodyRead, FileConfig, PagingResponse, PagingResult, UploadSession,
    };
    pub use graph_http::traits::{
        AsyncIterator, HttpResponseBuilderExt, HttpResponseExt, ODataDeltaLink, ODataDownloadLink,
        ODataMetadataLink, ODataNextLink, ODataQuery, ResponseBlockingExt, ResponseExt,
        UploadSessionLink,
    };
    pub use reqwest::tls::Version;
    pub use reqwest::{Body, Method};

    pub mod blocking {
        pub use graph_http::api_impl::UploadSessionBlocking;
        pub use reqwest::blocking::Body;
    }
}

/// Reexport of graph-error crate.
pub mod error {
    pub use graph_error::*;
}

/// Reexport of reqwest headers for use with API requests.
pub mod header {
    pub use reqwest::header::*;
}

pub(crate) mod api_default_imports {
    pub(crate) use handlebars::*;
    pub use reqwest::Method;
    pub use url::Url;

    pub use graph_core::resource::ResourceIdentity;
    pub use graph_error::*;
    pub(crate) use graph_http::api_impl::*;

    //pub use crate::client::Graph;
    pub(crate) use crate::client::{map_errors, map_parameters, ResourceProvisioner};
}
