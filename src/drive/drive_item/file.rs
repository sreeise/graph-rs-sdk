use crate::drive::drive_item::hashes::Hashes;
use std::io::Write;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct File {
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hashes: Option<Hashes>,
}

impl File {
    pub fn new(mime_type: Option<String>, hashes: Option<Hashes>) -> Self {
        File { mime_type, hashes }
    }
}
