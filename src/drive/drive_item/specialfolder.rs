use std::io::Write;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct SpecialFolder {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl SpecialFolder {
    pub fn new(name: Option<String>) -> Self {
        SpecialFolder { name }
    }
}
