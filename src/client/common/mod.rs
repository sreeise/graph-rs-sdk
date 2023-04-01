mod resource_provisioner;

use graph_error::GraphResult;
use graph_http::api_impl::RequestComponents;
pub(crate) use resource_provisioner::*;

pub(crate) fn map_parameters(params: &[&str]) -> serde_json::Value {
    let mut map = serde_json::Map::new();

    for (i, param) in params.iter().enumerate() {
        if i == 0 {
            map.entry("id").or_insert(serde_json::json!(param));
        } else {
            map.entry(&format!("id{}", i + 1))
                .or_insert(serde_json::json!(param));
        }
    }

    serde_json::Value::Object(map)
}

pub(crate) fn map_errors(
    rc_result: GraphResult<RequestComponents>,
    body_result: GraphResult<reqwest::Body>,
) -> GraphResult<(RequestComponents, reqwest::Body)> {
    Ok((rc_result?, body_result?))
}

#[cfg(feature = "blocking")]
pub(crate) fn map_errors_blocking(
    rc_result: GraphResult<RequestComponents>,
    body_result: GraphResult<reqwest::blocking::Body>,
) -> GraphResult<(RequestComponents, reqwest::blocking::Body)> {
    Ok((rc_result?, body_result?))
}
