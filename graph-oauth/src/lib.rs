//! # OAuth
//! An authorization and access token API implementing the OAuth 2.0 authorization
//! framework. This version is specifically meant for the OneDrive API V1.0
//! and the Graph beta API.
//!
//! Requires knowing the OAuth grant that is being used
//! to request authorization and access tokens. This is to ensure that
//! the credentials used in requests include only information that is
//! required or optional for that specific grant and not any other. Even
//! if you accidently pass a value, such as a nonce, for a grant type
//! that does not use it, any request that is made will not include the
//! nonce regardless.
//!
//! # Disclaimer
//! Using this API for other resource owners besides Microsoft may work but
//! functionality will more then likely be limited.
//!
//! # Example
//! ```
//! use graph_oauth::oauth::{OAuth, GrantType};
//! let oauth = OAuth::new(GrantType::AuthorizationCode);
//!
//! // or
//! let oauth_token_flow = OAuth::token_flow();
//! let oauth_code_flow = OAuth::code_flow();
//! let oauth_auth_grant = OAuth::authorization_code_grant();
//! let oauth_implicit = OAuth::implicit_grant();
//! let oauth_open_id = OAuth::open_id_connect();
//! ```

#![feature(vec_remove_item)]
#![feature(try_trait)]
#![feature(custom_attribute)]
#[macro_use]
extern crate serde_derive;
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate derive_from_to_file;

mod accesstoken;
mod auth;
mod discovery;
mod grants;
mod idtoken;
pub mod jwt;
mod oautherror;
mod scopes;
mod stdop;

pub mod scope {
    pub use crate::scopes::*;
}

pub mod oauth {
    pub use crate::accesstoken::AccessToken;
    pub use crate::auth::OAuth;
    pub use crate::auth::OAuthCredential;
    pub use crate::discovery::graphdiscovery;
    pub use crate::discovery::jwtkeys;
    pub use crate::discovery::wellknown;
    pub use crate::grants::Grant;
    pub use crate::grants::GrantRequest;
    pub use crate::grants::GrantType;
    pub use crate::idtoken::IdToken;
    pub use crate::oautherror::OAuthError;
    pub use crate::scope;
    pub use crate::scope::Scope;
    pub use crate::strum::IntoEnumIterator;
}

pub mod op {
    pub use crate::stdop::StdOp;
}
