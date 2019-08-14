use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/uploadsession?view=odsp-graph-online
#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct UploadSession {
    #[serde(rename = "uploadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_url: Option<String>,
    #[serde(rename = "expirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_date_time: Option<String>,
    #[serde(rename = "nextExpectedRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    next_expected_ranges: Option<Vec<String>>,
}
