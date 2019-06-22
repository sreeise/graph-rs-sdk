// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/defaultcolumnvalue?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct DefaultColumnValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    formula: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}
