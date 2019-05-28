#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
pub struct User {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
