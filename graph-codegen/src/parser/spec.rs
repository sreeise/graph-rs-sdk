use crate::parser::filter::{Filter, MatchTarget, ModifierMap};
use crate::parser::{HttpMethod, PathMap, Request, RequestMap, RequestSet};
use crate::traits::{RequestParser, RequestParserBuilder};
use from_as::*;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, VecDeque};

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
    modify_target: ModifierMap,
    modifiers: BTreeSet<String>,
}

impl ParserSpec {
    fn filter_split(&self) -> HashMap<String, PathMap> {
        let mut map = HashMap::new();
        for modifier in self.modifiers.iter() {
            let paths = self
                .paths
                .filter(Filter::PathStartsWith(&format!("/{}", modifier)));
            map.insert(modifier.clone(), PathMap { paths });
        }
        map
    }
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
                modify_target: Default::default(),
                modifiers: Default::default(),
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
                modify_target: Default::default(),
                modifiers: Default::default(),
            }),
        }
    }

    pub fn set_path_map(&self, path_map: PathMap) {
        self.spec.borrow_mut().paths = path_map;
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
        let shorthand_name = format!("{}.{}", name, shorthand);
        let double_name = format!("{}.{}", name, name);
        let functions = format!("{}.Functions", name);
        let actions = format!("{}.Actions", name);
        let mut spec = self.spec.borrow_mut();

        spec.modify_target.map.insert(
            MatchTarget::Tag("".to_string()),
            vec![MatchTarget::TagAndOperationMap(name.to_string())],
        );
        spec.modify_target.map.insert(
            MatchTarget::Tag(shorthand_name),
            vec![MatchTarget::TagAndOperationMap(name.to_string())],
        );
        spec.modify_target.map.insert(
            MatchTarget::Tag(double_name),
            vec![MatchTarget::TagAndOperationMap(name.to_string())],
        );
        spec.modify_target.map.insert(
            MatchTarget::Tag(actions),
            vec![MatchTarget::TagAndOperationMap(name.clone())],
        );
        spec.modify_target.map.insert(
            MatchTarget::Tag(functions),
            vec![MatchTarget::TagAndOperationMap(name.clone())],
        );
        spec.modifiers.insert(name.clone());
    }

    pub fn use_default_modifiers(&self, names: &[&str]) {
        let mut spec = self.spec.borrow_mut();

        for name in names.iter() {
            let shorthand = &name[..name.len() - 1];
            let shorthand_name = format!("{}.{}", name, shorthand);
            let double_name = format!("{}.{}", name, name);
            let functions = format!("{}.Functions", name);
            let actions = format!("{}.Actions", name);

            spec.modify_target.map.insert(
                MatchTarget::Tag("".to_string()),
                vec![MatchTarget::TagAndOperationMap(name.to_string())],
            );
            spec.modify_target.map.insert(
                MatchTarget::Tag(shorthand_name),
                vec![MatchTarget::TagAndOperationMap(name.to_string())],
            );
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
            spec.modifiers.insert(name.to_string());
        }
    }

    pub fn filter(&self, filter: Filter<'_>) -> PathMap {
        let spec = self.spec.borrow();
        PathMap {
            paths: spec.paths.filter(filter),
        }
    }

    pub fn multi_filter(&self, filters: Vec<Filter<'_>>) -> PathMap {
        let spec = self.spec.borrow();
        let mut path_map = spec.paths.clone();

        for filter in filters.iter() {
            path_map = PathMap {
                paths: path_map.filter(filter.clone()),
            }
        }

        path_map
    }

    pub fn filter_split(&self) -> HashMap<String, PathMap> {
        self.spec.borrow().filter_split()
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

    pub fn build_with_modifier_filter(&self) -> HashMap<String, RequestSet> {
        let mut spec = self.spec.borrow_mut();
        let modifier = spec.modify_target.clone();
        let modifier_filters = spec.modifiers.clone();
        let mut req_set_map = HashMap::new();

        let operation_mapping_fn = |request: &mut Request, modifier_filter: &str| {
            if request.operation_mapping.is_empty() {
                request.operation_mapping = modifier_filter.to_string();
            }
        };

        for modifier_filter in modifier_filters.iter() {
            let path_map: PathMap = spec
                .paths
                .filter(Filter::PathStartsWith(&format!("/{}", modifier_filter)))
                .into();

            for (path, path_spec) in path_map.paths.iter() {
                let mut req_map = RequestMap::default();
                req_map.path = path.transform_path();

                if let Some(operation) = path_spec.get.as_ref() {
                    let mut request = Parser::modify_target(operation.build(), &modifier);
                    request.method = HttpMethod::GET;
                    operation_mapping_fn(&mut request, modifier_filter.as_ref());
                    req_map.requests.push_back(request);
                }

                if let Some(operation) = path_spec.post.as_ref() {
                    let mut request = Parser::modify_target(operation.build(), &modifier);
                    request.method = HttpMethod::POST;
                    operation_mapping_fn(&mut request, modifier_filter.as_ref());
                    req_map.requests.push_back(request);
                }

                if let Some(operation) = path_spec.put.as_ref() {
                    let mut request = Parser::modify_target(operation.build(), &modifier);
                    request.method = HttpMethod::PUT;
                    operation_mapping_fn(&mut request, modifier_filter.as_ref());
                    req_map.requests.push_back(request);
                }

                if let Some(operation) = path_spec.patch.as_ref() {
                    let mut request = Parser::modify_target(operation.build(), &modifier);
                    request.method = HttpMethod::PATCH;
                    operation_mapping_fn(&mut request, modifier_filter.as_ref());
                    req_map.requests.push_back(request);
                }

                if let Some(operation) = path_spec.delete.as_ref() {
                    let mut request = Parser::modify_target(operation.build(), &modifier);
                    request.method = HttpMethod::DELETE;
                    operation_mapping_fn(&mut request, modifier_filter.as_ref());
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
            req_set_map.insert(modifier_filter.clone(), request_set);
            spec.requests.clear();
        }
        req_set_map
    }
}
