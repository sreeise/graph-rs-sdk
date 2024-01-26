#[macro_use]
extern crate serde;

mod blocking;
mod client;
mod core;
mod request_components;
mod request_handler;
mod resource_identifier;
mod upload_session;

pub mod url;

/// Traits for http utilities.
pub mod traits;

/// Io utilities for creating directories and files.
pub mod io_tools;

#[allow(unused_imports)]
pub(crate) mod internal {

    pub use crate::client::*;
    pub use crate::core::*;
    pub use crate::io_tools::*;
    pub use crate::request_components::*;
    pub use crate::request_handler::*;
    #[allow(unused_imports)]
    pub use crate::resource_identifier::*;
    pub use crate::traits::*;
    pub use crate::upload_session::*;
    pub use graph_core::http::*;
}

pub mod api_impl {
    pub use crate::blocking::{BlockingClient, BlockingRequestHandler, UploadSessionBlocking};
    pub use crate::client::*;
    pub use crate::core::*;
    pub use crate::request_components::RequestComponents;
    pub use crate::request_handler::{PagingResponse, PagingResult, RequestHandler};
    pub use crate::resource_identifier::{ResourceConfig, ResourceIdentifier};
    pub use crate::traits::{ApiClientImpl, BodyExt, ODataQuery};
    pub use crate::upload_session::UploadSession;
    pub use graph_core::identity::ClientApplication;
    pub use graph_error::{GraphFailure, GraphResult};
}
