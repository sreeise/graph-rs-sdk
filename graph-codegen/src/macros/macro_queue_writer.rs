use crate::api_types::{
    Metadata, PathMetadata, PathMetadataQueue, RequestClientList, RequestMetadata, RequestTask,
};
use crate::builder::{ClientLinkSettings, RegisterClient};
use crate::inflector::Inflector;
use crate::openapi::OpenApi;
use crate::parser::client_resource::ResourceParsingInfo;
use crate::parser::ParserSettings;
use bytes::{BufMut, BytesMut};
use from_as::*;
use graph_core::resource::ResourceIdentity;
use graph_http::iotools::create_dir;
use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};
use std::convert::TryFrom;
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

/// Writes the macro used for describing requests. This is the outer
/// most macro and is used to describe all requests.
///
/// # Example Macro
/// ```rust,ignore
/// get!({
///     doc: "# Get historyItems from me",
///     name: get_activity_history,
///     response: serde_json::Value,
///     path: "/activities/{{id}}/historyItems/{{id1}}}",
///     params: [ user_activity_id history_items_id ],
///     has_body: false
/// });
/// ```
pub trait MacroQueueWriter {
    type Metadata: Metadata;

    fn request_metadata(&self) -> VecDeque<Self::Metadata>;

    fn request_clients(&self) -> RequestClientList;

    /// The URL path.
    /// Macro type: expr
    fn path(&self) -> String;

    /// A list of parameter names that will be used for the request method.
    fn params(&self) -> &VecDeque<String>;

    fn param_size(&self) -> usize;

    fn parent(&self) -> String;

    fn imports(&self) -> Vec<String>;

    fn macro_params(&self) -> String {
        let mut parameter_str = String::new();
        for param in self.params().iter() {
            parameter_str.push_str(&format!(" {} ", param));
        }
        parameter_str
    }

    fn has_download_methods(&self) -> bool {
        let metadata = self.request_metadata();
        metadata
            .iter()
            .any(|m| m.request_task() == RequestTask::Download)
    }

    fn write_download_macros(&self, is_async_download: bool) -> Option<String> {
        let metadata = self.request_metadata();
        let mut buf = BytesMut::new();
        let mut has_download_methods = false;

        for m in metadata.iter() {
            let request_task = m.request_task();

            if request_task == RequestTask::Download {
                has_download_methods = true;
                let mut doc = String::new();
                if let Some(doc_string) = m.doc() {
                    doc = format!("\n\t\tdoc: \"{}\", ", doc_string);
                }

                let mut name = m.fn_name();
                if name.starts_with("reports_") {
                    name = name.replacen("reports_", "", 1);
                }
                let has_body = m.has_body();
                let mut macro_fn_name = m.macro_fn_name();
                if is_async_download {
                    macro_fn_name = "async_download".to_string();
                }

                let type_name = request_task.type_name();
                let path = self.path();
                let params = self.macro_params();

                buf.put(format!("\n\t{}!({{", macro_fn_name).as_bytes());
                buf.put(doc.as_bytes());
                buf.put(format!("\n\t\tname: {}", name).as_bytes());

                if is_async_download {
                    buf.put(",\n\t\tresponse: AsyncDownload".as_bytes());
                } else {
                    buf.put(format!(",\n\t\tresponse: {}", type_name).as_bytes());
                }

                buf.put(format!(",\n\t\tpath: \"{}\"", path).as_bytes());

                if self.param_size() > 0 {
                    buf.put(format!(",\n\t\tparams: [{}]", params).as_bytes());
                }

                buf.put(format!(",\n\t\thas_body: {}", has_body).as_bytes());
                buf.put("\n\t});".as_bytes());
            }
        }

        if has_download_methods {
            let s = std::str::from_utf8(buf.as_ref()).unwrap();
            Some(s.to_string())
        } else {
            None
        }
    }

