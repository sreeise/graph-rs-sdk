#![recursion_limit = "1024"]
#![allow(deprecated)]
pub extern crate inflector;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate serde;
extern crate from_as;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate strum;

pub mod api_types;
pub mod filter;
pub mod macros;
pub mod openapi;
pub mod parser;
pub mod settings;
pub mod traits;
