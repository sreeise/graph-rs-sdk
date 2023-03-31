extern crate reqwest;
#[macro_use]
pub extern crate serde;
pub extern crate serde_json;
pub extern crate serde_yaml;

mod byte_range;
mod client;
mod file_config;
mod request_components;
mod resource_identifier;
mod response_handler;
mod upload_session;

pub mod iotools;
pub mod traits;
pub mod url;

pub(crate) mod internal {
    pub use crate::client::*;
    pub use crate::file_config::*;
    pub use crate::iotools::*;
    pub use crate::request_components::*;
    pub use crate::resource_identifier::*;
    pub use crate::response_handler::*;
    pub use crate::traits::*;
    pub use crate::upload_session::*;
    pub use crate::url::*;
}

pub mod api_impl {
    pub use crate::client::{ApiClientImpl, Client, GraphClientBuilder};
    pub use crate::file_config::*;
    pub use crate::request_components::RequestComponents;
    pub use crate::resource_identifier::{ResourceConfig, ResourceIdentifier};
    pub use crate::response_handler::{ChannelResponse, RequestHandler};
    pub use crate::traits::{BodyExt, BodyRead, ODataQuery};
    pub use crate::upload_session::UploadSession;
    pub use crate::url::GraphUrl;
    pub use graph_error::{GraphFailure, GraphResult};

    pub extern crate handlebars;
}
