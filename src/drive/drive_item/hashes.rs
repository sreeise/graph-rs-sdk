use std::io::Write;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct Hashes {
    #[serde(rename = "sha1Hash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sha1_hash: Option<String>,
    #[serde(rename = "quickXorHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    quick_xor_hash: Option<String>,
    #[serde(rename = "crc32Hash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    crc32_hash: Option<String>,
}

impl Hashes {
    pub fn new(
        sha1_hash: Option<String>,
        quick_xor_hash: Option<String>,
        crc32_hash: Option<String>,
    ) -> Self {
        Hashes {
            sha1_hash,
            quick_xor_hash,
            crc32_hash,
        }
    }
}

