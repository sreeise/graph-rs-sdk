use std::io::Write;

#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct Package {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_field: Option<String>,
}

impl Package {
    pub fn new(type_field: Option<String>) -> Self {
        Package { type_field }
    }
}
