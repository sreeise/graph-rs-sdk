use crate::builder::spec_formatter::SpecClient;
use crate::parser::filter::Filter;
use crate::parser::{Parser, RequestSet, ResourceNames};
use bytes::BytesMut;
use from_as::*;
use graph_http::iotools::IoTools;
use inflector::Inflector;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::path::Path;

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct SpecBuilder {
    pub(crate) parser: Parser,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    imports: HashSet<String>,
    build_with_modifier_filter: bool,
}

impl SpecBuilder {
    fn add_imports(&mut self, imports: &[&str]) {
        self.imports.extend(imports.iter().map(|s| s.to_string()));
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Builder {
    pub(crate) spec: RefCell<SpecBuilder>,
}

impl Builder {
    pub fn new(parser: Parser) -> Builder {
        Builder {
            spec: RefCell::new(SpecBuilder {
                parser,
                imports: Default::default(),
                build_with_modifier_filter: false,
            }),
        }
    }

    pub fn new_use_mod_filter(parser: Parser) -> Builder {
        Builder {
            spec: RefCell::new(SpecBuilder {
                parser,
                imports: Default::default(),
                build_with_modifier_filter: true,
            }),
        }
    }

    pub(crate) fn set_build_with_modifier_filter(&self, build_with_modifier_filter: bool) {
        self.spec.borrow_mut().build_with_modifier_filter = build_with_modifier_filter;
    }

    pub fn add_imports(&self, imports: &[&str]) {
        self.spec.borrow_mut().add_imports(imports);
    }

    pub fn use_default_imports(&self) {
        self.add_imports(&[
            "crate::client::Graph",
            "graph_http::{GraphResponse, IntoResponse}",
            "reqwest::Method",
        ]);
    }

    pub fn build(&self) {
        let spec = self.spec.borrow();
        // let map = spec.parser.build_with_modifier_filter();

        let map = {
            if spec.build_with_modifier_filter {
                spec.parser.build_with_modifier_filter()
            } else {
                let path_map = spec.parser.path_map();
                let resource_names = ResourceNames::from(path_map);
                let vec = resource_names.to_vec();
                let vec_str: Vec<&str> = vec.iter().map(|s| s.as_str()).collect();
                spec.parser.use_default_modifiers(&vec_str);
                spec.parser.build_with_modifier_filter()
            }
        };

        let imports = spec.imports.clone();
        let links_override = spec.parser.get_links_override();

        for (name, request_set) in map.iter() {
            if !name.trim().is_empty() {
                IoTools::create_dir(format!("./src/{}", name.to_snake_case())).unwrap();

                let mod_file = format!("./src/{}/mod.rs", name.to_snake_case());
                let mut file1 = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(&mod_file)
                    .unwrap();
                file1
                    .write_all("mod request;\n\npub use request::*;".as_bytes())
                    .unwrap();
                file1.sync_all().unwrap();

                let request_file = format!("./src/{}/request.rs", name.to_snake_case());
                Builder::write_internal(
                    request_file,
                    name.to_string(),
                    &imports,
                    &links_override,
                    request_set.clone(),
                );
            }
        }
    }

    fn write_internal<P: AsRef<Path>>(
        file: P,
        parent: String,
        imports: &HashSet<String>,
        links_override: &HashMap<String, Vec<String>>,
        request_set: RequestSet,
    ) {
        let mut buf = BytesMut::with_capacity(1024);
        let mut request_set_imports = request_set.get_imports();
        request_set_imports.extend(imports.iter().map(|s| s.to_string()));
        let (links, mut map) = request_set.method_links();
        let operations_mapping = request_set.group_by_operation_mapping();

        for (key, value) in map.iter_mut() {
            if links_override.contains_key(key) {
                value.extend(links_override.get(key).cloned().unwrap());
            }
        }

        let mut spec_client = SpecClient::default();
        spec_client.set_name(parent.as_str());
        spec_client.set_imports(request_set_imports);
        spec_client.set_client_names(links);
        spec_client.set_struct_links(map);
        spec_client.set_methods(operations_mapping);

        let client_impl = spec_client.gen_api_impl();
        buf.extend(client_impl);

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(file)
            .unwrap();
        file.write_all(buf.as_mut()).unwrap();
        file.sync_all().unwrap();
    }

    pub fn build_with_modifier_filter(&self) -> HashMap<String, RequestSet> {
        self.spec.borrow_mut().parser.build_with_modifier_filter()
    }

    fn gen_spec_client(
        parent: String,
        request_set: RequestSet,
        links_override: HashMap<String, Vec<String>>,
    ) -> SpecClient {
        let mut spec_client = SpecClient::default();

        let (links, mut map) = request_set.method_links();
        let operations_mapping = request_set.group_by_operation_mapping();

        for (key, value) in map.iter_mut() {
            if links_override.contains_key(key) {
                value.extend(links_override.get(key).cloned().unwrap());
            }
        }

        spec_client.set_name(parent.as_str());
        spec_client.set_client_names(links);
        spec_client.set_struct_links(map);
        spec_client.set_methods(operations_mapping);
        spec_client
    }

    pub fn get_clients(&self) -> Vec<SpecClient> {
        let spec = self.spec.borrow();
        let request_set_map = spec.parser.build_with_modifier_filter();
        let links_override = spec.parser.get_links_override();
        let mut spec_clients: Vec<SpecClient> = Vec::new();

        for (name, request_set) in request_set_map.iter() {
            let spec_client = Builder::gen_spec_client(
                name.to_string(),
                request_set.clone(),
                links_override.clone(),
            );
            spec_client.gen_api_impl();
            spec_clients.push(spec_client);
        }

        spec_clients
    }

    pub fn gen_request_set<P: AsRef<Path>>(path: P, parent: &str, request_set: RequestSet) {
        let mut buf = BytesMut::with_capacity(1024);
        let (links, map) = request_set.method_links();
        let operations_mapping = request_set.group_by_operation_mapping();

        let mut spec_client = SpecClient::default();
        spec_client.set_name(parent);
        spec_client.set_client_names(links);
        spec_client.set_struct_links(map);
        spec_client.set_methods(operations_mapping);

        let client_impl = spec_client.gen_api_impl();
        buf.extend(client_impl);

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .unwrap();
        file.write_all(buf.as_mut()).unwrap();
        file.sync_all().unwrap();
    }
}
