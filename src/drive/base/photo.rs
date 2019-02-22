#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Photo {
    #[serde(rename = "takenDateTime")]
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
