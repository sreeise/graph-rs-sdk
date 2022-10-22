pub trait ODataNextLink<RHS = Self> {
    /// Get the OData next link URL.
    fn next_link(&self) -> Option<String>;
}

impl ODataNextLink for serde_json::Value {
    fn next_link(&self) -> Option<String> {
        self["@odata.nextLink"].as_str().map(|s| s.to_string())
    }
}

pub trait ODataDownloadLink<RHS = Self> {
    /// Get the OData next link URL.
    fn download_link(&self) -> Option<String>;
}

impl ODataDownloadLink for serde_json::Value {
    fn download_link(&self) -> Option<String> {
        self["@microsoft.graph.downloadUrl"]
            .as_str()
            .map(|s| s.to_string())
    }
}

pub trait ODataMetadataLink<RHS = Self> {
    /// Get the OData next link URL.
    fn metadata_link(&self) -> Option<String>;
}

impl ODataMetadataLink for serde_json::Value {
    fn metadata_link(&self) -> Option<String> {
        self["@odata.context"].as_str().map(|s| s.to_string())
    }
}

pub trait ODataDeltaLink<RHS = Self> {
    /// Get the OData next link URL.
    fn delta_link(&self) -> Option<String>;
}

impl ODataDeltaLink for serde_json::Value {
    fn delta_link(&self) -> Option<String> {
        self["@odata.deltaLink"].as_str().map(|s| s.to_string())
    }
}
