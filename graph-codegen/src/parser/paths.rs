use crate::parser::filter::*;
use from_as::*;
use regex::Regex;

use std::collections::{BTreeMap, HashMap, VecDeque};

pub trait PathRetain {
    fn path_retain(&mut self);
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile, PartialEq)]
#[serde(default)]
pub struct PropertyValue {
    #[serde(rename = "type")]
    type_: String,
    items: HashMap<String, String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile, PartialEq)]
#[serde(default)]
pub struct Properties {
    value: PropertyValue,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    properties: Properties,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "application/json")]
    content_type_map: HashMap<String, Schema>,
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
}

impl PathRetain for Option<Operation> {
    fn path_retain(&mut self) {
        if let Some(operation) = self.as_mut() {
            operation.parameters.path_retain();
        }
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
