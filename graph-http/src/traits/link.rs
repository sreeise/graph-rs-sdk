pub trait ODataLink<RHS = Self> {
    /// Get the download URL.
    fn download_link(&self) -> Option<String>;
    /// Get the OData context URL.
    fn metadata_link(&self) -> Option<String>;
    /// Get the OData next link URL.
    fn next_link(&self) -> Option<String>;
    /// Get the OData delta link URL.
    fn delta_link(&self) -> Option<String>;
}

impl ODataLink for serde_json::Value {
    fn download_link(&self) -> Option<String> {
        self["@microsoft.graph.downloadUrl"]
            .as_str()
            .map(|s| s.to_string())
    }

    fn metadata_link(&self) -> Option<String> {
        self["@odata.context"].as_str().map(|s| s.to_string())
    }

    fn next_link(&self) -> Option<String> {
        self["@odata.nextLink"].as_str().map(|s| s.to_string())
    }

    fn delta_link(&self) -> Option<String> {
        self["@odata.deltaLink"].as_str().map(|s| s.to_string())
    }
}
