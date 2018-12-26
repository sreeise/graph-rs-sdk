use serde_json;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn serialize_access_token(data: &str) -> AccessToken {
    let access_token: AccessToken = serde_json::from_str(&data).unwrap();
    let serialized = serde_json::to_string(&access_token).unwrap();
    AccessToken::json_file("graph_configs/access_token.json", serialized).unwrap();
    access_token
}

// TODO: Method here for OAuth V2 - Does not work yet
pub fn serialize_access_token_v2(data: &str) -> AccessToken {
    let access_token: AccessToken = serde_json::from_str(&data).unwrap();
    let serialized = serde_json::to_string(&access_token).unwrap();
    AccessToken::json_file("graph_configs/access_token_v2.json", serialized).unwrap();
    access_token
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccessToken {
    pub expires_in: usize,
    pub scope: String,
    pub access_token: String,
    pub user_id: String,
    pub token_type: String,
}

impl AccessToken {
    pub fn from_str(json_str: &str) -> io::Result<AccessToken> {
        let json: AccessToken = serde_json::from_str(json_str)?;
        Ok(json)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<AccessToken> {
        let f = File::open(path)?;
        let access_token: AccessToken = serde_json::from_reader(f).unwrap();
        Ok(access_token)
    }

    fn json_file<P: AsRef<Path>>(path: P, serialized: String) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .expect("Error writing file with serialized struct");
        file.write_all(serialized.as_bytes())
            .expect("Could not write AuthFlow as a new json file");
        Ok(())
    }
}
