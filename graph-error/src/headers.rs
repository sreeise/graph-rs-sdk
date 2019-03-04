#[allow(dead_code)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeaderInfo {
    url: String,
    status: i64,
    headers: Headers,
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Headers {
    #[serde(rename = "cache-control")]
    cache_control: String,
    #[serde(rename = "transfer-encoding")]
    transfer_encoding: String,
    #[serde(rename = "content-type")]
    content_type: String,
    date: String,
    vary: String,
    #[serde(rename = "request-id")]
    request_id: String,
    #[serde(rename = "client-request-id")]
    client_request_id: String,
    #[serde(rename = "x-ms-ags-diagnostic")]
    x_ms_ags_diagnostic: String,
    #[serde(rename = "odata-version")]
    odata_version: String,
    duration: String,
    #[serde(rename = "strict-transport-security")]
    strict_transport_security: String,
}
