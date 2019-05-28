#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
pub struct Package {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_field: Option<String>,
}

impl Package {
    pub fn new(type_field: Option<String>) -> Self {
        Package { type_field }
    }

    pub fn type_field(&self) -> Option<String> {
        self.type_field.clone()
    }
}
