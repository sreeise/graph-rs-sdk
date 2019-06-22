use crate::drive::Root;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/sitecollection?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct SiteCollection {
    #[serde(rename = "dataLocationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    data_location_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root: Option<Root>,
}
