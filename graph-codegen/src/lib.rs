#![recursion_limit = "1024"]
#![feature(trace_macros)]

pub extern crate inflector;
#[macro_use]
pub extern crate serde;
extern crate from_as;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate strum;

pub mod builder;
pub mod error;
pub mod generator;
pub mod macros;
pub mod openapi;
pub mod parser;
pub mod traits;
