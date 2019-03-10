#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    id: Option<String>,
}

impl Application {
    pub fn new(display_name: Option<String>, id: Option<String>) -> Self {
        Application { display_name, id }
    }

    pub fn display_name(&self) -> Option<String> {
        self.display_name.clone()
    }

    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }
}
