use crate::drive::drive_item::hashes::Hashes;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
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

impl File {
    pub fn mime_type(&self) -> Option<String> {
        self.mime_type.clone()
    }

    pub fn hashes(&self) -> Option<Hashes> {
        self.hashes.clone()
    }
}
