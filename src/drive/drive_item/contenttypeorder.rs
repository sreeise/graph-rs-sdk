use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/contenttypeorder?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct ContentTypeOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<i64>,
}
