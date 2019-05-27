#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckIn {
    #[serde(rename = "checkInAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    check_in_as: Option<String>,
    comment: String,
}

impl CheckIn {
    pub fn new(comment: &str, published: bool) -> CheckIn {
        if published {
            CheckIn {
                check_in_as: Some("published".into()),
                comment: comment.into(),
            }
        } else {
            CheckIn {
                check_in_as: None,
                comment: comment.into(),
            }
        }
    }
}
