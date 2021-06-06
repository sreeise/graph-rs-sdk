use crate::openapi::{Components, OpenAPI, PathItem};
use graph_error::GraphFailure;
use graph_http::url::GraphUrl;
use reqwest::Url;
use std::{collections::BTreeMap, convert::TryFrom};

pub struct OpenAPIParser {
    open_api: OpenAPI,
}

impl OpenAPIParser {
    pub fn new(open_api: OpenAPI) -> OpenAPIParser {
        OpenAPIParser { open_api }
    }

    pub fn paths(&self) -> BTreeMap<String, PathItem> {
        self.open_api.paths.clone()
    }

    pub fn components(&self) -> Components {
        self.open_api.components.clone()
    }
}

impl TryFrom<reqwest::Url> for OpenAPIParser {
    type Error = GraphFailure;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        let response = reqwest::blocking::get(value)?;
        let open_api_raw_text = response.text()?;
        let open_api: OpenAPI = serde_yaml::from_str(open_api_raw_text.as_str())?;
        Ok(OpenAPIParser::new(open_api))
    }
}

impl TryFrom<GraphUrl> for OpenAPIParser {
    type Error = GraphFailure;

    fn try_from(value: GraphUrl) -> Result<Self, Self::Error> {
        Ok(OpenAPIParser::new(OpenAPI::try_from(
            value.to_reqwest_url(),
        )?))
    }
}
