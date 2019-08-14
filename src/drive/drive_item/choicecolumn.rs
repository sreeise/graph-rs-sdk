use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/choicecolumn?view=odsp-graph-online
#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct ChoiceColumn {
    #[serde(rename = "allowTextEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_text_entry: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    choices: Option<Vec<String>>,
    #[serde(rename = "displayAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    display_as: Option<String>,
}
