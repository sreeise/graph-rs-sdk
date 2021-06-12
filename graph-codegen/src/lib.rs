#![recursion_limit = "1024"]

pub extern crate inflector;
#[macro_use]
extern crate serde;
extern crate from_as;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate strum;

pub mod traits;

pub mod builder;
pub mod generator;
pub mod macros;
pub mod openapi;
pub mod parser;
