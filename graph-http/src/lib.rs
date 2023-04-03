extern crate reqwest;
#[macro_use]
pub extern crate serde;
pub extern crate serde_json;
pub extern crate serde_yaml;

mod client;
mod core;
mod request_components;
mod request_handler;
mod resource_identifier;
mod upload_session;

pub mod blocking;
pub mod iotools;
pub mod traits;
pub mod url;

pub(crate) mod internal {
    pub use crate::blocking::*;
    pub use crate::client::*;
    pub use crate::core::*;
    pub use crate::iotools::*;
    pub use crate::request_components::*;
    pub use crate::request_handler::*;
    pub use crate::resource_identifier::*;
    pub use crate::traits::*;
    pub use crate::upload_session::*;
    pub use crate::url::*;
}

pub mod api_impl {
    pub use crate::blocking::{BlockingClient, BlockingRequestHandler};
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
