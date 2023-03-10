mod components;
mod contact;
mod discriminator;
mod either_t;
mod encoding;
mod example;
mod external_documentation;
mod header;
mod info;
mod license;
mod link;
mod media_type;
mod oauth_flow;
mod oauth_flows;
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
mod tag;
mod xml;

pub use components::*;
pub use contact::*;
pub use discriminator::*;
pub use either_t::*;
pub use encoding::*;
pub use example::*;
pub use external_documentation::*;
pub use header::*;
pub use info::*;
pub use license::*;
pub use link::*;
pub use media_type::*;
pub use oauth_flow::*;
pub use oauth_flows::*;
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
pub use tag::*;
pub use xml::*;

use crate::api_types::{PathMetadata, WriteConfiguration};
use crate::macros::OpenApiParser;
use crate::traits::{FilterPath, RequestParser};
use from_as::*;
use graph_error::GraphFailure;
use graph_http::url::GraphUrl;
use inflector::Inflector;
use rayon::prelude::*;
use reqwest::Url;
use serde_json::Value;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::{
    collections::{BTreeMap, VecDeque},
    convert::TryFrom,
    io::{Read, Write},
};

static MS_GRAPH_METADATA_URL: &str = "https://raw.githubusercontent.com/microsoftgraph/msgraph-metadata/master/openapi/v1.0/openapi.yaml";
static MS_GRAPH_BETA_METADATA_URL: &str = "https://raw.githubusercontent.com/microsoftgraph/msgraph-metadata/master/openapi/beta/openapi.yaml";

/// [OpenAPI Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#oasObject)
#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(rename_all = "camelCase")]
pub struct OpenApi {
    /// REQUIRED. This string MUST be the version number of the OpenAPI
    /// Specification that the OpenAPI document uses. The openapi field
    /// SHOULD be used by tooling to interpret the OpenAPI document. This is
    /// not related to the API info.version string.
    pub openapi: String,

    /// REQUIRED. Provides metadata about the API. The metadata MAY be used by
    /// tooling as required.
    pub info: Info,

    /// The default value for the $schema keyword within Schema Objects
    /// contained within this OAS document. This MUST be in the form of a
    /// URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema_dialect: Option<String>,

    /// An array of Server Objects, which provide connectivity information to a
    /// target server. If the servers property is not provided, or is an
    /// empty array, the default value would be a Server Object with a url
    /// value of /.
    #[serde(default)]
    #[serde(skip_serializing_if = "VecDeque::is_empty")]
    pub servers: VecDeque<Server>,

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
    #[serde(default)]
    pub webhooks: HashMap<String, EitherT<PathItem, Reference>>,

    /// An element to hold various schemas for the document.
    pub components: Components,

    /// A declaration of which security mechanisms can be used across the API.
    /// The list of values includes alternative security requirement objects
    /// that can be used. Only one of the security requirement objects need
    /// to be satisfied to authorize a request. Individual operations can
    /// override this definition. To make security optional, an empty security
    /// requirement ({}) can be included in the array.
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<SecurityRequirement>,

    /// A list of tags used by the document with additional metadata. The order
    /// of the tags can be used to reflect on their order by the parsing
    /// tools. Not all tags that are used by the Operation Object must be
    /// declared. The tags that are not declared MAY be organized randomly
    /// or based on the tools' logic. Each tag name in the list MUST be unique.
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub tags: Option<Tag>,

    /// Additional external documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
}

impl OpenApi {
    pub fn paths(&self) -> &BTreeMap<String, PathItem> {
        &self.paths
    }

    pub fn filter_path(&self, pat: &str) -> BTreeMap<String, PathItem> {
        self.paths
            .clone()
            .into_par_iter()
            .filter(|(path, _path_item)| path.starts_with(pat))
            .collect()
    }

    pub fn filter_path_contains(&self, pat: &str) -> BTreeMap<String, PathItem> {
        self.paths
            .clone()
            .into_par_iter()
            .filter(|(path, _path_item)| path.contains(pat))
            .collect()
    }

    pub fn filter_resource_parsing_info_path(
        &self,
        resource_parsing_info: &WriteConfiguration,
    ) -> BTreeMap<String, PathItem> {
        let trim_path_start = resource_parsing_info
            .trim_path_start
            .clone()
            .unwrap_or_default();
        let p = resource_parsing_info.path.to_string();
        self.paths
            .clone()
            .into_par_iter()
            .map(|(path, path_item)| {
                (
                    path.trim_start_matches(&trim_path_start).to_string(),
                    path_item,
                )
            })
            .filter(|(path, _path_item)| path.starts_with(&p))
            .collect()
    }

