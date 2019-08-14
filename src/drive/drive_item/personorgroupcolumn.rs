use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/personorgroupcolumn?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct PersonOrGroupColumn {
    #[serde(rename = "allowMultipleSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_multiple_selection: Option<bool>,
    #[serde(rename = "displayAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    display_as: Option<String>,
    #[serde(rename = "chooseFromType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    choose_from_type: Option<String>,
}
