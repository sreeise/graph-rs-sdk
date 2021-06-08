mod components;
mod discriminator;
mod either_t;
mod encoding;
mod example;
mod external_documentation;
mod header;
mod link;
mod media_type;
mod oauth_flow;
mod oauth_flows;
mod openapi_parser;
mod operation;
mod parameter;
mod path_item;
mod reference;
mod request_body;
mod response;
mod responses;
mod schema;
mod security_requirement;
mod security_scheme;
mod server;
mod server_variable;
mod xml;

pub use components::*;
pub use discriminator::*;
pub use either_t::*;
pub use encoding::*;
pub use example::*;
pub use external_documentation::*;
pub use header::*;
pub use link::*;
pub use media_type::*;
pub use oauth_flow::*;
pub use oauth_flows::*;
pub use openapi_parser::*;
pub use operation::*;
pub use parameter::*;
pub use path_item::*;
pub use reference::*;
pub use request_body::*;
pub use response::*;
pub use responses::*;
pub use schema::*;
pub use security_requirement::*;
pub use security_scheme::*;
pub use server::*;
pub use server_variable::*;
pub use xml::*;

use crate::traits::{RequestParser, FilterPath};
use from_as::*;
use graph_error::GraphFailure;
use graph_http::url::GraphUrl;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reqwest::Url;
use std::{
    collections::{BTreeMap, VecDeque},
    convert::TryFrom,
    io::{Read, Write},
};

/// [OpenAPI Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#oasObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct OpenAPI {
    /// REQUIRED. This string MUST be the version number of the OpenAPI
    /// Specification that the OpenAPI document uses. The openapi field
    /// SHOULD be used by tooling to interpret the OpenAPI document. This is
    /// not related to the API info.version string.
    pub openapi: String,

    /// REQUIRED. Provides metadata about the API. The metadata MAY be used by
    /// tooling as required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,

    /// The default value for the $schema keyword within Schema Objects
    /// contained within this OAS document. This MUST be in the form of a
    /// URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "jsonSchemaDialect")]
    pub json_schema_dialect: Option<String>,

    /// An array of Server Objects, which provide connectivity information to a
    /// target server. If the servers property is not provided, or is an
    /// empty array, the default value would be a Server Object with a url
    /// value of /.
    #[serde(default)]
    #[serde(skip_serializing_if = "VecDeque::is_empty")]
    pub servers: VecDeque<serde_json::Value>,

    /// The available paths and operations for the API.
    /// [Paths Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#pathsObject)
    pub paths: BTreeMap<String, PathItem>,

    /// The incoming webhooks that MAY be received as part of this API and that
    /// the API consumer MAY choose to implement. Closely related to the
    /// callbacks feature, this section describes requests initiated other
    /// than by an API call, for example by an out of band registration. The
    /// key name is a unique string to refer to each webhook, while the
    /// (optionally referenced) Path Item Object describes a request that
    /// may be initiated by the API provider and the expected responses. An
    /// example is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<serde_json::Value>,

    /// An element to hold various schemas for the document.
    ///#[serde(skip_serializing_if = "Option::is_none")]
    pub components: Components,

    /// A declaration of which security mechanisms can be used across the API.
    /// The list of values includes alternative security requirement objects
    /// that can be used. Only one of the security requirement objects need
    /// to be satisfied to authorize a request. Individual operations can
    /// override this definition. To make security optional, an empty security
    /// requirement ({}) can be included in the array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<serde_json::Value>,

    /// A list of tags used by the document with additional metadata. The order
    /// of the tags can be used to reflect on their order by the parsing
    /// tools. Not all tags that are used by the Operation Object must be
    /// declared. The tags that are not declared MAY be organized randomly
    /// or based on the tools' logic. Each tag name in the list MUST be unique.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

    /// Additional external documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
}

impl OpenAPI {
    pub fn contains_path(&self, pat: &str) -> Vec<(String, PathItem)> {
        self.paths
            .iter()
            .filter(|(path, _path_item)| path.contains(pat))
            .map(|(path, path_item)| (path.clone(), path_item.clone()))
            .collect()
    }

    pub fn path_starts_with(&self, pat: &str) -> Vec<(String, PathItem)> {
        self.paths
            .iter()
            .filter(|(path, _path_item)| path.starts_with(pat))
            .map(|(path, path_item)| (path.clone(), path_item.clone()))
            .collect()
    }

    pub fn filter_path(&mut self, pat: &str) -> BTreeMap<String, PathItem> {
        self.paths
            .clone()
            .into_par_iter()
            .filter(|(path, _path_item)| path.starts_with(pat))
            .collect()
    }

    pub fn filter_replace_path(&mut self, pat: &str) {
        self.paths = self.filter_path(pat);
    }

    pub fn transform_paths(&mut self) {
        self.paths = self.paths
            .clone()
            .into_par_iter()
            .map(|(path, path_item)| (path.transform_path(), path_item.clone()))
            .collect();
    }
}

impl TryFrom<reqwest::Url> for OpenAPI {
    type Error = GraphFailure;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        let response = reqwest::blocking::get(value)?;
        let open_api_raw_text = response.text()?;
        let open_api: OpenAPI = serde_yaml::from_str(open_api_raw_text.as_str())?;
        Ok(open_api)
    }
}

impl TryFrom<GraphUrl> for OpenAPI {
    type Error = GraphFailure;

    fn try_from(value: GraphUrl) -> Result<Self, Self::Error> {
        OpenAPI::try_from(value.to_reqwest_url())
    }
}

impl IntoIterator for OpenAPI {
    type IntoIter = std::collections::btree_map::IntoIter<String, PathItem>;
    type Item = (String, PathItem);

    fn into_iter(self) -> Self::IntoIter {
        self.paths.into_iter()
    }
}

impl AsRef<BTreeMap<String, PathItem>> for OpenAPI {
    fn as_ref(&self) -> &BTreeMap<String, PathItem> {
        &self.paths
    }
}

impl AsMut<BTreeMap<String, PathItem>> for OpenAPI {
    fn as_mut(&mut self) -> &mut BTreeMap<String, PathItem> {
        &mut self.paths
    }
}

impl FilterPath for OpenAPI {
    fn paths(&self) -> BTreeMap<String, PathItem> {
        self.paths.clone()
    }
}
