use crate::drive::base::hashes::Hashes;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "mimeType")]
    mime_type: Option<String>,
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
