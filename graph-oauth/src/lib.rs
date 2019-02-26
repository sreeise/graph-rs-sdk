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
mod jwt;
mod oautherror;
mod stdop;

pub mod oauth {
    pub use crate::accesstoken::AccessToken;
    pub use crate::accesstoken::AccessTokenBuilder;
    pub use crate::auth::OAuth;
    pub use crate::auth::OAuthCredential;
    pub use crate::auth::OAuthParam;
    pub use crate::jwt::JWTKeys;
    pub use crate::jwt::SigningKeys;
    pub use crate::oautherror::OAuthError;
}
