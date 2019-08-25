use std::io::Write;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct Quota {
    #[serde(skip_serializing_if = "Option::is_none")]
    deleted: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remaining: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    used: Option<i64>,
}

impl Quota {
    pub fn new(deleted: i64, remaining: i64, state: String, total: i64, used: i64) -> Self {
        Quota {
            deleted: Some(deleted),
            remaining: Some(remaining),
            state: Some(state),
            total: Some(total),
            used: Some(used),
        }
    }
}
