use crate::parser::filter::*;
use crate::parser::{HttpMethod, Request, ResponseType};
use crate::traits::{RequestParser, RequestParserBuilder};
use from_as::*;
use regex::Regex;
use std::collections::{BTreeMap, HashSet, VecDeque};

pub trait PathRetain {
    fn path_retain(&mut self);
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Response {
    #[serde(rename = "204")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_content: Option<ResponseObject>,
    #[serde(rename = "200")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok: Option<ResponseObject>,
    #[serde(rename = "201")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<ResponseObject>,
}

impl Response {
    pub fn get_type(&self) -> ResponseType {
        if self.no_content.is_some() {
            return ResponseType::NoContent;
        }

        if let Some(response_object) = self.ok.as_ref() {
            if response_object.is_array() {
                return ResponseType::Collection;
            }
        }

        if let Some(response_object) = self.created.as_ref() {
            if response_object.is_array() {
                return ResponseType::Collection;
            }
        }

        ResponseType::SerdeJson
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct ResponseObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
}

impl ResponseObject {
    pub fn is_array(&self) -> bool {
        if let Some(content) = self.content.as_ref() {
            content.is_array()
        } else {
            false
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Content {
    #[serde(rename = "application/json")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_json: Option<ApplicationJson>,
}

impl Content {
    pub fn is_array(&self) -> bool {
        if let Some(application_json) = self.application_json.as_ref() {
            application_json.is_array()
        } else {
            false
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct ApplicationJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
}

impl ApplicationJson {
    pub fn is_array(&self) -> bool {
        if let Some(schema) = self.schema.as_ref() {
            schema.is_array()
        } else {
            false
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Schema {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl Schema {
    pub fn is_array(&self) -> bool {
        if let Some(_type) = self._type.as_ref() {
            _type.eq("array")
        } else {
            false
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Operation {
    #[serde(skip_serializing_if = "VecDeque::is_empty")]
    pub tags: VecDeque<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "operationId")]
    pub operation_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "VecDeque::is_empty")]
    pub parameters: VecDeque<Parameter>,
    #[serde(rename = "requestBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Response>,
}

impl Operation {
    pub fn param_size(&self) -> usize {
        self.parameters
            .iter()
            .filter(|param| param.is_in_path())
            .count()
    }

    pub fn has_body(&self) -> bool {
        self.request_body.is_some()
    }

    pub fn response_type(&self) -> ResponseType {
        if let Some(response) = self.responses.as_ref() {
            response.get_type()
        } else {
            ResponseType::SerdeJson
        }
    }
}

impl PathRetain for Option<Operation> {
    fn path_retain(&mut self) {
        if let Some(operation) = self.as_mut() {
            operation.parameters.path_retain();
        }
    }
}

impl RequestParser for Operation {
    fn method_name(&self) -> String {
        self.operation_id.method_name()
    }

    fn operation_mapping(&self) -> String {
        self.operation_id.operation_mapping()
    }

    fn transform_path(&self) -> String {
        Default::default()
    }

    fn links(&self) -> HashSet<String> {
        self.operation_mapping().links()
    }
}

impl RequestParserBuilder for Operation {
    /// The build method is where each individual request
    /// is parsed.
    fn build(&self, modifiers: &ModifierMap) -> Request {
        let mut request = Request::default();
        request.operation_id = self.operation_id.to_string();
        request.operation_mapping = self.operation_id.operation_mapping();
        request.method_name = self.method_name();
        request.param_size = self.param_size();
        request.has_body = self.has_body();
        request.response = self.response_type();
        request.doc = self.summary.clone().map(|s| format!("# {}", s));
        if request.method.eq(&HttpMethod::DELETE) {
            request.response = ResponseType::NoContent;
        } else if request.method_name.starts_with("list") {
            request.response = ResponseType::Collection;
        } else if request.method_name.eq("delta") {
            request.response = ResponseType::Delta;
        }
        if let Some(tag) = self.tags.get(0) {
            request.tag = tag.to_string();
        }
        request.modify(modifiers);
        request
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Path {
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Operation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "VecDeque::is_empty")]
    pub parameters: VecDeque<Parameter>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Parameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Parameter {
    pub fn is_in_path(&self) -> bool {
        if let Some(in_) = self.in_.as_ref() {
            in_.eq("path")
        } else {
            false
        }
    }
}

impl PathRetain for VecDeque<Parameter> {
    fn path_retain(&mut self) {
        self.retain(|p| p.is_in_path());
    }
}

impl PathRetain for Option<VecDeque<Parameter>> {
    fn path_retain(&mut self) {
        if let Some(vec) = self.as_mut() {
            vec.path_retain();
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(default)]
pub struct PathMap {
    pub paths: BTreeMap<String, Path>,
}

impl PathMap {
    pub fn clean(&mut self) {
        self.path_retain();
    }
}

impl PathRetain for PathMap {
    fn path_retain(&mut self) {
        for (_s, path) in self.paths.iter_mut() {
            path.parameters.retain(|param| param.is_in_path());
            path.get.path_retain();
            path.put.path_retain();
            path.patch.path_retain();
            path.post.path_retain();
            path.delete.path_retain();
        }
    }
}

impl From<BTreeMap<String, Path>> for PathMap {
    fn from(paths: BTreeMap<String, Path>) -> Self {
        PathMap { paths }
    }
}

impl PathMap {
    pub fn filter(&self, filter: Filter<'_>) -> BTreeMap<String, Path> {
        match filter {
            Filter::PathStartsWith(filter) => self
                .paths
                .clone()
                .into_iter()
                .filter(|(path, _path_spec)| path.starts_with(filter))
                .collect(),
            Filter::None => self.paths.clone(),
            Filter::PathEquals(filter) => self
                .paths
                .clone()
                .into_iter()
                .filter(|(path, _path_spec)| path.eq(filter))
                .collect(),
            Filter::Regex(s) => {
                let regex = Regex::new(s).unwrap();
                self.paths
                    .clone()
                    .into_iter()
                    .filter(|(path, _path_spec)| regex.is_match(path.as_ref()))
                    .collect()
            },
            Filter::IgnoreIf(filter_ignore) => match filter_ignore {
                FilterIgnore::PathContains(s) => self
                    .paths
                    .clone()
                    .into_iter()
                    .filter(|(path, _path_spec)| !path.contains(s))
                    .collect(),
                FilterIgnore::PathStartsWith(s) => self
                    .paths
                    .clone()
                    .into_iter()
                    .filter(|(path, _path_spec)| !path.starts_with(s))
                    .collect(),
            },
            Filter::MultiFilter(vec) => {
                let mut map: BTreeMap<String, Path> = BTreeMap::new();
                let mut count = 0;
                for filter in vec.iter() {
                    if count == 0 {
                        map = self.filter(filter.clone());
                        count += 1;
                    } else {
                        map = PathMap { paths: map }.filter(filter.clone())
                    }
                }
                map
            },
        }
    }
}

impl IntoIterator for PathMap {
    type Item = (String, Path);
    type IntoIter = std::collections::btree_map::IntoIter<String, Path>;

    fn into_iter(self) -> Self::IntoIter {
        self.paths.into_iter()
    }
}