    fn write_method_macros(&self) -> String {
        let metadata = self.request_metadata();
        let mut buf = BytesMut::new();

        for m in metadata.iter() {
            let request_task = m.request_task();

            if request_task != RequestTask::Download {
                let mut doc = String::new();
                if let Some(doc_string) = m.doc() {
                    doc = format!("\n\t\tdoc: \"{}\", ", doc_string);
                }

                let mut name = m.fn_name();
                if name.starts_with("reports_") {
                    name = name.replacen("reports_", "", 1);
                }
                let has_body = m.has_body();
                let macro_fn_name = m.macro_fn_name();

                let is_upload = request_task == RequestTask::Upload;
                let is_upload_session = request_task == RequestTask::UploadSession;
                let type_name = request_task.type_name();

                let path = self.path();
                let params = self.macro_params();

                buf.put(format!("\n\t{}!({{", macro_fn_name).as_bytes());
                buf.put(doc.as_bytes());
                buf.put(format!("\n\t\tname: {}", name).as_bytes());

                if !is_upload_session {
                    buf.put(format!(",\n\t\tresponse: {}", type_name).as_bytes());
                }

                buf.put(format!(",\n\t\tpath: \"{}\"", path).as_bytes());

                if self.param_size() > 0 {
                    buf.put(format!(",\n\t\tparams: [{}]", params).as_bytes());
                }

                buf.put(format!(",\n\t\thas_body: {}", has_body).as_bytes());
                if is_upload {
                    buf.put(",\n\t\tupload: true".as_bytes());
                }
                if is_upload_session {
                    buf.put(",\n\t\tupload_session: true".as_bytes());
                }
                buf.put("\n\t});".as_bytes());
            }
        }
        let s = std::str::from_utf8(buf.as_ref()).unwrap();
        s.to_string()
    }

    fn write_impl_macros(&self) {}
}

pub trait MacroImplWriter {
    type Metadata: Debug + Clone + MacroQueueWriter;

    fn path_metadata_queue(&self) -> VecDeque<Self::Metadata>;

    fn request_metadata_queue(&self) -> VecDeque<RequestMetadata>;

    fn path_metadata_map(&self) -> BTreeMap<String, VecDeque<Self::Metadata>>;

    fn default_imports(&self) -> Vec<String>;

    /*
    fn path_metadata_map(&self) -> BTreeMap<String, VecDeque<Self::Metadata>> {
        let metadata = self.path_metadata_queue();
        let mut path_metadata_map: BTreeMap<String, VecDeque<Self::Metadata>> = BTreeMap::new();

        for m in metadata {
            path_metadata_map.entry(m.parent())
                .and_modify(|vec| {
                    vec.push_back(m.clone());
                })
                .or_insert_with(|| {
                    let mut v = VecDeque::new();
                    v.push_back(m.clone());
                    v
                });
        }
        path_metadata_map
    }
     */

