//! # Example
//! ```rust,ignore
//! # use graph_rs::prelude::*;
//! # let client = Graph::new("");
//! let result = client.v1()
//!   .me()
//!   .mail()
//!   .messages()
//!   .list()
//!   .send();
//!```

mod request;

pub use request::*;
