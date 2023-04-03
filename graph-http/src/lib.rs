#[macro_use]
extern crate serde;

mod blocking;
mod client;
mod core;
mod request_components;
mod request_handler;
mod resource_identifier;
mod upload_session;

mod io_tools;
pub mod url;

/// Traits for http utilities.
pub mod traits;

pub(crate) mod internal {
    pub type ReqwestResult = Result<reqwest::Response, reqwest::Error>;
    pub type ReqwestBlockingResult = Result<reqwest::blocking::Response, reqwest::Error>;
    pub use crate::blocking::*;
    pub use crate::client::*;
    pub use crate::core::*;
    pub use crate::io_tools::*;
    pub use crate::request_components::*;
    pub use crate::request_handler::*;
    pub use crate::resource_identifier::*;
    pub use crate::traits::*;
    pub use crate::upload_session::*;
    pub use crate::url::*;
}

pub mod api_impl {
    pub use crate::blocking::{BlockingClient, BlockingRequestHandler, UploadSessionBlocking};
    pub use crate::client::*;
    pub use crate::core::*;
    pub use crate::request_components::RequestComponents;
    pub use crate::request_handler::RequestHandler;
    pub use crate::resource_identifier::{ResourceConfig, ResourceIdentifier};
    pub use crate::traits::{BodyExt, ODataQuery};
    pub use crate::upload_session::UploadSession;
    pub use graph_error::{GraphFailure, GraphResult};

    pub extern crate handlebars;
}
