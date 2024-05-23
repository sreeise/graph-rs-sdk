#![allow(dead_code, unused, unused_imports)]

#[macro_use]
extern crate serde;

/// Users todos and todos tasks.
mod todos;

/// User specific APIs such as get and create users.
mod user;

#[tokio::main]
async fn main() {}
