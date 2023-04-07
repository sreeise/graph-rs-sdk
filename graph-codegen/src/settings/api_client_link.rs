use from_as::*;

use std::io::{Read, Write};

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile, Eq, PartialEq, Hash)]
pub struct ApiClientLinkSettings(pub Option<&'static str>, pub Vec<ApiClientLink>);

#[derive(
    Debug, Clone, Serialize, Deserialize, FromFile, AsFile, Eq, PartialEq, Hash, Ord, PartialOrd,
)]
pub enum ApiClientLink {
    Struct(&'static str, &'static str),
    StructId(&'static str, &'static str),
}

impl ApiClientLink {
    pub fn format(&self) -> String {
        match self {
            ApiClientLink::Struct(method_name, api_client_name) => {
                format!("api_client_link!({method_name}, {api_client_name});")
            }
            ApiClientLink::StructId(method_name, api_client_name) => {
                format!("api_client_link_id!({method_name}, {api_client_name});")
            }
        }
    }
}