    fn id_method(&self, struct_name: String, resource_identity: ResourceIdentity) -> String {
        format!(
            "pub fn id<ID: AsRef<str>>(&self, id: ID) -> {}<'a, Client> {{
            self.client.set_ident({});
            {}::new(id.as_ref(), self.client)
        }}",
            struct_name,
            resource_identity.enum_string(),
            struct_name
        )
    }

    fn create_impl_dir(&self, src_dir: &str) -> File {
        let snake_casing = src_dir.to_snake_case();
        let directory = format!("./src/{}", snake_casing);
        let mod_file = format!("./src/{}/mod.rs", snake_casing);
        let request_file = format!("./src/{}/request.rs", snake_casing);

        println!("Building Client: {:#?}", snake_casing);
        println!("Directory: {:#?}", directory);
        println!("Mod file: {:#?}", mod_file);
        println!("Request file: {:#?}", request_file);

        create_dir(directory).unwrap();

        let mut file1 = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&mod_file)
            .unwrap();
        let mut mod_buf = BytesMut::new();
        mod_buf.put("mod request;\n\npub use request::*;".as_bytes());
        file1.write_all(mod_buf.as_mut()).unwrap();
        file1.sync_data().unwrap();

        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&request_file)
            .unwrap()
    }

    fn get_impl_bytes(&self) -> BytesMut {
        let path_metadata_map = self.path_metadata_map();
        //println!("{:#?}", path_metadata_map);

        let mut buf = BytesMut::new();
        let values: Vec<_> = path_metadata_map.values().cloned().collect();
        let mut imports: HashSet<String> = HashSet::new();

        for path_metadata_queue in values.iter() {
            let current_imports: Vec<String> = path_metadata_queue
                .iter()
                .map(|m| m.imports())
                .flatten()
                .collect();
            imports.extend(current_imports);
        }

        let keys: Vec<_> = path_metadata_map.keys().cloned().collect();
        println!("Keys:\n{:#?}", keys);
        let mut links: BTreeMap<String, BTreeSet<ClientLinkSettings>> = BTreeMap::new();

        for name in keys.iter() {
            if let Ok(resource_identity) = ResourceIdentity::from_str(&name.to_camel_case()) {
                let known_imports = ParserSettings::imports(resource_identity);
                imports.extend(known_imports);
            }
        }

        buf.put("// GENERATED CODE\n\nuse crate::api_default_imports::*;\n".as_bytes());
        for import in imports.iter() {
            buf.put(format!("use {};\n", import).as_bytes());
        }

        buf.put("\n".as_bytes());

        for name in keys {
            if name.contains("Id") {
                let client_struct = RegisterClient::IdentClient.format(name.as_str());
                buf.put(client_struct);
            } else {
                let client_struct = RegisterClient::BaseClient.format(name.as_str());
                buf.put(client_struct);
            }

            if let Ok(resource_identity) = ResourceIdentity::from_str(&name.to_camel_case()) {
                links.extend(ParserSettings::client_link_settings(resource_identity));
            }
        }

        for (name, path_metadata_queue) in path_metadata_map.iter() {
            buf.put(format!(
                "\nimpl<'a, Client> {}Request<'a, Client> where Client: graph_http::RequestClient {{",
                name
            ).as_bytes());

            if let Some(current_links) = links.get(&name.to_camel_case()) {
                for link in current_links.iter() {
                    let s = link.format();
                    buf.put(s.as_bytes());
                    buf.put("\n".as_bytes());
                }
            }

            let mut has_downloads = false;
            for path_metadata in path_metadata_queue.iter() {
                let method_macros = path_metadata.write_method_macros();
                buf.put(method_macros.as_bytes());
                if !has_downloads && path_metadata.has_download_methods() {
                    has_downloads = true;
                }
            }

            buf.put("\n}\n".as_bytes());

            if has_downloads {
                buf.put(
                    format!("\n\n\nimpl<'a> {}Request<'a, BlockingHttpClient> {{", name).as_bytes(),
                );

                for path_metadata in path_metadata_queue.iter() {
                    let download_macros_option = path_metadata.write_download_macros(false);
                    if let Some(download_macros) = download_macros_option.as_ref() {
                        buf.put(download_macros.as_bytes());
                    }
                }

                buf.put(
                    format!(
                        "\n}}\n\n\n\nimpl<'a> {}Request<'a, AsyncHttpClient> {{",
                        name
                    )
                    .as_bytes(),
                );

                for path_metadata in path_metadata_queue.iter() {
                    let download_macros_option = path_metadata.write_download_macros(true);
                    if let Some(download_macros) = download_macros_option.as_ref() {
                        buf.put(download_macros.as_bytes());
                    }
                }

                buf.put("\n}\n".as_bytes());
            }
        }
        buf
    }

    fn get_impl_metadata(resource_parsing_info: ResourceParsingInfo) -> PathMetadataQueue {
        PathMetadataQueue::from(resource_parsing_info)
    }

    /// Writes the rust file for a single resource. Resources can contain multiple secondary resources.
    fn write_impl(&self, src_dir: &str) {
        let mut buf = self.get_impl_bytes();
        let mut request_file = self.create_impl_dir(src_dir);
        request_file.write_all(buf.as_mut()).unwrap();
        request_file.sync_data().unwrap();
        println!("\nDone")
    }
}

pub trait OpenApiParser {
    fn write(resource_parsing_info: ResourceParsingInfo) {
        let name = {
            if let Some(name) = resource_parsing_info.modifier_name.as_ref() {
                name.to_string()
            } else {
                resource_parsing_info.resource_identity.to_string()
            }
        };

        let metadata_queue = PathMetadataQueue::from(resource_parsing_info);
        metadata_queue.debug_print();
        metadata_queue.write_impl(name.as_str());

        let metadata_file = format!(
            "./graph-codegen/src/parsed_metadata/{}.json",
            name.to_snake_case()
        );
        metadata_queue.as_file_pretty(&metadata_file).unwrap();
    }

    fn write_metadata<P: AsRef<Path>>(
        resource_parsing_info: ResourceParsingInfo,
        path: &P,
    ) -> Result<(), FromAsError> {
        let metadata_queue = PathMetadataQueue::from(resource_parsing_info);
        metadata_queue.debug_print();
        let path_buf = path.as_ref().to_path_buf();
        metadata_queue.as_file_pretty(&path_buf)
    }
}
