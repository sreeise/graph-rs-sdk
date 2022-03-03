use crate::api_types::{Metadata, MetadataModifier, RequestClientList, RequestTask};
use crate::inflector::Inflector;
use crate::macros::{MacroImplWriter, MacroQueueWriter};
use crate::openapi::{OpenApi, PathItem};
use crate::parser::client_resource::ResourceParsingInfo;
use crate::parser::filter::{Filter, ModifierMap};
use crate::parser::{HttpMethod, ParserSettings};
use crate::traits::{FilterMetadata, HashMapExt, RequestParser, INTERNAL_PATH_ID};
use from_as::*;
use graph_core::resource::ResourceIdentity;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::str::FromStr;

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct RequestMetadata {
    pub has_body: bool,
    pub request_task: RequestTask,
    pub operation_id: String,
    pub operation_mapping: String,
    pub http_method: HttpMethod,
    pub doc: Option<String>,
    pub parent: String,
    pub original_parent: String,
    pub resource_identity: Option<ResourceIdentity>,
}

impl RequestMetadata {
    pub fn type_name(&self) -> &'static str {
        self.request_task.type_name()
    }

    pub fn links(&self) -> HashSet<String> {
        self.operation_mapping.links()
    }

    pub fn operation_str_replacen(&mut self, pat: &str, to: &str) {
        self.operation_mapping = self.operation_mapping.replacen(pat, to, 1);
        self.operation_id = self.operation_id.replacen(pat, to, 1);
    }

    pub fn trim_operation_id_start(&mut self, operation_id_start_name: &str) {
        if self.operation_id.starts_with(operation_id_start_name) {
            self.operation_id = self
                .operation_id
                .trim_start_matches(operation_id_start_name)
                .trim_start_matches('.')
                .to_string();
        }

        if self.operation_mapping.starts_with(operation_id_start_name) {
            self.operation_mapping = self
                .operation_mapping
                .trim_start_matches(operation_id_start_name)
                .trim_start_matches('.')
                .to_string();
        }
    }

    pub fn transform_id_request(&mut self) {
        if let Some(rid) = self.resource_identity.as_ref() {
            self.operation_mapping = rid.to_string().to_pascal_case();
            self.parent = self.operation_mapping.to_string();
        } else {
            self.operation_mapping = format!("{}Id", self.operation_mapping.to_pascal_case());
            self.parent = self.operation_mapping.to_string();
        }
    }

    pub fn transform_secondary_id_request(
        &mut self,
        operation_mapping: &str,
        original_parent: &str,
    ) {
        self.operation_mapping = format!("{}Id", operation_mapping);

        if let Some(resource_identity) = self.resource_identity {
            let resource_id_string = resource_identity.to_string();
            self.parent = format!("{}Id", resource_id_string.to_pascal_case());
            self.original_parent = resource_id_string.to_pascal_case();
        } else {
            self.parent = format!("{}Id", original_parent.to_pascal_case());
            self.original_parent = original_parent.to_pascal_case();
            self.resource_identity = ResourceIdentity::from_str(&self.original_parent).ok();
        }
    }

    pub fn transform_secondary_request(&mut self, operation_mapping: &str, original_parent: &str) {
        self.operation_mapping = operation_mapping.to_string();
        self.parent = original_parent.to_pascal_case();
        self.original_parent = original_parent.to_pascal_case();
        self.resource_identity = ResourceIdentity::from_str(&self.original_parent).ok();
    }

    // Replace parts of a doc comment to prevent confusion on apis that are used by
    // multiple resources.
    pub fn filter_doc_comments(&mut self, resource_identity: ResourceIdentity) {
        let filters = ParserSettings::doc_comment_filters(resource_identity);
        for filter in filters.iter() {
            if self.doc.is_some() {
                let doc = self.doc.clone().unwrap();
                self.doc.replace(doc.replacen(filter.as_str(), "", 1));
            }
        }
    }

    pub fn resource_identity(&self) -> Option<ResourceIdentity> {
        self.resource_identity
    }

    pub fn set_resource_identity(&mut self, resource_identity: ResourceIdentity) {
        self.resource_identity = Some(resource_identity);
    }

    pub fn force_resource_identity_mapping(&mut self) {
        if let Some(rid) = self.resource_identity.as_ref() {
            let name = rid.to_string().to_pascal_case();
            self.operation_mapping = name.to_string();
            self.parent = name.to_string();
            self.original_parent = name.to_string();
            self.operation_id = format!("{}.{}", name, self.fn_name());
        }
    }
}

impl Metadata for RequestMetadata {
    fn doc(&self) -> Option<String> {
        self.doc.clone()
    }