    pub fn filter_replace_path(&mut self, pat: &str) {
        self.paths = self.filter_path(pat);
    }

    pub fn transform_paths(&mut self) {
        self.paths = self
            .paths
            .clone()
            .into_par_iter()
            .map(|(path, path_item)| (path.transform_path(), path_item))
            .collect();
    }

    pub fn requests_secondary(
        &self,
        trim_pat: &str,
        parameter_filter: &[String],
    ) -> VecDeque<PathMetadata> {
        self.paths
            .iter()
            .filter(|(path, _path_item)| path.starts_with(trim_pat))
            .map(|(path, path_item)| {
                path_item.request_metadata_secondary(path.as_str(), trim_pat, parameter_filter)
            })
            .collect()
    }

    pub fn requests(&self) -> VecDeque<PathMetadata> {
        self.paths
            .iter()
            .map(|(path, path_item)| path_item.request_metadata(path.as_str()))
            .collect()
    }

    pub fn requests_filter(&self, path_start: &str) -> VecDeque<PathMetadata> {
        self.paths
            .iter()
            .filter(|(path, _path_item)| path.starts_with(path_start))
            .map(|(path, path_item)| path_item.request_metadata(path.as_str()))
            .collect()
    }

    pub fn operations(&self) -> VecDeque<Operation> {
        self.paths
            .iter()
            .flat_map(|(_path, path_item)| path_item.operations())
            .collect()
    }

    /// Returns all unique first path parts of a split path array of each path.
    ///
    /// # Explanation
    /// Given the following paths
    /// ```json
    /// /deviceAppManagement/managedAppRegistrations
    /// /deviceManagement/managedDevices
    /// /directory/federationConfigurations
    /// ```
    /// The method returns the following BTreeSet:
    /// ```json
    /// [deviceAppManagement, deviceManagement, directory]
    /// ```
    ///
    /// # Example
    /// ```rust
    /// use graph_codegen::openapi::OpenApi;
    /// let open_api = OpenApi::default();
    ///
    /// let set = open_api.top_level_resources();
    /// assert!(set.contains("DeviceAppManagement"))
    pub fn top_level_resources(&self) -> BTreeSet<String> {
        self.paths()
            .iter()
            .map(|(path, _path_item)| {
                path.split('/')
                    .into_iter()
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.to_pascal_case())
                    .take(1)
                    .collect()
            })
            .collect()
    }

    /// Get the first secondary path names for a given resource.
    ///
    /// # Explanation
    /// Take deviceAppManagement as an example.
    /// deviceAppManagement is a top level resource and its path starts with /deviceAppManagement
    ///
    /// managedAppRegistrations is a first second level resource of deviceAppManagement and the path
    /// is /deviceAppManagement/managedAppRegistrations so the first level resources are any `path[1]`
    /// given a split string array of the path.
    ///
    /// This method will ignore and filter out any parts of paths used for insertions such as
    /// - `{id}` in `/resource/{id}`,
    /// - Any parts of paths that have the following chars: ['{', '(', '.', '$'] such as $count and microsoft.graph
    /// - Any second level resource that does not appear greater than or equal to 3 times
    ///
    /// # Example
    /// ```rust
    /// use graph_codegen::openapi::OpenApi;
    /// let open_api = OpenApi::default();
    ///
    /// let set = open_api.first_second_level_resources("deviceAppManagement");
    /// assert!(set.contains("managedAppRegistrations"))
    /// ```
    pub fn first_second_level_resources(&self, pat: &str) -> BTreeSet<String> {
        let pat = {
            if !pat.starts_with('/') {
                format!("/{pat}")
            } else {
                pat.to_string()
            }
        };

        let paths = self.filter_path(pat.as_str());
        let mut set: BTreeSet<String> = BTreeSet::new();
        let mut set_empty: BTreeSet<String> = BTreeSet::new();
        let mut frequency: HashMap<String, i64> = HashMap::new();

        for (path, _) in paths.iter() {
            let p = path.replace("{group-id}/", "");
            let mut path_arr: VecDeque<String> = p
                .split('/')
                .filter(|s| !s.trim().is_empty())
                .filter(|s| !s.contains(['{', '(', '.', '$']))
                .map(|s| s.to_string())
                .collect();

            if path_arr.len() > 1 {
                frequency
                    .entry(path_arr[1].to_string())
                    .and_modify(|mut i| *i += 1)
                    .or_insert(1);
            }

            for (key, value) in frequency.iter() {
                if *value >= 3 {
                    set.insert(key.to_string());
                }
            }
        }

        set
    }
}

