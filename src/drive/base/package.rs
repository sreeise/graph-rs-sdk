#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Package {
    #[serde(rename = "type")]
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
