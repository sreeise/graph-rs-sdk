#[macro_use]
extern crate derive_builder;
extern crate reqwest;
#[macro_use]
pub extern crate serde;
pub extern crate serde_json;
pub extern crate serde_yaml;

mod file_config;
mod resource_identifier;
mod response_handler;
mod upload_session;

pub mod byte_range;

pub mod client;
pub mod iotools;
pub mod next_link;
pub mod odata_query;
pub mod traits;
pub mod url;

pub use file_config::*;
pub use resource_identifier::*;
pub use response_handler::*;
pub use upload_session::*;

pub mod api_impl {
    pub use crate::client::{ApiClientImpl, Client};
    pub use crate::odata_query::*;
    pub use crate::response_handler::{RequestComponents, ResourceConfig, ResponseHandler};
    pub use crate::traits::{AsBytesMut, BodyExt};
    pub use crate::url::GraphUrl;
    pub use crate::ResourceIdentifier;
    pub use graph_error::{GraphFailure, GraphResult};
    pub extern crate handlebars;
}
