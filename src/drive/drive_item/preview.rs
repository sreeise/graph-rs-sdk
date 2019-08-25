use std::io::Write;

#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct Preview {
    #[serde(rename = "getUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    get_url: Option<String>,
    #[serde(rename = "postUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    post_url: Option<String>,
    #[serde(rename = "postParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    post_parameters: Option<String>,
}
