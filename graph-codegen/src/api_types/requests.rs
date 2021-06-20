use crate::api_types::{RequestFunction, ResponseBody};
use crate::inflector::Inflector;
use crate::parser::HttpMethod;
use crate::traits::RequestParser;
use from_as::*;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::io::{Read, Write};

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct RequestMetadata {
    pub has_body: bool,
    pub request_function: RequestFunction,
    pub response_body: ResponseBody,
    pub operation_id: String,
    pub operation_mapping: String,
    pub http_method: HttpMethod,
    pub doc: Option<String>,
}

impl RequestMetadata {
    pub fn fn_name(&self) -> String {
        self.operation_id.method_name()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct RequestPathItem {
    pub path: String,
    pub param_size: usize,
    pub parameters: VecDeque<String>,
    pub metadata: VecDeque<RequestMetadata>,
}

impl RequestPathItem {
    pub fn replace_brackets(&mut self) {
        self.path = self.path.replace('{', "{{").replace('}', "}}");
    }

    fn snake_case_parameters(&self) -> VecDeque<String> {
        self.parameters
            .iter()
            .map(|param| param.to_snake_case())
            .collect()
    }

    pub fn format_path_parameters(&mut self) {
        let mut counter = 0;
        let mut path = self.path.clone();

        for param in self.parameters.iter() {
            let param_snake_case = format!("{{{}}}", param.to_snake_case());
            if counter == 0 {
                path = path.replacen(param.as_str(), param_snake_case.as_str(), 1);
            } else {
                path = path.replacen(param.as_str(), param_snake_case.as_str(), 1);
            }
            counter += 1;
        }

        self.path = path;
        self.parameters = self.snake_case_parameters();
    }
}
