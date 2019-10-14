//! # Example
//! ```rust,ignore
//! # use graph_rs::prelude::*;
//! # let client = Graph::new("");
//! let result = client.v1()
//!     .me()
//!     .drive()
//!     .root_children()
//!     .send();
//! ```

mod request;

pub use request::*;
