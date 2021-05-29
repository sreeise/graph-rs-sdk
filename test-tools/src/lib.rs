extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

pub mod common;
pub mod drive;
mod files;
pub mod oauth;
pub mod oauthrequest;
pub mod support;

pub use files::FileUtils;

use graph_http::BlockingHttpClient;
use graph_rs_sdk::{client::Graph, GRAPH_URL, GRAPH_URL_BETA};

pub fn assert_url_eq<T: AsRef<str>>(client: &Graph<BlockingHttpClient>, path: T) {
    client.url_ref(|url| {
        assert_eq!(format!("{}{}", GRAPH_URL, path.as_ref()), url.to_string());
    });
}

pub fn assert_url_beta_eq<T: AsRef<str>>(client: &Graph<BlockingHttpClient>, path: T) {
    client.url_ref(|url| {
        assert_eq!(
            format!("{}{}", GRAPH_URL_BETA, path.as_ref()),
            url.to_string()
        );
    });
}
