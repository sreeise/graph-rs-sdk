#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
pub struct FileSystemInfo {
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_date_time: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_date_time: Option<String>,
}

impl FileSystemInfo {
    pub fn new(created_date_time: Option<String>, last_modified_date_time: Option<String>) -> Self {
        FileSystemInfo {
            created_date_time,
            last_modified_date_time,
        }
    }

    pub fn created_date_time(&self) -> Option<String> {
        self.created_date_time.clone()
    }

    pub fn last_modified_time(&self) -> Option<String> {
        self.last_modified_date_time.clone()
    }
}
