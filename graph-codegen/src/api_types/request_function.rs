use from_as::*;
use std::convert::TryFrom;
use std::io::{Read, Write};

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub enum RequestFunction {
    Normal,
    Upload,
    UploadSession,
    Download,
    AsyncDownload,
}

impl Default for RequestFunction {
    fn default() -> Self {
        RequestFunction::Normal
    }
}
