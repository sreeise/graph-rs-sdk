use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use from_as::*;
use crate::parser::{PathMap, RequestMap, Path, Request, HttpMethod, RequestParser, RequestParserBuilder, Operation, RequestSet};
use std::cell::RefCell;
use crate::parser::filter::{MatchTarget, Filter, StoredFilter, ModifierMap};
use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use std::ops::Deref;

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(default)]
pub struct ParserSpec {
    paths: PathMap,
    #[serde(skip_serializing_if = "VecDeque::is_empty")]
    requests: VecDeque<RequestMap>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    tag_map: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    operation_map: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    imports: HashSet<String>,
    modify_target: ModifierMap,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(default)]
pub struct Parser {
    pub spec: RefCell<ParserSpec>
}

impl Parser {
    pub fn paths<P: AsRef<str>>(file: P) -> Parser {
        let mut path_map: PathMap = PathMap::from_file(file.as_ref()).unwrap();
        path_map.clean();
        Parser {
            spec: RefCell::new(ParserSpec {
                paths: path_map,
                requests: Default::default(),
                tag_map: Default::default(),
                operation_map: Default::default(),
                imports: Default::default(),
                modify_target: Default::default(),
            })
        }
    }

    pub fn paths_filter<P: AsRef<str>>(file: P, filter: Filter<'_>) -> Parser {
        let mut path_map: PathMap = PathMap::from_file(file.as_ref()).unwrap();
        path_map.clean();

        Parser {
            spec: RefCell::new(ParserSpec {
                paths: path_map.filter(filter).into(),
                requests: Default::default(),
                tag_map: Default::default(),
                operation_map: Default::default(),
                imports: Default::default(),
                modify_target: Default::default(),
            })
        }
    }

    fn insert_request_map(&self, map: RequestMap) {
        let mut spec = self.spec.borrow_mut();
        if let Some(r) = spec.requests.iter_mut()
            .find(|r| r.path.eq(map.path.as_str())) {
            r.requests.extend(map.requests);
        } else {
            spec.requests.push_back(map);
        }
    }

    pub fn set_operation_map(&self, operation_map: HashMap<String, String>) {
        self.spec.borrow_mut().operation_map = operation_map;
    }

    pub fn add_operation_mapping(&self, original: &str, replace_with: &str) {
        self.spec.borrow_mut().operation_map.insert(original.into(), replace_with.into());
    }

    pub fn add_tag_mapping(&self, original: &str, replace_with: &str) {
        self.spec.borrow_mut().tag_map.insert(original.into(), replace_with.into());
    }

    pub fn add_imports(&self, imports: &[&str]) {
        self.spec.borrow_mut().imports.extend(imports.iter().map(|s| s.to_string()));
    }

    pub fn add_match_modifier(&self, matcher: MatchTarget, modifier: Vec<MatchTarget>) {
        self.spec.borrow_mut().modify_target.map.insert(matcher, modifier);
    }

    pub fn filter(&self, filter: Filter<'_>) -> PathMap {
        let spec = self.spec.borrow();
        PathMap {
            paths: spec.paths.filter(filter)
        }
    }

    pub fn build(&self, filter: Filter<'_>) -> RequestSet {
        let path_map = self.filter(filter);
        self.build_requests(path_map)
    }

    fn build_next_request_map(&self, path: &str, operation: &Operation, method: HttpMethod) {
        let mut req_map = RequestMap::default();
        req_map.path = path.transform_path();
        let mut request = operation.build();
        request.method = method;
        let mut spec = self.spec.borrow_mut();
        if let Some(value) = spec.operation_map.get(request.operation_mapping.as_str()) {
            request.operation_mapping = value.to_string();
        }

        for (mat, modify_vec) in spec.modify_target.map.iter() {
            if mat.matches(&mut request) {
                for modifier in modify_vec.iter() {
                    modifier.modify(&mut request);
                }
            }
        }

        req_map.requests.push_back(request);
        if let Some(r) = spec.requests.iter_mut()
            .find(|r| r.path.eq(req_map.path.as_str())) {
            r.requests.extend(req_map.requests);
        } else {
            spec.requests.push_back(req_map);
        }
    }

    pub fn build_requests(&self, path_map: PathMap) -> RequestSet {
        for (path, path_spec) in path_map.paths.iter() {
            if let Some(operation) = path_spec.get.as_ref() {
                self.build_next_request_map(path.as_str(), operation, HttpMethod::GET);
            }

            if let Some(operation) = path_spec.post.as_ref() {
                self.build_next_request_map(path.as_str(), operation, HttpMethod::POST);
            }

            if let Some(operation) = path_spec.put.as_ref() {
                self.build_next_request_map(path.as_str(), operation, HttpMethod::PUT);
            }

            if let Some(operation) = path_spec.patch.as_ref() {
                self.build_next_request_map(path.as_str(), operation, HttpMethod::PATCH);
            }

            if let Some(operation) = path_spec.delete.as_ref() {
                self.build_next_request_map(path.as_str(), operation, HttpMethod::DELETE);
            }
        }

        let mut request_set = RequestSet::default();
        let mut requests = self.spec.borrow().requests.clone();
        while let Some(req) = requests.pop_front() {
            request_set.join_inner_insert(req);
        }
        request_set
    }
}
