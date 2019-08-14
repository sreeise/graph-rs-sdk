use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/identity?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
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
}
