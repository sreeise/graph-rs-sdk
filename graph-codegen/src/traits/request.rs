use crate::parser::Request;
use inflector::Inflector;
use regex::Regex;

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

        if self.contains('.') {
            let mut ops: Vec<&str> = self.split('.').collect();
            ops.retain(|s| !s.is_empty());

            if let Some(last) = ops.pop() {
                let re = Regex::new(r"[0-9]").unwrap();
                if !re.is_match(last) && ops.len() > 1 {
                    op_mapping = ops.join(".");
                }
            }
        } else {
            op_mapping = self.to_string();
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
