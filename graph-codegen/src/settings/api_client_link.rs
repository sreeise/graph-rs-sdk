use from_as::*;
use graph_core::resource::ResourceIdentity;
use std::io::{Read, Write};

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile, Eq, PartialEq, Hash)]
pub struct ApiClientLinkSettings(pub Option<&'static str>, pub Vec<ApiClientLink>);

#[derive(
    Debug, Clone, Serialize, Deserialize, FromFile, AsFile, Eq, PartialEq, Hash, Ord, PartialOrd,
)]
pub enum ApiClientLink {
    IdMethod(String, ResourceIdentity),
    Resource(String, String, ResourceIdentity),
    ResourceId(String, String, ResourceIdentity),
    // Macros that won't be formatted with a ResourceIdentity.
    Struct(&'static str, &'static str),
    StructId(&'static str, &'static str),
}

impl ApiClientLink {
    pub fn format(&self) -> String {
        match self {
            ApiClientLink::IdMethod(api_client_name, resource_identity) => format!(
                "api_client_link_id!({}, {});",
                resource_identity.enum_string(),
                api_client_name
            ),
            ApiClientLink::Resource(method_name, api_client_name, resource_identity) => format!(
                "api_client_link!({}, {}, {});",
                method_name,
                resource_identity.enum_string(),
                api_client_name
            ),
            ApiClientLink::ResourceId(method_name, api_client_name, resource_identity) => format!(
                "api_client_link_id!({}, {}, {});",
                method_name,
                resource_identity.enum_string(),
                api_client_name
            ),
            ApiClientLink::Struct(method_name, api_client_name) => {
                format!("api_client_link!({method_name}, {api_client_name});")
            }
            ApiClientLink::StructId(method_name, api_client_name) => {
                format!("api_client_link_id!({method_name}, {api_client_name});")
            }
        }
    }
}
