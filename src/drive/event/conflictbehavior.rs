#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConflictBehavior {
    Fail,
    Replace,
    Rename,
}

impl AsRef<str> for ConflictBehavior {
    fn as_ref(&self) -> &str {
        match self {
            ConflictBehavior::Fail => "fail",
            ConflictBehavior::Replace => "replace",
            ConflictBehavior::Rename => "rename",
        }
    }
}

impl ToString for ConflictBehavior {
    fn to_string(&self) -> String {
        self.as_ref().to_string()
    }
}
