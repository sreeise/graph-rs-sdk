use graph_core::resource::ResourceIdentity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryModFile {
    pub(crate) resource_identity: ResourceIdentity,
    pub(crate) mod_name: String,
    pub(crate) use_all: bool,
}
