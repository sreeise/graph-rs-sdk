// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/identity?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
pub struct Application {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
