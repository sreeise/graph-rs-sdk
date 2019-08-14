use std::io::Write;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
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
