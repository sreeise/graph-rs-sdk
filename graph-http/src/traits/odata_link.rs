pub trait ODataNextLink<RHS = Self> {
    /// Get the OData next link URL.
    fn odata_next_link(&self) -> Option<String>;
}

impl ODataNextLink for serde_json::Value {
    fn odata_next_link(&self) -> Option<String> {
        self["@odata.nextLink"].as_str().map(|s| s.to_string())
    }
}

pub trait ODataDownloadLink<RHS = Self> {
    /// Get the OData next link URL.
    fn odata_download_link(&self) -> Option<String>;
}

impl ODataDownloadLink for serde_json::Value {
    fn odata_download_link(&self) -> Option<String> {
        self["@microsoft.graph.downloadUrl"]
            .as_str()
            .map(|s| s.to_string())
    }
}

pub trait ODataMetadataLink<RHS = Self> {
    /// Get the OData next link URL.
    fn odata_metadata_link(&self) -> Option<String>;
}

impl ODataMetadataLink for serde_json::Value {
    fn odata_metadata_link(&self) -> Option<String> {
        self["@odata.context"].as_str().map(|s| s.to_string())
    }
}

pub trait ODataDeltaLink<RHS = Self> {
    /// Get the OData next link URL.
    fn odata_delta_link(&self) -> Option<String>;
}

impl ODataDeltaLink for serde_json::Value {
    fn odata_delta_link(&self) -> Option<String> {
        self["@odata.deltaLink"].as_str().map(|s| s.to_string())
    }
}

pub trait UploadSessionLink {
    fn upload_session_link(&self) -> Option<String>;
}

impl UploadSessionLink for serde_json::Value {
    fn upload_session_link(&self) -> Option<String> {
        self["uploadUrl"].as_str().map(|s| s.to_string())
    }
}
