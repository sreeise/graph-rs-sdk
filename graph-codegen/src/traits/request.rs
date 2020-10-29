use crate::parser::filter::ModifierMap;
use crate::parser::Request;
use inflector::Inflector;
use regex::Regex;
use std::collections::{HashSet, VecDeque};

lazy_static! {
    static ref NUM_REG: Regex = Regex::new(r"[0-9]").unwrap();
    static ref PATH_ID_REG: Regex = Regex::new(r"[{a-bA-B_}]").unwrap();
}

pub trait RequestParserBuilder<RHS: ?Sized = Self> {
    fn build(&self, modifier: &ModifierMap) -> Request;
}

pub trait RequestParser<RHS = Self> {
    fn method_name(&self) -> String;
    fn operation_mapping(&self) -> String;
    fn transform_path(&self) -> String;
    fn links(&self) -> HashSet<String> {
        Default::default()
    }
}

pub trait Modify<T> {
    fn modify(&self, value: &mut T);
}

impl RequestParser for &str {
    fn method_name(&self) -> String {
        let mut method_name = String::new();
        if let Some(index) = self.rfind('.') {
            let last: &str = self[index + 1..].as_ref();
            if NUM_REG.is_match(last) {
                if let Some(idx) = self[..index].rfind('.') {
                    method_name.push_str(self[idx + 1..index].as_ref());
                }
            } else {
                method_name.push_str(self[index + 1..].as_ref());
            }
        } else {
            method_name.push_str(&self);
        }

        if method_name.is_empty() {
            self.to_snake_case()
        } else {
            method_name.to_snake_case()
        }
    }

    fn operation_mapping(&self) -> String {
        let mut op_mapping = String::new();

        if self.contains('.') {
            let mut ops: Vec<&str> = self.split('.').collect();
            ops.retain(|s| !s.is_empty());

            if let Some(last) = ops.pop() {
                if !NUM_REG.is_match(last) {
                    if ops.len() > 1 {
                        op_mapping = ops.join(".");
                    } else {
                        op_mapping = ops.join("");
                    }
                }
            }
        } else {
            op_mapping = self.to_string();
        }

        if op_mapping.ends_with('.') {
            op_mapping.truncate(op_mapping.len() - 1);
        }

        if op_mapping.matches('.').count() == 1 {
            let mut queue: VecDeque<&str> = op_mapping.split('.').collect();
            let first = queue.pop_front().unwrap();
            let last = queue.pop_front().unwrap();
            if first[..first.len() - 1].eq(last) {
                op_mapping = first.to_string();
            } else if last[..last.len() - 1].eq(first) {
                op_mapping = last.to_string();
            }
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

    fn links(&self) -> HashSet<String> {
        let mut links: HashSet<String> = HashSet::new();

        if self.contains('.') {
            let mut vec: Vec<&str> = self.split('.').collect();
            vec.retain(|s| !s.is_empty());

            let mut iter = vec.iter().peekable();

            while let Some(current) = iter.next() {
                if let Some(next) = iter.peek() {
                    links.insert(format!("{}.{}", current, next));
                }
            }
        } else {
            links.insert(self.to_string());
        }

        links
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

    fn links(&self) -> HashSet<String> {
        self.as_str().links()
    }
}
