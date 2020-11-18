use crate::builder::spec_formatter::SpecClient;
use crate::builder::{Client, ClientBuilder, ClientLinkSettings};
use crate::parser::filter::{Filter, UrlMatchTarget};
use crate::parser::{Parser, ParserSettings, PathMap, RequestMap, RequestSet, ResourceNames};
use bytes::BytesMut;
use from_as::*;
use graph_core::resource::ResourceIdentity;
use graph_http::iotools::IoTools;
use inflector::Inflector;
use std::cell::{Ref, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fs::OpenOptions;
use std::path::Path;
use std::str::FromStr;

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct SpecBuilder {
    pub(crate) parser: Parser,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    imports: HashSet<String>,
    ident_clients: HashSet<String>,
    ident_client_id_links: BTreeMap<String, String>,
    secondary_links: BTreeMap<String, Vec<String>>,
    client_links: BTreeMap<String, BTreeSet<ClientLinkSettings>>,
    build_with_modifier_filter: bool,
}

impl SpecBuilder {
    fn add_imports(&mut self, imports: &[&str]) {
        self.imports.extend(imports.iter().map(|s| s.to_string()));
    }

    fn add_ident_clients(&mut self, ident_clients: &[&str]) {
        self.ident_clients
            .extend(ident_clients.iter().map(|s| s.to_string()));
    }

    fn set_ident_clients(&mut self, ident_clients: HashSet<String>) {
        self.ident_clients = ident_clients;
    }

    fn set_ident_client_id_links(&mut self, ident_client_id_links: BTreeMap<String, String>) {
        self.ident_client_id_links = ident_client_id_links;
    }

    fn set_secondary_links(&mut self, secondary_links: BTreeMap<String, Vec<String>>) {
        self.secondary_links = secondary_links;
    }

    fn add_client_link(&mut self, name: &str, client_link: ClientLinkSettings) {
        let mut set = BTreeSet::new();
        set.insert(client_link);
        self.client_links.insert(name.to_string(), set);
    }

    fn extend_client_links(&mut self, name: &str, client_links: BTreeSet<ClientLinkSettings>) {
        self.client_links
            .entry(name.to_string())
            .and_modify(|set| {
                set.extend(client_links.clone());
            })
            .or_insert(client_links.clone());
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
                ident_clients: Default::default(),
                ident_client_id_links: Default::default(),
                secondary_links: Default::default(),
                client_links: Default::default(),
                build_with_modifier_filter: false,
            }),
        }
    }

    pub fn new_use_mod_filter(parser: Parser) -> Builder {
        Builder {
            spec: RefCell::new(SpecBuilder {
                parser,
                imports: Default::default(),
                ident_clients: Default::default(),
                ident_client_id_links: Default::default(),
                secondary_links: Default::default(),
                client_links: Default::default(),
                build_with_modifier_filter: true,
            }),
        }
    }

    pub(crate) fn set_build_with_modifier_filter(&self, build_with_modifier_filter: bool) {
        self.spec.borrow_mut().build_with_modifier_filter = build_with_modifier_filter;
    }

    pub fn filter(&self, filter: Filter<'_>) -> PathMap {
        self.spec.borrow().parser.filter(filter)
    }

    pub fn add_imports(&self, imports: &[&str]) {
        self.spec.borrow_mut().add_imports(imports);
    }

    pub fn add_ident_clients(&self, ident_clients: &[&str]) {
        self.spec.borrow_mut().add_ident_clients(ident_clients);
    }

    pub fn use_default_imports(&self) {
        self.add_imports(&[
            "crate::client::Graph",
            "graph_http::IntoResponse",
            "reqwest::Method",
        ]);
    }

    pub fn use_defaults(&self) {
        let mut spec = self.spec.borrow_mut();
        spec.add_imports(&[
            "crate::client::Graph",
            "graph_http::IntoResponse",
            "reqwest::Method",
        ]);

        let mut ident_clients: HashSet<String> = HashSet::new();
        let modifiers = spec.parser.resource_modifier_set();
        let client_links_override = spec.parser.client_links();

        for resource_target in modifiers.iter() {
            match resource_target {
                UrlMatchTarget::ResourceId(_s, replacement, name) => {
                    ident_clients.insert(name.to_string());
                    let mut client_link = ClientLinkSettings::new(name.as_str());
                    client_link.as_id_method_link();
                    spec.add_client_link(replacement.as_str(), client_link);
                },
            }
        }

        spec.client_links.extend(client_links_override);
        spec.set_ident_clients(ident_clients);
    }

    pub fn build_clients(&self) {
        let spec = self.spec.borrow();
        let map = Builder::parser_build(&spec);
        let imports = spec.imports.clone();

        for (name, request_set) in map.iter() {
            if !name.trim().is_empty() {
                let mut request_set_imports = request_set.get_imports();
                request_set_imports.extend(imports.iter().map(|s| s.to_string()));

                if let Ok(resource_identity) = ResourceIdentity::from_str(name.as_str()) {
                    request_set_imports.extend(
                        ParserSettings::imports(resource_identity)
                            .iter()
                            .map(|s| s.to_string()),
                    );
                }

                let is_ident_client = spec.ident_clients.contains(name);
                if is_ident_client {
                    request_set_imports.insert("handlebars::*".into());
                }
                let imports: BTreeSet<String> = request_set_imports.into_iter().collect();

                let struct_links = request_set.client_links();
                let methods: BTreeMap<String, Vec<RequestMap>> = request_set.methods();

                let mut clients: BTreeMap<String, Client> = BTreeMap::new();
                for (name, methods) in methods.iter() {
                    let client_methods: BTreeSet<RequestMap> =
                        methods.into_iter().cloned().collect();

                    let mut client = Client::new(name.as_str(), client_methods);

                    if let Some(client_link) = spec.client_links.get(name) {
                        client.extend_client_links(client_link.clone());
                    }

                    if spec.ident_clients.contains(name) {
                        client.set_ident_client(true);
                    }

                    clients.insert(name.to_string(), client);
                }

                for (name, links) in struct_links.iter() {
                    clients.entry(name.to_string()).and_modify(|client| {
                        for link in links.iter() {
                            if !client.get_client_link_setting(link).is_some() &&
                                name.ne(link.as_str())
                            {
                                let link_settings = ClientLinkSettings::new(link.as_str());
                                client.insert_client_link(link_settings);
                            }
                        }
                    });
                }

                for (client_name, client) in clients.iter() {
                    dbg!(client_name);
                    let client_link_settings = client.client_link_settings();
                    for link in client_link_settings.iter() {
                        dbg!(link.name());
                    }
                }

                let snake_casing = name.to_snake_case();
                let dir = format!("./src/{}", snake_casing);
                let mod_file = format!("./src/{}/mod.rs", snake_casing);
                let file = format!("./src/{}/request.rs", snake_casing);
                let mut client_builder = ClientBuilder::new(imports, clients);
                Builder::write(client_builder, dir, mod_file, file);
            }
        }
    }

    fn write(client_builder: ClientBuilder, dir: String, mod_file: String, request_file: String) {
        IoTools::create_dir(dir).unwrap();

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

        let mut buf = client_builder.build();

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(&request_file)
            .unwrap();
        file.write_all(buf.as_mut()).unwrap();
        file.sync_all().unwrap();
    }

    pub fn build(&self) {
        let spec = self.spec.borrow();

        let map = Builder::parser_build(&spec);
        let imports = spec.imports.clone();
        let links_override = spec.parser.get_links_override();

        for (name, request_set) in map.iter() {
            if !name.trim().is_empty() {
                let id_links = spec.ident_client_id_links.clone();
                let is_ident_client = spec.ident_clients.contains(name);
                let secondary_links = spec.secondary_links.clone();

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
                    id_links,
                    secondary_links,
                    request_set.clone(),
                    is_ident_client,
                );
            }
        }
    }

    fn write_internal<P: AsRef<Path>>(
        file: P,
        parent: String,
        imports: &HashSet<String>,
        links_override: &HashMap<String, Vec<String>>,
        id_links: BTreeMap<String, String>,
        secondary_links: BTreeMap<String, Vec<String>>,
        request_set: RequestSet,
        is_ident_client: bool,
    ) {
        let mut buf = BytesMut::with_capacity(1024);
        let mut request_set_imports = request_set.get_imports();
        request_set_imports.extend(imports.iter().map(|s| s.to_string()));
        if is_ident_client {
            request_set_imports.insert("handlebars::*".into());
        }
        let imports: BTreeSet<String> = request_set_imports.into_iter().collect();

        let mut spec_client = SpecClient::from(&request_set);
        spec_client.set_name(parent.as_str());
        spec_client.set_imports(imports);
        spec_client.extend_links(links_override);
        spec_client.set_ident_client(is_ident_client);
        spec_client.set_id_links(id_links);
        spec_client.set_secondary_links(secondary_links);

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
        let spec = self.spec.borrow();
        Builder::parser_build(&spec)
    }

    fn parser_build(spec: &Ref<SpecBuilder>) -> HashMap<String, RequestSet> {
        if spec.build_with_modifier_filter {
            spec.parser.build()
        } else {
            let path_map = spec.parser.path_map();
            let resource_names = ResourceNames::from(path_map);
            let vec = resource_names.to_vec();
            let vec_str: Vec<&str> = vec.iter().map(|s| s.as_str()).collect();
            spec.parser.use_default_modifiers(&vec_str);
            spec.parser.build()
        }
    }

    pub fn generate_resource_names(&self) -> ResourceNames {
        ResourceNames::from(self.spec.borrow().parser.path_map())
    }
}
