#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
pub struct SpecialFolder {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl SpecialFolder {
    pub fn new(name: Option<String>) -> Self {
        SpecialFolder { name }
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }
}