    fn http_method(&self) -> HttpMethod {
        self.http_method
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

impl MetadataModifier for RequestMetadata {
    fn replace_operation_mapping(&mut self, replacement: &str) {
        self.operation_mapping = replacement.to_string();
        self.parent = replacement.to_pascal_case();
    }

    fn replace_operation_id(&mut self, replacement: &str) {
        self.operation_id = replacement.to_string();
    }

    fn replace_operation_mapping_n(&mut self, pat: &str, replacement: &str, count: usize) {
        self.operation_mapping = self.operation_mapping.replacen(pat, replacement, count);
    }

    fn replace_operation_id_n(&mut self, pat: &str, replacement: &str, count: usize) {
        self.operation_id = self.operation_id.replacen(pat, replacement, count);
    }

    fn operation_mapping(&self) -> String {
        self.operation_mapping.clone()
    }

    fn operation_id(&self) -> String {
        self.operation_id.clone()
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
        let metadata: VecDeque<RequestMetadata> =
            vec.iter().flat_map(|p| p.metadata.clone()).collect();

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
            .any(|metadata| metadata.operation_id.starts_with(operation_id_start))
    }

    pub fn path_starts_with(&self, path: &str) -> bool {
        self.path.starts_with(path)
    }

    pub fn path_contains(&self, pat: &str) -> bool {
        self.path.contains(pat)
    }

    pub fn trim_operation_id_start(&mut self, operation_id_start_name: &str) {
        self.metadata
            .iter_mut()
            .for_each(|m| m.trim_operation_id_start(operation_id_start_name));
    }

    pub fn trim_path_start(&mut self, path_start: &str) {
        self.path = self.path.trim_start_matches(path_start).to_string();
    }

    #[allow(unused_assignments)]
    pub fn operation_id_start(&mut self, pat: &str) {
        let mut to = String::new();
        if pat.contains('.') {
            let v: Vec<&str> = pat.split('.').collect();
            to = v.last().map(|s| s.to_string()).unwrap();
        } else {
            to = pat.to_string();
        }

        let mut metadata: VecDeque<RequestMetadata> = self
            .metadata
            .iter()
            .filter(|r| r.operation_id.starts_with(pat))
            .cloned()
            .collect();

        for r in metadata.iter_mut() {
            r.operation_str_replacen(pat, to.as_str());
        }

        self.metadata = metadata;
    }

    pub fn filter_doc_comments(&mut self, resource_identity: ResourceIdentity) {
        for m in self.metadata.iter_mut() {
            m.filter_doc_comments(resource_identity);
        }
    }

    pub fn format_named_path_parameters(&mut self) {
        let mut path = self.path.clone();

        for param in self.parameters.iter() {
            let param_snake_case = format!("{{{}}}", param.to_snake_case());
            path = path.replacen(param.as_str(), param_snake_case.as_str(), 1);
        }

        self.path = path;
        self.parameters = self.snake_case_parameters();
    }

    pub fn format_path_parameters(&mut self) {
        self.path = self.path.transform_path();
        self.parameters = self.snake_case_parameters();
    }

    fn update_rid_path(&mut self) {
        let previous_path = self.path.clone();
        let updated_path = self.path.replacen("{{id}}", "{{RID}}", 1);
        if updated_path.ne(previous_path.as_str()) {
            self.path = updated_path;
            self.param_size = INTERNAL_PATH_ID.captures_iter(&self.path).count() - 1;

            while self.parameters.len() > self.param_size {
                self.parameters.pop_front();
            }

            self.path = self.path.shift_path_ids();
        }
    }

    pub fn transform_id_metadata(&mut self) {
        self.update_rid_path();

        for m in self.metadata.iter_mut() {
            m.transform_id_request();
        }
    }

    pub fn transform_secondary_id_metadata(
        &mut self,
        operation_mapping: &str,
        original_parent: &str,
    ) {
        self.update_rid_path();

        for m in self.metadata.iter_mut() {
            m.transform_secondary_id_request(operation_mapping, original_parent);
        }
    }

    pub fn transform_secondary_metadata(&mut self, operation_mapping: &str, original_parent: &str) {
        self.param_size = INTERNAL_PATH_ID.captures_iter(&self.path).count();
        while self.parameters.len() > self.param_size {
            self.parameters.pop_front();
        }
        for m in self.metadata.iter_mut() {
            m.transform_secondary_request(operation_mapping, original_parent);
        }
    }

    pub fn force_resource_identity_mapping(&mut self) {
        for metadata in self.metadata.iter_mut() {
            metadata.force_resource_identity_mapping();
        }
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
        let links = self.links();

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

    pub fn update_targets(&mut self, modifier_map: &ModifierMap) {
        for metadata in self.metadata.iter_mut() {
            metadata.update_targets(modifier_map);
        }
    }

    pub fn set_resource_identity(&mut self, resource_identity: ResourceIdentity) {
        for metadata in self.metadata.iter_mut() {
            metadata.set_resource_identity(resource_identity);
        }
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
        self.metadata
            .get(0)
            .map(|m| m.parent.clone())
            .unwrap_or_default()
    }

    fn imports(&self) -> Vec<String> {
        self.metadata
            .iter()
            .flat_map(|m| m.request_task.imports())
            .map(|s| s.to_string())
            .collect()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct PathMetadataMap(BTreeMap<String, VecDeque<PathMetadata>>);

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct PathItemMap(BTreeMap<String, PathItem>);

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

    pub fn filter_metadata_path(&mut self, filter: Filter) {
        let metadata = self.filter_path(filter);
        self.0 = metadata;
    }

    pub fn filter_doc_comments(&mut self, resource_identity: ResourceIdentity) {
        for m in self.0.iter_mut() {
            m.filter_doc_comments(resource_identity);
        }
    }

    pub fn transform_id_metadata(&mut self, path_start: &str) {
        for path_metadata in self.0.iter_mut() {
            if path_metadata.path_starts_with(&format!("{}/{{{{id}}}}", path_start)) {
                path_metadata.transform_id_metadata();
            }
        }
    }

    pub fn transform_secondary_id_metadata(
        &mut self,
        path_start: &str,
        operation_mapping: &str,
        original_parent: &str,
    ) {
        for path_metadata in self.0.iter_mut() {
            let id_path = format!("{}/{{{{id}}}}", path_start);
            if path_metadata.path_starts_with(&id_path) {
                path_metadata.transform_secondary_id_metadata(operation_mapping, original_parent);
            } else if path_metadata.path_starts_with(path_start) {
                path_metadata.transform_secondary_metadata(operation_mapping, original_parent);
            }
        }
    }

    pub fn sort_by_parent(&self) -> PathMetadataMap {
        let mut set: BTreeMap<String, VecDeque<PathMetadata>> = BTreeMap::new();

        for path_metadata in self.0.iter() {
            let path_metadata_clone = path_metadata.clone();
            let parent = path_metadata.parent();
            set.entry(parent)
                .and_modify(|vec| {
                    vec.push_back(path_metadata_clone);
                })
                .or_insert_with(|| {
                    let mut vec = VecDeque::new();
                    vec.push_back(path_metadata.clone());
                    vec
                });
        }

        PathMetadataMap(set)
    }

    pub fn debug_print(&self) {
        for path_metadata in self.0.iter() {
            dbg!(path_metadata);
        }
    }

    pub fn update_targets(&mut self, modifier_map: &ModifierMap) {
        for metadata in self.0.iter_mut() {
            metadata.update_targets(modifier_map);
        }
    }

    pub fn set_resource_identity(&mut self, resource_identity: ResourceIdentity) {
        for metadata in self.0.iter_mut() {
            metadata.set_resource_identity(resource_identity);
        }
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
            .flat_map(|p| p.metadata.clone())
            .collect()
    }

    fn path_metadata_map(&self) -> BTreeMap<String, VecDeque<Self::Metadata>> {
        self.sort_by_parent().0
    }

    fn default_imports(&self) -> Vec<String> {
        vec!["crate::client::Graph".to_string()]
    }
}

impl FilterMetadata for PathMetadataQueue {
    type Metadata = PathMetadata;

    fn metadata_iter(&self) -> std::collections::vec_deque::IntoIter<Self::Metadata> {
        let vec_dequeue = self.0.clone();
        vec_dequeue.into_iter()
    }
}

impl From<ResourceParsingInfo> for PathMetadataQueue {
    fn from(resource_parsing_info: ResourceParsingInfo) -> Self {
        let open_api = OpenApi::default();
        let requests = open_api.requests();

        let name = {
            if let Some(name) = resource_parsing_info.modifier_name.as_ref() {
                name.to_string()
            } else {
                resource_parsing_info.resource_identity.to_string()
            }
        };

        let path_item_map = PathItemMap(open_api.filter_path(&resource_parsing_info.path));

        let path_item_file = format!(
            "./graph-codegen/src/parsed_metadata/{}_openapi.json",
            name.to_snake_case()
        );
        path_item_map.as_file_pretty(&path_item_file).unwrap();

        let metadata: VecDeque<PathMetadata> = requests
            .iter()
            .filter(|r| r.path_starts_with(&resource_parsing_info.path))
            .cloned()
            .collect();

        let mut metadata_queue = PathMetadataQueue(metadata);

        let modifier_map =
            ParserSettings::target_modifiers(resource_parsing_info.resource_identity);
        metadata_queue.update_targets(&modifier_map);

        if let Some(trim_path_start) = resource_parsing_info.trim_path_start.as_ref() {
            metadata_queue.set_resource_identity(resource_parsing_info.resource_identity);
            let resource_identity_string = resource_parsing_info.resource_identity.to_string();

            metadata_queue.transform_secondary_id_metadata(
                resource_parsing_info.path.as_str(),
                name.as_str(),
                resource_identity_string.as_str(),
            );

            metadata_queue.trim_path_start(trim_path_start.as_str());
        } else {
            metadata_queue.set_resource_identity(resource_parsing_info.resource_identity);
            metadata_queue.transform_id_metadata(resource_parsing_info.path.as_str());
        }

        let filters = ParserSettings::path_filters(resource_parsing_info.resource_identity);
        for filter in filters {
            metadata_queue.filter_metadata_path(filter);
        }
        metadata_queue
    }
}
