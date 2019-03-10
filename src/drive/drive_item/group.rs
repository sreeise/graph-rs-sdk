#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    id: Option<String>,
    #[serde(rename = "displayName")]
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
