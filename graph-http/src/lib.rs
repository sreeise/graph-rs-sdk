#[macro_use]
extern crate derive_builder;
extern crate reqwest;
#[macro_use]
pub extern crate serde;
pub extern crate serde_json;
pub extern crate serde_yaml;

mod async_client;
mod blocking_client;
mod dispatch;
mod download;
mod http_client;
mod into_response;
mod registry;
mod request;
mod resource_identifier;
mod response;
mod response_handler;
mod upload_session;

pub mod byte_range;
pub mod client;
pub mod iotools;
pub mod odata_query;
pub mod traits;
pub mod types;
pub mod url;

pub use async_client::*;
pub use blocking_client::*;
pub use dispatch::*;
pub use download::*;
pub use http_client::*;
pub use into_response::*;
pub use registry::*;
pub use request::*;
pub use resource_identifier::*;
pub use response::*;
pub use response_handler::*;
pub use upload_session::*;

pub mod api_impl {
    pub use crate::client::{ApiClientImpl, Client};
    pub use crate::odata_query::*;
    pub use crate::response_handler::{ResourceConfig, ResponseHandler};
    pub use crate::url::GraphUrl;
    pub use crate::{IntoResponse, RequestClient, ResourceIdentifier};
    pub use graph_error::{GraphFailure, GraphResult};
    pub extern crate handlebars;
}
