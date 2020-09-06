use crate::parser::Operation;
use from_as::*;
use inflector::Inflector;
use regex::Regex;
use std::collections::{BTreeSet, HashSet, VecDeque};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, FromFile, AsFile, Eq, PartialEq, Hash)]
pub enum HttpMethod {
    GET,
    PUT,
    POST,
    DELETE,
    PATCH,
    TRACE,
}

impl Default for HttpMethod {
    fn default() -> Self {
        HttpMethod::GET
    }
}

impl AsRef<str> for HttpMethod {
    fn as_ref(&self) -> &str {
        match self {
            HttpMethod::GET => "get",
            HttpMethod::PUT => "put",
            HttpMethod::POST => "post",
            HttpMethod::DELETE => "delete",
            HttpMethod::PATCH => "patch",
            HttpMethod::TRACE => "trace",
        }
    }
}

impl From<HttpMethod> for reqwest::Method {
    fn from(method: HttpMethod) -> Self {
        match method {
            HttpMethod::GET => reqwest::Method::GET,
            HttpMethod::PUT => reqwest::Method::PUT,
            HttpMethod::POST => reqwest::Method::POST,
            HttpMethod::DELETE => reqwest::Method::DELETE,
            HttpMethod::PATCH => reqwest::Method::PATCH,
            HttpMethod::TRACE => reqwest::Method::TRACE,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, FromFile, AsFile, Hash)]
pub struct Request {
    pub method: HttpMethod,
    pub method_name: String,
    pub param_size: usize,
    pub has_body: bool,
    pub has_rid: bool,
    pub response: String,
    pub tag: String,
    pub operation_id: String,
    pub operation_mapping: String,
}

impl Request {
    pub fn build(&self, base_url: &str, path: &str) -> reqwest::Request {
        let mut url = reqwest::Url::parse(base_url).unwrap();
        url.set_path(path);
        reqwest::Request::new(self.method.into(), url)
    }
}

impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        self.method == other.method &&
            self.param_size == other.param_size &&
            self.has_body == other.has_body &&
            self.operation_id == other.operation_id
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, FromFile, AsFile, Hash)]
pub struct RequestMap {
    pub path: String,
    pub requests: VecDeque<Request>,
}

impl PartialEq for RequestMap {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(other.path.as_str())
    }
}

impl Eq for RequestMap {}

impl RequestMap {
    pub fn build(&self) -> VecDeque<reqwest::Request> {
        let path = self.path.as_str();
        let host = "https://graph.microsoft.com/v1.0";
        self.requests.iter().map(|r| r.build(host, path)).collect()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct RequestSet {
    pub set: HashSet<RequestMap>,
}

impl RequestSet {
    pub fn insert(&mut self, request_map: RequestMap) {
        self.set.insert(request_map);
    }

    pub fn join_inner_insert(&mut self, request_map: RequestMap) {
        if self.set.contains(&request_map) {
            let mut req_map = self.set.get(&request_map).cloned().unwrap();
            for request in request_map.requests.iter() {
                if req_map.requests.iter().find(|r| r.eq(&request)).is_none() {
                    req_map.requests.push_back(request.clone());
                }
            }

            self.set.insert(req_map);
        } else {
            self.set.insert(request_map);
        }
    }
}

pub trait RequestParserBuilder<RHS: ?Sized = Self> {
    fn build(&self) -> Request;
}

pub trait RequestParser<RHS = Self> {
    fn method_name(&self) -> String;
    fn operation_mapping(&self) -> String;
    fn transform_path(&self) -> String;
}

impl RequestParser for &str {
    fn method_name(&self) -> String {
        let mut method_name = String::new();
        if let Some(index) = self.rfind('.') {
            let last: &str = self[index + 1..].as_ref();
            let re = Regex::new(r"[0-9]").unwrap();
            if re.is_match(last) {
                if let Some(idx) = self[..index].rfind('.') {
                    method_name.push_str(self[idx + 1..index].as_ref());
                }
            } else {
                method_name.push_str(self[index + 1..].as_ref());
            }
        } else {
            method_name.push_str(&self);
        }
        method_name.to_snake_case()
    }

    fn operation_mapping(&self) -> String {
        let mut op_mapping = String::new();
        let mut set: BTreeSet<&str> = BTreeSet::new();
        if self.matches('.').count() >= 1 {
            let ops: Vec<&str> = self.split('.').collect();
            if let Some(last) = ops.last() {
                let re = Regex::new(r"[0-9]").unwrap();
                if !re.is_match(last) && ops.len() > 1 {
                    for op in ops.iter() {
                        if op.len() > 1 {
                            let mut chars = op.chars();
                            if let Some(c) = chars.next() {
                                if !c.is_uppercase() {
                                    set.insert(op);
                                }
                            }
                        }
                    }
                }
            }
        }

        for s in set.iter() {
            op_mapping.push_str(s);
            op_mapping.push('.');
        }

        if op_mapping.ends_with('.') {
            op_mapping.truncate(op_mapping.len() - 1);
        }
        op_mapping
    }

    fn transform_path(&self) -> String {
        self.replace("({id})", "/{{id}}")
            .replace("({id1})", "/{{id2}}")
            .replace("({id2})", "/{{id3}}")
            .replace("({id3})", "/{{id4}}")
            .replace("({id4})", "/{{id5}}")
            .replace("({id5})", "/{{id6}}")
    }
}

impl RequestParser for String {
    fn method_name(&self) -> String {
        self.as_str().method_name()
    }

    fn operation_mapping(&self) -> String {
        self.as_str().operation_mapping()
    }

    fn transform_path(&self) -> String {
        self.as_str().transform_path()
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
        unimplemented!()
    }
}

impl RequestParserBuilder for Operation {
    fn build(&self) -> Request {
        let mut request = Request::default();
        request.operation_id = self.operation_id.to_string();
        request.operation_mapping = self.operation_mapping();
        request.method_name = self.method_name();
        request.param_size = self.param_size();
        request.has_body = self.has_body();
        if let Some(tag) = self.tags.get(0) {
            request.tag = tag.to_string();
        }
        request
    }
}
