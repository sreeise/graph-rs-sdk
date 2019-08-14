use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/fieldvalueset?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct FieldValueSet {
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<String>,
    #[serde(rename = "AuthorLookupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    author_lookup_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "Color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<i64>,
}
