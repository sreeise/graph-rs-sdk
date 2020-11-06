use crate::parser::{RequestMap, RequestSet};
use bytes::{BufMut, Bytes, BytesMut};
use inflector::Inflector;
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
pub enum RegisterClient {
    BaseClient,
    IdentClient,
}

impl RegisterClient {
    pub fn format(self, name: &str) -> Bytes {
        Bytes::copy_from_slice(SpecFormatter::register_client(name.as_ref(), self).as_bytes())
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SpecClientImpl {
    name: String,
    links: Vec<String>,
    id_links: BTreeMap<String, String>,
    requests: Vec<RequestMap>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SpecClient {
    // The main client. This will mainly be resource clients
    // such as users, groups, teams, etc.
    pub name: String,
    pub is_ident_client: bool,
    pub imports: BTreeSet<String>,
    // All client names that have requests for the current
    // parent client.
    pub client_names: BTreeSet<String>,
    // Clients that need an id method.
    pub client_name_id_links: BTreeMap<String, String>,
    pub struct_links: BTreeMap<String, Vec<String>>,
    pub methods: BTreeMap<String, Vec<RequestMap>>,
}

impl SpecClient {
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_imports(&mut self, imports: BTreeSet<String>) {
        self.imports = imports;
    }

    pub fn set_client_names(&mut self, client_names: BTreeSet<String>) {
        self.client_names = client_names;
    }

    pub fn set_struct_links(&mut self, struct_links: BTreeMap<String, Vec<String>>) {
        self.struct_links = struct_links;
    }

    pub fn set_methods(&mut self, methods: BTreeMap<String, Vec<RequestMap>>) {
        self.methods = methods;
    }

    pub fn set_ident_client(&mut self, is_ident_client: bool) {
        self.is_ident_client = is_ident_client;
    }

    pub fn set_id_links(&mut self, links: BTreeMap<String, String>) {
        self.client_name_id_links = links;
    }

    pub fn extend_links(&mut self, links_override: &HashMap<String, Vec<String>>) {
        for (key, value) in self.struct_links.iter_mut() {
            if links_override.contains_key(key) {
                value.extend(links_override.get(key).cloned().unwrap());
            }
        }
    }

    fn gen_client_registrations(&self) -> BytesMut {
        let imports_vec: Vec<u8> = self
            .imports
            .iter()
            .map(|s| format!("use {};\n", s).into_bytes())
            .flatten()
            .collect();

        let mut buf = BytesMut::new();
        buf.extend(imports_vec);
        buf.put_u8(b'\n');
        if self.is_ident_client {
            buf.put(RegisterClient::IdentClient.format(self.name.as_str()));
        } else {
            buf.put(RegisterClient::BaseClient.format(self.name.as_str()));
        }

        for name in self.client_names.iter() {
            if self.name.ne(name.as_str()) {
                buf.put(RegisterClient::BaseClient.format(name.as_str()));
            }
        }
        buf
    }

    pub fn gen_api_impl(&self) -> BytesMut {
        let mut buf = self.gen_client_registrations();

        for (name, request_map) in self.methods.iter() {
            if name.contains('.') {
                let mut vec_queue: VecDeque<&str> = name.split('.').collect();

                let links = self
                    .struct_links
                    .get(vec_queue.pop_back().unwrap())
                    .cloned()
                    .unwrap_or_default();

                let id_links = self.client_name_id_links.clone();

                let spec_client_impl = SpecClientImpl {
                    name: name.to_string(),
                    links: links.clone(),
                    id_links,
                    requests: request_map.clone(),
                };
                buf.extend(SpecFormatter::gen_api_impl(spec_client_impl));
            } else {
                let links = self
                    .struct_links
                    .get(name.as_str())
                    .cloned()
                    .unwrap_or_default();

                let id_links = self.client_name_id_links.clone();

                let spec_client_impl = SpecClientImpl {
                    name: name.to_string(),
                    links,
                    id_links,
                    requests: request_map.clone(),
                };
                buf.extend(SpecFormatter::gen_api_impl(spec_client_impl));
            }
        }
        buf
    }
}

impl From<&RequestSet> for SpecClient {
    fn from(request_set: &RequestSet) -> Self {
        let (links, map) = request_set.method_links();
        let operations_mapping = request_set.group_by_operation_mapping();
        let links: BTreeSet<String> = links.into_iter().collect();
        let struct_links: BTreeMap<String, Vec<String>> = map.into_iter().collect();
        let methods: BTreeMap<String, Vec<RequestMap>> = operations_mapping.into_iter().collect();

        let mut spec_client = SpecClient::default();
        spec_client.set_client_names(links);
        spec_client.set_struct_links(struct_links);
        spec_client.set_methods(methods);
        spec_client
    }
}

pub struct SpecFormatter;

impl SpecFormatter {
    pub fn register_client(client_name: &str, register_type: RegisterClient) -> String {
        let ends_with = client_name.ends_with("Request");
        let client_pascal_casing = client_name.to_pascal_case();
        match register_type {
            RegisterClient::BaseClient => {
                if ends_with {
                    format!("register_client!({},);\n", client_pascal_casing)
                } else {
                    format!("register_client!({}Request,);\n", client_pascal_casing)
                }
            },
            RegisterClient::IdentClient => {
                if ends_with {
                    format!("register_client!({}, ());\n", client_pascal_casing)
                } else {
                    format!("register_client!({}Request, ());\n", client_pascal_casing)
                }
            },
        }
    }

    pub fn id_method_link(struct_name: &str) -> String {
        let pascal_casing = struct_name.to_pascal_case();
        format!(
            "\n\tpub fn id<ID: AsRef<str>>(&self, id: ID) -> {}<'a, Client> {{
            \t{}::new(id.as_ref(), self.client)
            }}",
            pascal_casing, pascal_casing,
        )
    }

    pub fn struct_method_link(method_link: &str, struct_name: &str) -> String {
        format!(
            "\n\tpub fn {}(&self) -> {}<'a, Client> {{
            \t{}::new(&self.client)
            }}",
            method_link.to_snake_case(),
            struct_name.to_pascal_case(),
            struct_name.to_pascal_case(),
        )
    }

    pub fn base_struct_name(name: &str) -> String {
        format!("{}Request", name.to_pascal_case())
    }

    pub fn gen_api_impl(spec_client: SpecClientImpl) -> BytesMut {
        let mut buf = BytesMut::with_capacity(1024);
        let mut names: VecDeque<&str> = spec_client.name.split('.').collect();
        let current_client_name = names.pop_back().unwrap();
        let impl_struct_name = SpecFormatter::base_struct_name(current_client_name);

        // #[allow(dead_code)]\n
        let impl_start = format!(
            "\nimpl<'a, Client> {}<'a, Client> where Client: graph_http::RequestClient {{",
            &impl_struct_name
        );
        buf.put(impl_start.as_bytes());

        if let Some(id_link_name) = spec_client.id_links.get(current_client_name) {
            let id_method_link =
                SpecFormatter::id_method_link(&SpecFormatter::base_struct_name(id_link_name));
            buf.put(id_method_link.as_bytes());
        }

        for link in spec_client.links.iter() {
            let method_link = SpecFormatter::struct_method_link(
                link,
                &SpecFormatter::base_struct_name(link.as_str()),
            );
            buf.put(method_link.as_bytes());
        }

        for request_map in spec_client.requests.iter() {
            for request in request_map.requests.iter() {
                if let Some(doc_comment) = request.doc.as_ref() {
                    let method_macro = format!(
                        "\n\t{}!({{\n\t\tdoc: \"{}\",\n\t\tname: {},\n\t\tresponse: {},\n\t\tpath: \"{}\",\n\t\tparams: {},\n\t\thas_body: {}\n\t}});",
                        request.method.as_ref(),
                        doc_comment,
                        request.method_name.as_str(),
                        request.response.as_str(),
                        request_map.path.as_str(),
                        request.param_size,
                        request.has_body
                    );
                    buf.put(method_macro.as_bytes());
                } else {
                    let method_macro = format!(
                        "\n\t{}!({{\n\t\tname: {},\n\t\tresponse: {},\n\t\tpath: \"{}\",\n\t\tparams: {},\n\t\thas_body: {}\n\t}});",
                        request.method.as_ref(),
                        request.method_name.as_str(),
                        request.response.as_str(),
                        request_map.path.as_str(),
                        request.param_size,
                        request.has_body
                    );
                    buf.put(method_macro.as_bytes());
                }
            }
        }
        buf.put_slice(b"\n}\n");
        buf
    }
}
