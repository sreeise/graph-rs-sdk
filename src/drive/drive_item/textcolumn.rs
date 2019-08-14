use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/textcolumn?view=odsp-graph-online
#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct TextColumn {
    #[serde(rename = "allowMultipleLines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_multiple_lines: Option<bool>,
    #[serde(rename = "appendChangesToExistingText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    append_changes_to_existing_text: Option<bool>,
    #[serde(rename = "linesForEditing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    lines_for_editing: Option<i64>,
    #[serde(rename = "maxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<i64>,
    #[serde(rename = "textType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    text_type: Option<String>,
}
