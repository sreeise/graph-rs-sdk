#![allow(dead_code, unused, unused_imports, clippy::module_inception)]
use graph_rs_sdk::{header::HeaderMap, header::HeaderValue, GraphClient, GraphClientConfiguration};
use http::header::ACCEPT;
use http::HeaderName;
use std::time::Duration;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

fn main() {
    let client_config = GraphClientConfiguration::new()
        .access_token(ACCESS_TOKEN)
        .timeout(Duration::from_secs(30))
        .default_headers(HeaderMap::default());

    let _ = GraphClient::from(client_config);
}

// Custom headers

async fn per_request_headers() {
    let client = GraphClient::new("token");

    let _result = client
        .users()
        .list_user()
        .header(ACCEPT, HeaderValue::from_static("*/*"))
        .header(
            HeaderName::from_static("HeaderName"),
            HeaderValue::from_static("HeaderValue"),
        )
        .send()
        .await;
}
