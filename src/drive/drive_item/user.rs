#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    id: Option<String>,
    email: Option<String>,
}

impl User {
    pub fn new(display_name: Option<String>, id: Option<String>, email: Option<String>) -> Self {
        User {
            display_name,
            id,
            email,
        }
    }

    pub fn display_name(&self) -> Option<String> {
        self.display_name.clone()
    }

    pub fn id(&self) -> Option<String> {
        self.display_name.clone()
    }

    pub fn email(&self) -> Option<String> {
        self.email.clone()
    }
}