impl Default for OpenApi {
    fn default() -> Self {
        match OpenApi::try_from(GraphUrl::parse(MS_GRAPH_METADATA_URL).unwrap()) {
            Ok(open_api) => open_api,
            Err(e) => {
                println!("Error parsing v1.0 metadata: {e:#?}\n\nAttempting beta Api metadata");
                OpenApi::try_from(GraphUrl::parse(MS_GRAPH_BETA_METADATA_URL).unwrap()).unwrap()
            }
        }
    }
}

impl TryFrom<reqwest::Url> for OpenApi {
    type Error = GraphFailure;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        let response = reqwest::blocking::get(value)?;
        let open_api_yaml = response.text()?;
        let open_api: OpenApi = serde_yaml::from_str(open_api_yaml.as_str())?;
        Ok(open_api)
    }
}

impl TryFrom<GraphUrl> for OpenApi {
    type Error = GraphFailure;

    fn try_from(value: GraphUrl) -> Result<Self, Self::Error> {
        OpenApi::try_from(value.to_reqwest_url())
    }
}

impl TryFrom<serde_yaml::Value> for OpenApi {
    type Error = GraphFailure;

    fn try_from(value: serde_yaml::Value) -> Result<Self, Self::Error> {
        serde_yaml::from_value(value).map_err(GraphFailure::from)
    }
}

impl TryFrom<OpenApiRaw> for OpenApi {
    type Error = GraphFailure;

    fn try_from(value: OpenApiRaw) -> Result<Self, Self::Error> {
        serde_json::from_value(value.open_api).map_err(GraphFailure::from)
    }
}

impl IntoIterator for OpenApi {
    type Item = (String, PathItem);
    type IntoIter = std::collections::btree_map::IntoIter<String, PathItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.paths.into_iter()
    }
}

impl AsRef<BTreeMap<String, PathItem>> for OpenApi {
    fn as_ref(&self) -> &BTreeMap<String, PathItem> {
        &self.paths
    }
}

impl AsMut<BTreeMap<String, PathItem>> for OpenApi {
    fn as_mut(&mut self) -> &mut BTreeMap<String, PathItem> {
        &mut self.paths
    }
}

impl FilterPath for OpenApi {
    fn paths(&self) -> BTreeMap<String, PathItem> {
        self.paths.clone()
    }
}

impl OpenApiParser for OpenApi {}

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(rename_all = "camelCase")]
pub struct OpenApiRaw {
    pub open_api: serde_json::Value,
}

impl OpenApiRaw {
    pub fn requests_filter(&self, path_start: &str) -> HashMap<String, Value> {
        let paths = self.open_api["paths"].as_object().unwrap();
        paths
            .iter()
            .filter(|(s, _v)| s.starts_with(path_start))
            .map(|(s, v)| (s.clone(), v.clone()))
            .collect()
    }

    pub fn path_filter(&mut self, path_start: &str, key_map: Vec<String>) {
        let mut schema = self.open_api["paths"].clone();
        let mut schema_map = schema.as_object().unwrap().clone();
        self.open_api["paths"] = serde_json::to_value(schema_map).unwrap();
    }

    pub fn components(&self) -> serde_json::Value {
        self.open_api["components"].clone()
    }
}

impl Default for OpenApiRaw {
    fn default() -> Self {
        OpenApiRaw::try_from(GraphUrl::parse(MS_GRAPH_METADATA_URL).unwrap()).unwrap()
    }
}

impl TryFrom<GraphUrl> for OpenApiRaw {
    type Error = GraphFailure;

    fn try_from(value: GraphUrl) -> Result<Self, Self::Error> {
        OpenApiRaw::try_from(value.to_reqwest_url())
    }
}

impl TryFrom<reqwest::Url> for OpenApiRaw {
    type Error = GraphFailure;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        let response = reqwest::blocking::get(value)?;
        let open_api_raw_text = response.text()?;
        let open_api: serde_json::Value = serde_yaml::from_str(open_api_raw_text.as_str())?;
        Ok(OpenApiRaw { open_api })
    }
}
