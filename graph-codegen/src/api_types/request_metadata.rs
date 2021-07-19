use crate::api_types::{Metadata, RequestTask};
use crate::inflector::Inflector;
use crate::macros::{MacroFormatter, MacroQueueWriter};
use crate::parser::HttpMethod;
use crate::traits::RequestParser;
use from_as::*;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::io::{Read, Write};

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct RequestMetadata {
    pub has_body: bool,
    pub request_task: RequestTask,
    pub operation_id: String,
    pub operation_mapping: String,
    pub http_method: HttpMethod,
    pub doc: Option<String>,
}

impl RequestMetadata {
    pub fn type_name(&self) -> &'static str {
        self.request_task.type_name()
    }
}

impl Metadata for RequestMetadata {
    fn doc(&self) -> Option<String> {
        self.doc.clone()
    }

    fn http_method(&self) -> HttpMethod {
        self.http_method.clone()
    }

    fn fn_name(&self) -> String {
        self.operation_id.method_name()
    }

    fn request_task(&self) -> RequestTask {
        self.request_task.clone()
    }

    fn has_body(&self) -> bool {
        self.has_body
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct PathMetadata {
    pub path: String,
    pub param_size: usize,
    pub parameters: VecDeque<String>,
    pub metadata: VecDeque<RequestMetadata>,
}

impl PathMetadata {
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

impl MacroQueueWriter for PathMetadata {
    type Metadata = RequestMetadata;

    fn request_metadata(&self) -> VecDeque<Self::Metadata> {
        self.metadata.clone()
    }

    fn path(&self) -> String {
        self.path.clone()
    }

    fn params(&self) -> &VecDeque<String> {
        &self.parameters
    }
}

pub struct PathMetadataQueue(VecDeque<PathMetadata>);

impl PathMetadataQueue {
    pub fn new() -> PathMetadataQueue {
        PathMetadataQueue(VecDeque::new())
    }
}

impl From<VecDeque<PathMetadata>> for PathMetadataQueue {
    fn from(queue: VecDeque<PathMetadata>) -> Self {
        PathMetadataQueue(queue)
    }
}
