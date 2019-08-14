use std::io::Write;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
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
}
