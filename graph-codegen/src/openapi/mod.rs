mod discriminator;
mod encoding;
mod example;
mod external_documentation;
mod header;
mod media_type;
mod operation;
mod parameter;
mod path_item;
mod paths;
mod reference;
mod request_body;
mod schema;
mod security_requirement;
mod server;
mod server_variable;
mod xml;

pub use discriminator::*;
pub use encoding::*;
pub use example::*;
pub use external_documentation::*;
pub use header::*;
pub use media_type::*;
pub use operation::*;
pub use parameter::*;
pub use path_item::*;
pub use paths::*;
pub use reference::*;
pub use request_body::*;
pub use schema::*;
pub use security_requirement::*;
pub use server::*;
pub use server_variable::*;
pub use xml::*;

use from_as::*;
use graph_http::url::GraphUrl;
use reqwest::Url;
use std::{
    collections::VecDeque,
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
    pub paths: Paths,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<serde_json::Value>,

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

impl TryFrom<reqwest::Url> for OpenAPI {
    type Error = reqwest::Error;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        let response = reqwest::blocking::get(value)?;
        let open_api_raw_text = response.text().unwrap();
        let open_api: OpenAPI = serde_yaml::from_str(open_api_raw_text.as_str()).unwrap();
        Ok(open_api)
    }
}

impl TryFrom<GraphUrl> for OpenAPI {
    type Error = reqwest::Error;

    fn try_from(value: GraphUrl) -> Result<Self, Self::Error> {
        let response = reqwest::blocking::get(value.to_reqwest_url())?;
        let open_api_raw_text = response.text().unwrap();
        let open_api: OpenAPI = serde_yaml::from_str(open_api_raw_text.as_str()).unwrap();
        Ok(open_api)
    }
}
