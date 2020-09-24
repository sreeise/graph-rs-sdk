use crate::parser::RequestMap;
use bytes::{BufMut, BytesMut};
use graph_http::iotools::IoTools;
use inflector::Inflector;
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SpecClientImpl {
    name: String,
    links: Vec<String>,
    requests: Vec<RequestMap>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SpecClient {
    name: String,
    client_names: HashSet<String>,
    struct_links: HashMap<String, Vec<String>>,
    methods: HashMap<String, Vec<RequestMap>>,
}

impl SpecClient {
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_client_names(&mut self, client_names: HashSet<String>) {
        self.client_names = client_names;
    }

    pub fn set_struct_links(&mut self, struct_links: HashMap<String, Vec<String>>) {
        self.struct_links = struct_links;
    }

    pub fn set_methods(&mut self, methods: HashMap<String, Vec<RequestMap>>) {
        self.methods = methods;
    }

    pub fn create_dir(&self) {
        IoTools::create_dir(format!("./src/{}", self.name.to_snake_case())).unwrap();
    }

    fn gen_client_registrations(&self) -> BytesMut {
        let mut buf = BytesMut::new();
        let pascal_casing = SpecFormatter::base_struct_name(self.name.as_str());
        let parent_register = SpecFormatter::register_client(pascal_casing.as_str());

        buf.put(parent_register.as_bytes());
        buf.put_slice(b"\n");

        for name in self.client_names.iter() {
            let name_str = name.as_str();
            if self.name.ne(name_str) {
                let client_pascal_casing = SpecFormatter::struct_name(self.name.as_str(), name_str);
                buf.put(SpecFormatter::register_client(client_pascal_casing.as_str()).as_bytes());
                buf.put_slice(b"\n");
            }
        }
        buf
    }

    pub fn gen_api_impl(&self) -> BytesMut {
        let mut buf = self.gen_client_registrations();

        for (name, request_map) in self.methods.iter() {
            let links = {
                if let Some(l) = self.struct_links.get(name.as_str()) {
                    l.clone()
                } else {
                    vec![]
                }
            };

            let spec_client_impl = SpecClientImpl {
                name: name.to_string(),
                links,
                requests: request_map.clone(),
            };

            buf.extend(SpecFormatter::gen_api_impl(
                self.name.as_str(),
                spec_client_impl,
            ));
        }
        buf
    }
}

pub struct SpecFormatter;

impl SpecFormatter {
    pub fn register_client(client_name: &str) -> String {
        format!("register_client!({},);\n", client_name.to_pascal_case())
    }

    pub fn register_client_single(parent: &str, client_name: &str) -> String {
        format!(
            "register_client!({},);\n",
            SpecFormatter::struct_name(parent, client_name)
        )
    }

    pub fn register_clients(set: HashSet<String>) -> Vec<String> {
        let mut client_registers = Vec::new();
        for name in set.iter() {
            client_registers.push(SpecFormatter::register_client(name.as_str()));
        }
        client_registers
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

    pub fn struct_name(parent: &str, name: &str) -> String {
        if (parent.eq(name)) ||
            (parent.ends_with('s') &&
                name.to_pascal_case()
                    .starts_with(&parent[..parent.len() - 1]))
        {
            format!("{}Request", name.to_pascal_case())
        } else if !parent.ends_with('s') &&
            !name
                .to_pascal_case()
                .starts_with(parent.to_pascal_case().as_str())
        {
            format!(
                "{}{}Request",
                parent.to_pascal_case(),
                name.to_pascal_case()
            )
        } else {
            format!("{}Request", name.to_pascal_case())
        }
    }

    pub fn gen_api_impl(parent: &str, spec_client: SpecClientImpl) -> BytesMut {
        let mut buf = BytesMut::with_capacity(1024);
        let pascal_casing = SpecFormatter::struct_name(parent, spec_client.name.as_str());

        let impl_start = format!(
            "\nimpl<'a, Client> {}<'a, Client> where Client: graph_http::RequestClient {{\n",
            &pascal_casing
        );
        buf.put(impl_start.as_bytes());

        for link in spec_client.links.iter() {
            let method_link = SpecFormatter::struct_method_link(
                link,
                &SpecFormatter::struct_name(parent, link.as_str()),
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
