use crate::parser::filter::{Filter, MatchTarget, ModifierMap};
use crate::parser::{
    HttpMethod, PathMap, Request, RequestMap, RequestParser, RequestParserBuilder,
    RequestSet,
};
use from_as::*;

use serde::Serialize;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};

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
    pub spec: RefCell<ParserSpec>,
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
            }),
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
            }),
        }
    }

    pub fn set_operation_map(&self, operation_map: HashMap<String, String>) {
        self.spec.borrow_mut().operation_map = operation_map;
    }

    pub fn add_operation_mapping(&self, original: &str, replace_with: &str) {
        self.spec
            .borrow_mut()
            .operation_map
            .insert(original.into(), replace_with.into());
    }

    pub fn add_tag_mapping(&self, original: &str, replace_with: &str) {
        self.spec
            .borrow_mut()
            .tag_map
            .insert(original.into(), replace_with.into());
    }

    pub fn add_imports(&self, imports: &[&str]) {
        self.spec
            .borrow_mut()
            .imports
            .extend(imports.iter().map(|s| s.to_string()));
    }

    pub fn add_match_modifier(&self, matcher: MatchTarget, modifier: Vec<MatchTarget>) {
        self.spec
            .borrow_mut()
            .modify_target
            .map
            .insert(matcher, modifier);
    }

    pub fn use_default_modifier(&self, name: &str) {
        let name = name.to_string();
        let shorthand = &name[..name.len() - 1];
        let double_name = format!("{}.{}", name, shorthand);
        let functions = format!("{}.Functions", name);
        let actions = format!("{}.Actions", name);
        let mut spec = self.spec.borrow_mut();

        spec.modify_target.map.insert(
            MatchTarget::Tag(double_name),
            vec![MatchTarget::TagAndOperationMap(name.clone())],
        );
        spec.modify_target.map.insert(
            MatchTarget::Tag(actions),
            vec![MatchTarget::TagAndOperationMap(name.clone())],
        );
        spec.modify_target.map.insert(
            MatchTarget::Tag(functions),
            vec![MatchTarget::TagAndOperationMap(name.clone())],
        );
    }

    pub fn use_default_modifiers(&self, names: &[&str]) {
        let mut spec = self.spec.borrow_mut();

        for name in names.iter() {
            let shorthand = &name[..name.len() - 1];
            let double_name = format!("{}.{}", name, shorthand);
            let functions = format!("{}.Functions", name);
            let actions = format!("{}.Actions", name);

            spec.modify_target.map.insert(
                MatchTarget::Tag(double_name),
                vec![MatchTarget::TagAndOperationMap(name.to_string())],
            );
            spec.modify_target.map.insert(
                MatchTarget::Tag(actions),
                vec![MatchTarget::TagAndOperationMap(name.to_string())],
            );
            spec.modify_target.map.insert(
                MatchTarget::Tag(functions),
                vec![MatchTarget::TagAndOperationMap(name.to_string())],
            );
        }
    }

    pub fn filter(&self, filter: Filter<'_>) -> PathMap {
        let spec = self.spec.borrow();
        PathMap {
            paths: spec.paths.filter(filter),
        }
    }

    fn modify_target(mut request: Request, map: &ModifierMap) -> Request {
        for (mat, modify_vec) in map.map.iter() {
            if mat.matches(&mut request) {
                for modifier in modify_vec.iter() {
                    modifier.modify(&mut request);
                }
            }
        }
        request
    }

    pub fn build(&self, filter: Filter<'_>) -> RequestSet {
        let mut spec = self.spec.borrow_mut();
        let modifier = spec.modify_target.clone();
        let path_map: PathMap = spec.paths.filter(filter).into();

        for (path, path_spec) in path_map.paths.iter() {
            let mut req_map = RequestMap::default();
            req_map.path = path.transform_path();

            if let Some(operation) = path_spec.get.as_ref() {
                let mut request = Parser::modify_target(operation.build(), &modifier);
                request.method = HttpMethod::GET;
                req_map.requests.push_back(request);
            }

            if let Some(operation) = path_spec.post.as_ref() {
                let mut request = Parser::modify_target(operation.build(), &modifier);
                request.method = HttpMethod::POST;
                req_map.requests.push_back(request);
            }

            if let Some(operation) = path_spec.put.as_ref() {
                let mut request = Parser::modify_target(operation.build(), &modifier);
                request.method = HttpMethod::PUT;
                req_map.requests.push_back(request);
            }

            if let Some(operation) = path_spec.patch.as_ref() {
                let mut request = Parser::modify_target(operation.build(), &modifier);
                request.method = HttpMethod::PATCH;
                req_map.requests.push_back(request);
            }

            if let Some(operation) = path_spec.delete.as_ref() {
                let mut request = Parser::modify_target(operation.build(), &modifier);
                request.method = HttpMethod::DELETE;
                req_map.requests.push_back(request);
            }

            if let Some(r) = spec
                .requests
                .iter_mut()
                .find(|r| r.path.eq(req_map.path.as_str()))
            {
                r.requests.extend(req_map.requests);
            } else {
                spec.requests.push_back(req_map);
            }
        }

        let mut request_set = RequestSet::default();
        let mut requests = spec.requests.clone();
        while let Some(req) = requests.pop_front() {
            request_set.join_inner_insert(req);
        }
        request_set
    }
}
