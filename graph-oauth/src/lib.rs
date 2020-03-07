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
//! oauth.access_code("<ACCESS CODE>");
//! ```
//!
//! Perform an authorization code grant request for an access token:
//! ```rust,ignore
//! # use graph_oauth::oauth::OAuth;
//! # let mut oauth = OAuth::new();
//! let mut request = oauth.build().authorization_code_grant();
//!
//! let access_token = request.access_token().send().unwrap();
//! println!("{:#?}", access_token);
//! ```
//!

#![feature(vec_remove_item)]
#![feature(try_trait)]
#[macro_use]
extern crate serde_derive;
extern crate strum;
#[macro_use]
extern crate strum_macros;

mod accesstoken;
mod auth;
mod discovery;
mod grants;
mod idtoken;
pub mod jwt;
mod oautherror;

pub mod oauth {
    pub use crate::accesstoken::AccessToken;
    pub use crate::auth::GrantSelector;
    pub use crate::auth::OAuth;
    pub use crate::auth::OAuthCredential;
    pub use crate::discovery::graphdiscovery;
    pub use crate::discovery::jwtkeys;
    pub use crate::discovery::wellknown;
    pub use crate::grants::GrantRequest;
    pub use crate::grants::GrantType;
    pub use crate::idtoken::IdToken;
    pub use crate::oautherror::OAuthError;
    pub use crate::strum::IntoEnumIterator;
}
