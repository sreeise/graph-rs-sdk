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
mod encode;
mod grants;
mod idtoken;
pub mod jwt;
mod oautherror;
mod stdop;

pub mod oauth {
    pub use crate::accesstoken::AccessToken;
    pub use crate::auth::OAuth;
    pub use crate::auth::OAuthCredential;
    pub use crate::discovery::graphdiscovery;
    pub use crate::discovery::jwtkeys;
    pub use crate::discovery::wellknown;
    pub use crate::encode::EncodeBuilder;
    pub use crate::encode::Encoder;
    pub use crate::grants::AuthorizationCodeGrant;
    pub use crate::grants::ClientCredentialsGrant;
    pub use crate::grants::CodeFlow;
    pub use crate::grants::GrantRequest;
    pub use crate::grants::GrantType;
    pub use crate::grants::ImplicitGrant;
    pub use crate::grants::OpenIdConnect;
    pub use crate::grants::TokenFlow;
    pub use crate::idtoken::IdToken;
    pub use crate::oautherror::OAuthError;
}

pub mod op {
    pub use crate::stdop::StdOp;
}
