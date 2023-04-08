//! Various errors used to implement the Graph API and OAuth clients for the graph-rs project
//! See the project on [GitHub](https://github.com/sreeise/graph-rs-sdk).
#[macro_use]
extern crate serde;

pub mod download;
mod error;
mod graph_failure;
mod internal;
pub mod io_error;

pub use error::*;
pub use graph_failure::*;
pub use internal::*;

pub type GraphResult<T> = Result<T, GraphFailure>;
