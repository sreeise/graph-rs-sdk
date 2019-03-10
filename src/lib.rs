#![feature(associated_type_defaults)]
pub extern crate strum;
#[macro_use]
pub extern crate strum_macros;
#[macro_use]
pub extern crate serde_derive;
pub extern crate base64;
pub extern crate encoding;
pub extern crate graph_error;
pub extern crate graph_oauth;
pub extern crate jsonfile;
pub extern crate reqwest;
pub extern crate serde;
pub extern crate serde_json;

pub mod drive;
pub mod process;

pub mod oauth {
    pub use graph_error::ErrorType;
    pub use graph_error::GraphError;
    pub use graph_error::RequestError;
    pub use graph_oauth::commons;
    pub use graph_oauth::jwt;
    pub use graph_oauth::oauth::*;
    pub use graph_oauth::op;
}
