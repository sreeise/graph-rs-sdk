use crate::drive::drive_item::thumbnail::ThumbnailSet;
use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/identity?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct Identity {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnails: Option<Vec<ThumbnailSet>>,
}
