#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecialFolder {
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
