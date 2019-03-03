#![feature(custom_attribute)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_builder;

mod accesstoken;
mod auth;
mod encode;
mod granttypes;
mod graphheaders;
pub mod jwt;
mod discovery;
mod oautherror;
mod stdop;

pub mod oauth {
    pub use crate::accesstoken::AccessToken;
    pub use crate::accesstoken::AccessTokenBuilder;
    pub use crate::auth::OAuth;
    pub use crate::auth::OAuthCredential;
    pub use crate::auth::OAuthParam;
    pub use crate::discovery::JWTKeys;
    pub use crate::discovery::SigningKeys;
    pub use crate::oautherror::OAuthError;
}

pub mod op {
    pub use crate::stdop::StdOp;
}

