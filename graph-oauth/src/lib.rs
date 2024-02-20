//! # Microsoft Identity Platform Client
//!
//! Support For OAuth 2.0 and OpenId authorization flows from the Microsoft Identity Platform.
//!
//! Part of the [graph-rs-sdk](https://crates.io/crates/graph-rs-sdk) project on [GitHub](https://crates.io/crates/graph-rs-sdk)
//!
//!
//! # Example ConfidentialClientApplication Authorization Code Flow
//! ```rust
//! use url::Url;
//! use graph_oauth::{AuthorizationCodeCredential, ConfidentialClientApplication};
//!
//! pub fn authorization_url(client_id: &str) -> anyhow::Result<Url> {
//!     Ok(ConfidentialClientApplication::builder(client_id)
//!         .auth_code_url_builder()
//!         .with_redirect_uri(Url::parse("http://localhost:8000/redirect")?)
//!         .with_scope(vec!["user.read"])
//!         .url()?)
//! }
//!
//! pub fn get_confidential_client(authorization_code: &str, client_id: &str, client_secret: &str) -> anyhow::Result<ConfidentialClientApplication<AuthorizationCodeCredential>> {
//!     Ok(ConfidentialClientApplication::builder(client_id)
//!         .with_auth_code(authorization_code)
//!         .with_client_secret(client_secret)
//!         .with_scope(vec!["user.read"])
//!         .with_redirect_uri(Url::parse("http://localhost:8000/redirect")?)
//!         .build())
//! }
//! ```
//! #### Supported Authorization Flows From The Microsoft Identity Platform
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

#[macro_use]
extern crate serde;
#[macro_use]
extern crate strum;
#[macro_use]
extern crate lazy_static;

pub(crate) mod oauth_serializer;

pub(crate) mod identity;

#[cfg(feature = "interactive-auth")]
pub mod interactive;

pub(crate) mod internal {
    pub use crate::oauth_serializer::*;
}

pub mod extensions {
    pub use crate::oauth_serializer::*;
}

pub use crate::identity::*;
pub use graph_core::{crypto::GenPkce, crypto::ProofKeyCodeExchange};
pub use jsonwebtoken::{Header, TokenData};
