use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/calculatedcolumn?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct CalculatedColumn {
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    formula: Option<String>,
    #[serde(rename = "outputType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    output_type: Option<String>,
}
