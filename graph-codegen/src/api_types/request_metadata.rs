use crate::api_types::{Metadata, RequestTask, RequestClientList};
use crate::inflector::Inflector;
use crate::macros::{MacroFormatter, MacroQueueWriter, MacroImplWriter};
use crate::parser::HttpMethod;
use crate::traits::{RequestParser, HashMapExt};
use from_as::*;
use std::collections::{VecDeque, HashSet, BTreeSet, HashMap, BTreeMap};
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
    pub parent: String,
}

impl RequestMetadata {
    pub fn type_name(&self) -> &'static str {
        self.request_task.type_name()
    }

    pub fn links(&self) -> HashSet<String> {
        self.operation_mapping.links()
    }

    pub fn operation_str_replacen(&mut self, pat: &str, to: &str ) {
        self.operation_mapping = self.operation_mapping.replacen(pat, to, 1);
        self.operation_id = self.operation_id.replacen(pat, to, 1);
    }

    pub fn trim_operation_id_start(&mut self, operation_id_start_name: &str) {
        if self.operation_id.starts_with(operation_id_start_name) {
            self.operation_id = self.operation_id.trim_start_matches(operation_id_start_name).to_string();
            self.operation_id = self.operation_id.trim_start_matches('.').to_string();
        }

        if self.operation_mapping.starts_with(operation_id_start_name) {
            self.operation_mapping = self.operation_mapping.trim_start_matches(operation_id_start_name).to_string();
            self.operation_mapping = self.operation_mapping.trim_start_matches('.').to_string();
        }
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

    fn parent(&self) -> String {
        self.parent.to_string()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct PathMetadata {
    pub path: String,
    pub param_size: usize,
    pub parameters: VecDeque<String>,
    pub metadata: VecDeque<RequestMetadata>,
}

impl From<VecDeque<PathMetadata>> for RequestClientList {
    fn from(vec: VecDeque<PathMetadata>) -> Self {
        let metadata: VecDeque<RequestMetadata> = vec.iter().map(|p| p.metadata.clone())
            .flatten()
            .collect();

        RequestClientList::from(metadata)
    }
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

    pub fn contains_operation_id_start(&self, operation_id_start: &str) -> bool {
        self.metadata
            .iter()
            .find(|metadata| metadata.operation_id.starts_with(operation_id_start))
            .is_some()
    }

    pub fn path_starts_with(&self, path: &str) -> bool {
        self.path.starts_with(path)
    }

    pub fn trim_operation_id_start(&mut self, operation_id_start_name: &str) {
        self.metadata
            .iter_mut()
            .for_each(|m| m.trim_operation_id_start(operation_id_start_name));
    }

    pub fn trim_path_start(&mut self, path_start: &str) {
        self.path = self.path.trim_start_matches(path_start).to_string();
    }

    pub fn operation_id_start(&mut self, pat: &str) {
        let mut to = String::new();
        if pat.contains('.') {
            let v: Vec<&str> = pat.split('.').collect();
            to = v.last().map(|s| s.to_string()).unwrap();
        } else {
            to = pat.to_string();
        }

        let mut metadata: VecDeque<RequestMetadata> = self.metadata
            .iter()
            .filter(|r| r.operation_id.starts_with(pat))
            .cloned()
            .collect();

        for r in metadata.iter_mut() {
            r.operation_str_replacen(pat, to.as_str());
            println!("{:#?}", r);
        }

        self.metadata = metadata;
    }

    pub fn format_named_path_parameters(&mut self) {
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

    pub fn format_path_parameters(&mut self) {
        self.path = self.path.transform_path();
        self.parameters = self.snake_case_parameters();
    }

    pub fn request_client_list(&self) -> RequestClientList {
       self.metadata.clone().into()
    }

    pub fn operation_map_set(&self) -> BTreeSet<String> {
        let mut set = BTreeSet::new();
        for metadata in self.metadata.iter() {
            set.extend(metadata.links());
        }
        set
    }

    pub fn links(&self) -> BTreeSet<String> {
        let mut set = BTreeSet::new();
        for metadata in self.metadata.iter() {
            set.extend(metadata.links());
        }
        set
    }

    /// Creates a hash map of each struct and the client structs
    /// it links too.
    ///
    /// # Example
    ///
    /// Say we have the following operation id's or operation mappings:
    ///     groups.calendar.calendarView
    ///     groups.calendarView
    ///     groups.drive
    ///
    /// {
    ///     "groups": [
    ///         "calendar",
    ///         "calendarView",
    ///         "drive"
    ///     ],
    ///     "calendar": [
    ///         "calendarView"
    ///     ]
    /// }
    pub fn struct_links(&mut self) -> HashMap<String, Vec<String>> {
        let mut links = self.links();

        println!("links btreeset: {:#?}", links);

        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut vec: Vec<&str> = links.iter().map(|s| s.as_str()).collect();
        vec.sort_unstable();

        for link in vec.iter() {
            if link.contains('.') {
                let mut vec: VecDeque<&str> = link.split('.').collect();
                vec.retain(|l| !l.is_empty());
                let first = vec.pop_front().unwrap();
                let last = vec.pop_front().unwrap();
                map.entry_modify_insert(first.to_string(), last.to_string());
            } else {
                map.insert(link.to_string(), vec![]);
            }
        }

        map
    }
}

impl MacroQueueWriter for PathMetadata {
    type Metadata = RequestMetadata;

    fn request_metadata(&self) -> VecDeque<Self::Metadata> {
        self.metadata.clone()
    }

    fn request_clients(&self) -> RequestClientList {
        RequestClientList::from(self.metadata.clone())
    }

    fn path(&self) -> String {
        self.path.clone()
    }

    fn params(&self) -> &VecDeque<String> {
        &self.parameters
    }

    fn param_size(&self) -> usize {
        self.param_size
    }

    fn parent(&self) -> String {
        self.metadata.get(0)
            .map(|m| m.parent.clone())
            .unwrap_or_default()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct PathMetadataQueue(VecDeque<PathMetadata>);

impl PathMetadataQueue {
    pub fn new() -> PathMetadataQueue {
        PathMetadataQueue(VecDeque::new())
    }

    pub fn trim_operation_id_start(&mut self, operation_id_start_name: &str) {
        self.0
            .iter_mut()
            .for_each(|m| m.trim_operation_id_start(operation_id_start_name));
    }

    pub fn trim_path_start(&mut self, path_start: &str) {
        self.0
            .iter_mut()
            .for_each(|m| m.trim_path_start(path_start));
    }
}

impl From<VecDeque<PathMetadata>> for PathMetadataQueue {
    fn from(queue: VecDeque<PathMetadata>) -> Self {
        PathMetadataQueue(queue)
    }
}

impl MacroImplWriter for PathMetadataQueue {
    type Metadata = PathMetadata;

    fn path_metadata_queue(&self) -> VecDeque<PathMetadata> {
        self.0.clone()
    }

    fn request_metadata_queue(&self) -> VecDeque<RequestMetadata> {
        self.path_metadata_queue()
            .iter()
            .map(|p| p.metadata.clone())
            .flatten()
            .collect()
    }
}
