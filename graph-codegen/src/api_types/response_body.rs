use from_as::*;
use std::convert::TryFrom;
use std::io::{Read, Write};

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub enum ResponseBody {
    NoContent,
    Json,
    UploadSession,
    Bytes,
    Download,
    AsyncDownload,
    Delta,
}

impl ResponseBody {
    pub fn type_name(&self) -> &str {
        match self {
            ResponseBody::NoContent => "NoContent",
            ResponseBody::Json => "serde_json::Value",
            ResponseBody::UploadSession => "UploadSessionClient<Client>",
            ResponseBody::Bytes => "Vec<u8>",
            ResponseBody::Download => "BlockingDownload",
            ResponseBody::AsyncDownload => "AsyncDownload",
            ResponseBody::Delta => "DeltaPhantom<serde_json::Value>",
        }
    }

    pub fn imports(&self) -> Vec<&str> {
        match self {
            ResponseBody::NoContent => vec!["graph_http::types::NoContent"],
            ResponseBody::Json => vec![],
            ResponseBody::UploadSession => vec![
                "std::path::Path",
                "graph_error::GraphFailure",
                "graph_http::UploadSessionClient",
            ],
            ResponseBody::Bytes => vec![],
            ResponseBody::Download => vec![
                "std::path::Path",
                "graph_error::GraphFailure",
                "graph_http::{BlockingDownload, BlockingHttpClient}",
            ],
            ResponseBody::AsyncDownload => vec![
                "std::path::Path",
                "graph_error::GraphFailure",
                "graph_http::{AsyncDownload, AsyncHttpClient}",
            ],
            ResponseBody::Delta => vec!["graph_http::types::DeltaPhantom"],
        }
    }
}

impl Default for ResponseBody {
    fn default() -> Self {
        ResponseBody::Json
    }
}
