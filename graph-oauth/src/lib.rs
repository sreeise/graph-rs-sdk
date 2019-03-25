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
pub mod jwt;
mod oautherror;
mod stdop;

pub mod oauth {
    pub use crate::accesstoken::AccessToken;
    pub use crate::auth::Credential;
    pub use crate::auth::OAuth;
    pub use crate::auth::OAuthCredential;
    pub use crate::discovery::graphdiscovery;
    pub use crate::discovery::jwtkeys;
    pub use crate::discovery::wellknown;
    pub use crate::grants::ClientCredentialsGrant;
    pub use crate::grants::ImplicitGrant;
    pub use crate::oautherror::OAuthError;
}

pub mod op {
    pub use crate::stdop::StdOp;
}
