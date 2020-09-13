pub trait MetadataLink<RHS = Self> {
    fn metadata_link(&self) -> Option<String>;
}

pub trait NextLink<RHS = Self> {
    fn next_link(&self) -> Option<String>;
}

pub trait DeltaLink<RHS = Self> {
    fn delta_link(&self) -> Option<String>;
}

impl NextLink for serde_json::Value {
    fn next_link(&self) -> Option<String> {
        self["@odata.nextLink"].as_str().map(|s| s.to_string())
    }
}

impl DeltaLink for serde_json::Value {
    fn delta_link(&self) -> Option<String> {
        self["@odata.deltaLink"].as_str().map(|s| s.to_string())
    }
}

impl MetadataLink for serde_json::Value {
    fn metadata_link(&self) -> Option<String> {
        self["@odata.context"].as_str().map(|s| s.to_string())
    }
}
