extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

pub mod common;
pub mod drive;
pub mod oauth;
pub mod oauthrequest;
pub mod support;

use graph_rs::client::Graph;
use graph_rs::{GRAPH_URL, GRAPH_URL_BETA};

pub fn assert_url_eq<T: AsRef<str>>(client: &Graph, path: T) {
    client.url_ref(|url| {
        assert_eq!(format!("{}{}", GRAPH_URL, path.as_ref()), url.to_string());
    });
}

pub fn assert_url_beta_eq<T: AsRef<str>>(client: &Graph, path: T) {
    client.url_ref(|url| {
        assert_eq!(
            format!("{}{}", GRAPH_URL_BETA, path.as_ref()),
            url.to_string()
        );
    });
}
