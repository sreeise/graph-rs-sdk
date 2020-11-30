use crate::builder::{Client, ClientBuilder, ClientLinkSettings};
use crate::parser::filter::{Filter, UrlMatchTarget};
use crate::parser::{
    ApiImpl, Parser, ParserSettings, PathMap, RequestMap, RequestSet, ResourceNames,
};
use from_as::*;
use graph_core::resource::ResourceIdentity;
use graph_http::iotools::IoTools;
use inflector::Inflector;
use std::cell::{Ref, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::convert::TryFrom;
use std::fs::OpenOptions;
use std::io::{Read, Write};
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
    dry_run: bool,
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

    fn add_client_link(&mut self, name: &str, client_link: ClientLinkSettings) {
        let mut set = BTreeSet::new();
        set.insert(client_link);
        self.client_links.insert(name.to_string(), set);
    }

    fn extend_client_link(&mut self, name: &str, client_link: ClientLinkSettings) {
        self.client_links
            .entry(name.to_string())
            .and_modify(|set| set.extend(vec![client_link.clone()]))
            .or_insert_with(|| {
                let mut set = BTreeSet::new();
                set.insert(client_link);
                set
            });
    }

    fn extend_client_links(&mut self, name: &str, client_links: BTreeSet<ClientLinkSettings>) {
        self.client_links
            .entry(name.to_string())
            .and_modify(|set| {
                set.extend(client_links.clone());
            })
            .or_insert(client_links.clone());
    }

    fn extend_client_links_map(
        &mut self,
        client_links: BTreeMap<String, BTreeSet<ClientLinkSettings>>,
    ) {
        for (name, set) in client_links {
            self.client_links
                .entry(name)
                .and_modify(|inner_set| inner_set.extend(set.clone()))
                .or_insert(set);
        }
    }

    fn set_dry_run(&mut self, dry_run: bool) {
        self.dry_run = dry_run;
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
                dry_run: false,
            }),
        }
    }

    pub(crate) fn set_build_with_modifier_filter(&self, build_with_modifier_filter: bool) {
        self.spec.borrow_mut().build_with_modifier_filter = build_with_modifier_filter;
    }

    pub fn filter(&self, filter: Filter<'_>) -> PathMap {
        self.spec.borrow().parser.filter(filter)
    }

    pub fn generate_requests(&self) -> HashMap<String, RequestSet> {
        let spec = self.spec.borrow();
        Builder::parser_build(&spec)
    }

    pub fn set_dry_run(&self, dry_run: bool) {
        self.spec.borrow_mut().set_dry_run(dry_run);
    }

    pub fn use_defaults(&self) {
        let mut spec = self.spec.borrow_mut();
        spec.add_imports(&[
            "crate::client::Graph",
            "graph_http::IntoResponse",
            "reqwest::Method",
        ]);

        let mut ident_clients: HashSet<String> = HashSet::new();

        // Drives does not get added because we use drive instead of drives
        // to create both.
        ident_clients.insert("drives".to_string());

        let modifiers = spec.parser.resource_modifier_set();
        let client_links_override = spec.parser.client_links();

        for resource_target in modifiers.iter() {
            match resource_target {
                UrlMatchTarget::ResourceId(_s, replacement, name) => {
                    ident_clients.insert(name.to_string());
                    let mut client_link = ClientLinkSettings::new(name.as_str());
                    client_link.as_id_method_link();
                    spec.extend_client_link(replacement.as_str(), client_link);
                },
            }
        }

        spec.extend_client_links_map(client_links_override);
        spec.set_ident_clients(ident_clients);
    }

    fn add_custom_clients(
        &self,
        name: &str,
        custom_methods: &HashMap<String, RequestSet>,
        client_map: &mut BTreeMap<String, Client>,
    ) {
        for (name, client) in client_map.iter_mut() {
            if let Some(request_set) = custom_methods.get(name) {
                let methods = request_set.methods();

                for (_name, methods) in methods.iter() {
                    let client_methods: BTreeSet<RequestMap> =
                        methods.into_iter().cloned().collect();
                    client.extend_methods(client_methods);
                }
            }
        }

        if let Some(request_set) = custom_methods.get(name) {
            let request_set = request_set.clone();
            let methods = request_set.methods();

            for (name, methods) in methods.iter() {
                let client_methods: BTreeSet<RequestMap> = methods.into_iter().cloned().collect();

                client_map
                    .entry(name.to_string())
                    .and_modify(|client| client.extend_methods(client_methods.clone()))
                    .or_insert_with(|| {
                        let mut client = Client::new(name.as_str(), client_methods.clone());
                        client
                    });
            }
        }
    }

    // Temporary workaround to deal with the differences in the path
    // for drives when the resource comes from me, users, groups,
    // and sites.
    // For instance, the get_items methods for the drives resource
    // has a path that starts with /drives/drive-id/items
    // but the path for users is /users/user-id/drive/items
    // The users path has drive/items while the drive path is just items.
    fn fix_drive_methods(&self, methods: &mut Vec<RequestMap>) {
        let mat = "/drives/{{RID}}";
        let mat2 = "/drives/{{id}}";

        let empty_root = "{{drive_root}}";
        for request_map in methods {
            if request_map.path.starts_with(mat) || request_map.path.starts_with(mat2) {
                let mut reduce_param_count = false;
                let mat = {
                    if request_map.path.starts_with(mat) {
                        mat
                    } else {
                        reduce_param_count = true;
                        mat2
                    }
                };

                request_map.path = request_map.path.trim_start_matches(mat).to_string();

                if !request_map.path.ends_with("/drives") {
                    if request_map.path.starts_with('/') {
                        request_map.path =
                            format!("{{{{resource_drive_path}}}}{}", request_map.path);
                    } else {
                        request_map.path =
                            format!("{{{{resource_drive_path}}}}/{}", request_map.path);
                    }
                }

                request_map.path = request_map
                    .path
                    .replace("{{id2}}", "{{id}}")
                    .replace("{{id3}}", "{{id2}}")
                    .replace("{{id4}}", "{{id3}}");

                if request_map.path.is_empty() {
                    request_map.path = empty_root.into();
                }

                // Doing the exact same thing with the path in request map but we can't
                // just clone the path to the requests because we have a mutable
                // borrow of the Vec<RequestMap> and we need an immutable borrow
                // to clone.
                for request in request_map.iter_mut() {
                    request.has_rid = false;
                    request.path = request.path.trim_start_matches(mat).to_string();

                    if !request.path.ends_with("/drives") {
                        if request.path.starts_with('/') {
                            request.path = format!("{{{{resource_drive_path}}}}{}", request.path);
                        } else {
                            request.path = format!("{{{{resource_drive_path}}}}/{}", request.path);
                        }
                    }

                    request.path = request
                        .path
                        .replace("{{id2}}", "{{id}}")
                        .replace("{{id3}}", "{{id2}}")
                        .replace("{{id4}}", "{{id3}}");

                    if request.path.is_empty() {
                        request.path = empty_root.into();
                    }

                    if reduce_param_count {
                        if request.param_size > 0 {
                            request.param_size -= 1;
                        }
                    }
                }
            }
        }
    }

    pub fn build_clients(&self) {
        let spec = self.spec.borrow();
        let custom_methods = spec.parser.custom_methods();
        let mut map = Builder::parser_build(&spec);
        let imports = spec.imports.clone();
        let dry_run = spec.dry_run;

        for (name, request_set) in map.iter_mut() {
            if !name.trim().is_empty() {
                println!("Name: {}", name);
                let mut directory_mods = BTreeSet::new();
                let mut request_set_imports = request_set.get_imports();
                request_set_imports.extend(imports.iter().map(|s| s.to_string()));

                if let Ok(resource_identity) = ResourceIdentity::from_str(name.as_str()) {
                    request_set_imports.extend(
                        ParserSettings::imports(resource_identity)
                            .iter()
                            .map(|s| s.to_string()),
                    );

                    if let Some(directory_mod_vec) =
                        ParserSettings::get_directory_mod_files(resource_identity)
                    {
                        directory_mods.extend(directory_mod_vec);
                    }
                }

                let is_ident_client = spec.ident_clients.contains(name);
                if is_ident_client {
                    request_set_imports.insert("handlebars::*".into());
                }
                let imports: BTreeSet<String> = request_set_imports.into_iter().collect();

                let struct_links = request_set.client_links();
                let mut methods: BTreeMap<String, Vec<RequestMap>> = request_set.methods();
                println!("methods: {:#?}", methods);

                // Temporary workaround to deal with the differences in the path
                // for drives when the resource comes from me, users, groups,
                // and sites.
                for (_operation_id, request_map) in methods.iter_mut() {
                    self.fix_drive_methods(request_map);
                }

                let mut clients: BTreeMap<String, Client> = BTreeMap::new();
                for (name, methods) in methods.iter() {
                    let mut client_methods: BTreeSet<RequestMap> =
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

                if let Some(custom_methods) = custom_methods.as_ref() {
                    self.add_custom_clients(name.as_str(), custom_methods, &mut clients);
                }

                for (client_name, client) in clients.iter() {
                    println!("\nClient: {:#?}", client_name);
                    let client_link_settings = client.client_link_settings();
                    println!("Client Link Settings: {:#?}\n", client_link_settings);
                }

                let snake_casing = name.to_snake_case();
                let dir = format!("./src/{}", snake_casing);
                let mod_file = format!("./src/{}/mod.rs", snake_casing);
                let file = format!("./src/{}/request.rs", snake_casing);
                let client_builder = ClientBuilder::new(imports, clients, directory_mods);

                println!("Building Client: {:#?}", snake_casing);
                println!("Directory: {:#?}", dir);
                println!("Mod file: {:#?}", mod_file);
                println!("Request file: {:#?}", file);

                if !dry_run {
                    Builder::write(client_builder, dir, mod_file, file);
                }
            }
        }
    }

    fn write(client_builder: ClientBuilder, dir: String, mod_file: String, request_file: String) {
        IoTools::create_dir(dir).unwrap();

        let mut mod_file_buf = client_builder.build_mod_file();
        let mut file1 = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&mod_file)
            .unwrap();
        file1.write_all(mod_file_buf.as_mut()).unwrap();
        file1.sync_data().unwrap();

        let mut buf = client_builder.build();

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&request_file)
            .unwrap();
        file.write_all(buf.as_mut()).unwrap();
        file.sync_data().unwrap();
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
