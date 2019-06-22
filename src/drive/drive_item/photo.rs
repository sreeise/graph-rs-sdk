#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
pub struct Photo {
    #[serde(rename = "takenDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    taken_date_time: Option<String>,
}

impl Photo {
    pub fn new(taken_date_time: Option<String>) -> Self {
        Photo { taken_date_time }
    }
}

impl Photo {
    pub fn taken_date_time(&self) -> Option<String> {
        self.taken_date_time.clone()
    }
}
