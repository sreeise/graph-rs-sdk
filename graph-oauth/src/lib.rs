//! # OAuth client implementing the OAuth 2.0 and OpenID Connect protocols on Microsoft identity platform
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
//! use graph_oauth::oauth::OAuth;
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
//!     .authorization_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize")
//!     .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
//!     .refresh_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
//!     .response_type("code")
//!     .logout_url("https://login.microsoftonline.com/common/oauth2/v2.0/logout")
//!     .post_logout_redirect_uri("http://localhost:8000/redirect");
//! ```
//! Get the access code for the authorization code grant by sending the user to
//! log in using their browser.
//! ```rust,ignore
//! # use graph_oauth::oauth::OAuth;
//! # let mut oauth = OAuth::new();
//! let mut request = oauth.build().authorization_code_grant();
//! let _ = request.browser_authorization().open();
//! ```
//!
//! The access code will be appended to the url on redirect. Pass
//! this code to the OAuth instance:
//! ```
//! # use graph_oauth::oauth::OAuth;
//! # let mut oauth = OAuth::new();
//! oauth.authorization_code("<ACCESS CODE>");
//! ```
//!
//! Perform an authorization code grant request for an access token:
//! ```rust,ignore
//! # use graph_oauth::oauth::{AccessToken, OAuth};
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
//!     let result: reqwest::Result<serde_json::Value> = response.json()?;
//!     println!("{:#?}", result);
//! }
//!
//! ```
#[macro_use]
extern crate strum;
#[macro_use]
extern crate serde;

mod access_token;
mod auth;
mod discovery;
mod grants;
mod id_token;
pub mod jwt;
mod oauth_error;

pub mod identity;

pub mod oauth {
    pub use crate::access_token::AccessToken;
    pub use crate::auth::GrantSelector;
    pub use crate::auth::OAuth;
    pub use crate::auth::OAuthCredential;
    pub use crate::discovery::graph_discovery;
    pub use crate::discovery::jwt_keys;
    pub use crate::discovery::well_known;
    pub use crate::grants::GrantRequest;
    pub use crate::grants::GrantType;
    pub use crate::id_token::IdToken;
    pub use crate::identity::*;
    pub use crate::oauth_error::OAuthError;
    pub use crate::strum::IntoEnumIterator;
}
