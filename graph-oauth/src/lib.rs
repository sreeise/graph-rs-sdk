//! # OAuth client implementing the OAuth 2.0 and OpenID Connect protocols on Microsoft identity platform
//!
//! Purpose built as OAuth client for Microsoft Graph and the [graph-rs-sdk](https://crates.io/crates/graph-rs-sdk) project.
//! This project can however be used outside [graph-rs-sdk](https://crates.io/crates/graph-rs-sdk) as an OAuth client
//! for Microsoft Identity Platform.
//!
//! ### Supported Authorization Flows
//!
//! #### Microsoft Identity Platform
//!
//! - [Authorization Code Grant](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow)
//! - [Authorization Code Grant PKCE](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow)
//! - [Authorization Code Certificate](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow#request-an-access-token-with-a-certificate-credential)
//! - [Open ID Connect](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc)
//! - [Implicit Grant](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-implicit-grant-flow)
//! - [Device Code Flow](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code)
//! - [Client Credentials - Client Secret](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow#first-case-access-token-request-with-a-shared-secret)
//! - [Client Credentials - Client Certificate](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow#second-case-access-token-request-with-a-certificate)
//! - [Resource Owner Password Credentials](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth-ropc)
//!
//! #### Microsoft OneDrive and SharePoint
//!
//! Can only be used with personal Microsoft accounts. Not recommended - use the Microsoft
//! Identity Platform if at all possible.
//!
//! - [Token Flow](https://learn.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/graph-oauth?view=odsp-graph-online#token-flow)
//! - [Code Flow](https://learn.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/graph-oauth?view=odsp-graph-online#code-flow)
//!
//!
//! # Example ConfidentialClientApplication Authorization Code Flow
//! ```rust
//! use url::Url;
//! use graph_error::IdentityResult;
//! use graph_oauth::identity::{AuthorizationCodeCredential, ConfidentialClientApplication};
//!
//! pub fn authorization_url(client_id: &str) -> IdentityResult<Url> {
//!     ConfidentialClientApplication::builder(client_id)
//!         .auth_code_authorization_url_builder()
//!         .with_redirect_uri("http://localhost:8000/redirect")
//!         .with_scope(vec!["user.read"])
//!         .url()
//! }
//!
//! pub fn get_confidential_client(authorization_code: &str, client_id: &str, client_secret: &str) -> anyhow::Result<ConfidentialClientApplication<AuthorizationCodeCredential>> {
//!     Ok(ConfidentialClientApplication::builder(client_id)
//!         .with_authorization_code(authorization_code)
//!         .with_client_secret(client_secret)
//!         .with_scope(vec!["user.read"])
//!         .with_redirect_uri("http://localhost:8000/redirect")?
//!         .build())
//! }
//! ```

#[macro_use]
extern crate serde;
#[macro_use]
extern crate strum;

mod auth;
mod discovery;
mod grants;
pub mod jwt;
mod oauth_error;

pub mod identity;

#[cfg(feature = "interactive-auth")]
pub(crate) mod web;

pub(crate) mod internal {
    pub use graph_core::http::*;
}

pub mod oauth {
    pub use graph_extensions::{crypto::GenPkce, crypto::ProofKeyCodeExchange};

    pub use crate::auth::OAuthParameter;
    pub use crate::auth::OAuthSerializer;
    pub use crate::discovery::graph_discovery;
    pub use crate::discovery::jwt_keys;
    pub use crate::identity::*;
    pub use crate::oauth_error::OAuthError;
    pub use crate::strum::IntoEnumIterator;
}
