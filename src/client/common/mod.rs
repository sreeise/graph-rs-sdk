mod resource_provisioner;

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
