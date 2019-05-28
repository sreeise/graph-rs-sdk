#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
pub struct Group {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
}

impl Group {
    pub fn new(id: Option<String>, display_name: Option<String>) -> Self {
        Group { id, display_name }
    }

    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn display_name(&self) -> Option<String> {
        self.display_name.clone()
    }
}
