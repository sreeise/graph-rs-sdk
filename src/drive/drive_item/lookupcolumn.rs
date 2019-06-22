// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/lookupcolumn?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct LookupColumn {
    #[serde(rename = "allowMultipleValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_multiple_values: Option<bool>,
    #[serde(rename = "allowUnlimitedLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_unlimited_length: Option<bool>,
    #[serde(rename = "columnName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    column_name: Option<String>,
    #[serde(rename = "listId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    list_id: Option<String>,
    #[serde(rename = "primaryLookupColumnId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_lookup_column_id: Option<String>,
}
