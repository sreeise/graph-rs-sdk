use crate::api_types::{
    Metadata, MethodMacro, ModFile, PathMetadataQueue, RequestClientList, RequestMetadata,
    RequestTask,
};
use crate::api_types::{ModWriteConfiguration, WriteConfiguration};
use crate::builder::ClientLinkSettings;
use crate::inflector::Inflector;
use crate::openapi::OpenApi;
use crate::parser::ParserSettings;
use crate::settings::{get_method_macro_modifiers, ResourceSettings};
use anyhow::anyhow;
use bytes::{BufMut, BytesMut};
use from_as::*;
use graph_core::resource::ResourceIdentity;
use graph_error::GraphFailure;
use graph_http::iotools::create_dir;
use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fmt::Write as _;
use std::fmt::{format, Debug};
use std::fs::{File, OpenOptions};
use std::io::Write;
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
            // Clippy lint suggested using write! - TODO verify works
            // parameter_str.push_str(&write!(" {} ", param));
            let _ = write!(parameter_str, " {},", param);
        }

        let params: Vec<String> = self.params().iter().map(|s| s.to_string()).collect();
        let param_str = format!(" {}", params.join(", "));
        param_str
    }

    fn has_download_methods(&self) -> bool {
        let metadata = self.request_metadata();
        metadata
            .iter()
            .any(|m| m.request_task() == RequestTask::Download)
    }

    fn method_macros(&self) -> BTreeSet<MethodMacro>;

    fn write_download_macros(&self, is_async_download: bool) -> Option<String> {
        let metadata = self.request_metadata();
        let mut buf = BytesMut::new();
        let mut has_download_methods = false;

        for m in metadata.iter() {
            let request_task = m.request_task();
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

        if has_download_methods {
            let s = std::str::from_utf8(buf.as_ref()).unwrap();
            Some(s.to_string())
        } else {
            None
        }
    }

    fn write_method_macros(&self) -> String {
        let mut buf = BytesMut::new();
        let method_macros = self.method_macros();

        for method_macro in method_macros.iter() {
            let mut doc = String::new();
            if let Some(doc_string) = method_macro.doc_comment.as_ref() {
                doc = format!("\n\t\tdoc: \"{}\", ", doc_string);
            }

            buf.put(format!("\n\t{}!({{", method_macro.macro_fn_name).as_bytes());
            buf.put(doc.as_bytes());
            buf.put(format!("\n\t\tname: {}", method_macro.fn_name).as_bytes());

            if !method_macro.is_upload_session {
                buf.put(
                    format!(",\n\t\tresponse: {}", method_macro.response_type_name()).as_bytes(),
                );
            }

            buf.put(format!(",\n\t\tpath: \"{}\"", method_macro.path).as_bytes());

            if self.param_size() > 0 {
                buf.put(format!(",\n\t\tparams: [{}]", method_macro.params).as_bytes());
            }
            buf.put(format!(",\n\t\tbody: {}", method_macro.has_body).as_bytes());

            /*
            if method_macro.is_upload {
                buf.put(",\n\t\tupload: true".as_bytes());
            }
            if method_macro.is_upload_session {
                buf.put(",\n\t\tupload_session: true".as_bytes());
            }
             */
            buf.put("\n\t});".as_bytes());
        }

        let s = std::str::from_utf8(buf.as_ref()).unwrap();
        s.to_string()
    }

    fn write_async_default_method_macros(&self) -> String {
        let mut buf = BytesMut::new();
        let method_macros = self.method_macros();

        for method_macro in method_macros.iter() {
            let mut doc = String::new();
            if let Some(doc_string) = method_macro.doc_comment.as_ref() {
                doc = format!("\n\t\tdoc: \"{doc_string}\", ");
            }

            buf.put(format!("\n\t{}!(", method_macro.macro_fn_name).as_bytes());
            buf.put(doc.as_bytes());
            buf.put(format!("\n\t\tname: {}", method_macro.fn_name).as_bytes());

            buf.put(format!(",\n\t\tpath: \"{}\"", method_macro.path).as_bytes());

            if method_macro.has_body {
                buf.put(format!(",\n\t\tbody: {}", method_macro.has_body).as_bytes());
            }

            if self.param_size() > 0 {
                buf.put(format!(",\n\t\tparams:{}", method_macro.params).as_bytes());
            }

            /*
            if method_macro.is_upload {
                buf.put(",\n\t\tupload: true".as_bytes());
            }
            if method_macro.is_upload_session {
                buf.put(",\n\t\tupload_session: true".as_bytes());
            }
             */
            buf.put("\n\t);".as_bytes());
        }

        let s = std::str::from_utf8(buf.as_ref()).unwrap();
        s.to_string()
    }

    fn list_method_macros(&self, resource_identity: ResourceIdentity) -> BTreeSet<MethodMacro> {
        let mut set = BTreeSet::new();
        let metadata = self.request_metadata();

        let method_macro_settings = get_method_macro_modifiers(resource_identity);

        for m in metadata.iter() {
            let request_task = m.request_task();
            let is_upload = request_task == RequestTask::Upload;
            let is_upload_session = request_task == RequestTask::UploadSession;

            let mut method_macro = MethodMacro {
                doc_comment: m.doc(),
                fn_name: m.fn_name(),
                macro_fn_name: m.macro_fn_name(),
                path: self.path(),
                params: self.macro_params(),
                param_size: self.param_size(),
                request_task,
                has_body: m.has_body(),
                is_upload,
                is_upload_session,
                http_method: m.http_method(),
            };

            for setting in method_macro_settings.iter() {
                if method_macro.matches(setting) {
                    method_macro.update(setting);
                }
            }

            set.insert(method_macro);
        }

        set
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
        let directory = format!("./src/{snake_casing}");
        let mod_file = format!("./src/{snake_casing}/mod.rs");
        let request_file = format!("./src/{snake_casing}/request.rs");

        println!("Building Client: {snake_casing:#?}");
        println!("Directory: {directory:#?}");
        println!("Mod file: {mod_file:#?}");
        println!("Request file: {request_file:#?}");

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

    fn create_impl_dir_override(&self, mod_write_configuration: ModWriteConfiguration) -> File {
        let folder = mod_write_configuration.folder_path.to_string();
        let snake_casing = mod_write_configuration.folder_name.to_snake_case();
        let mod_name_snake = mod_write_configuration.mod_name.to_snake_case();
        let directory = format!("./src/{folder}");
        let mod_file = format!("./src/{snake_casing}/{mod_name_snake}/mod.rs");
        let request_file = format!("./src/{snake_casing}/{mod_name_snake}/request.rs");

        println!("Building Client: {snake_casing:#?}");
        println!("Directory: {directory:#?}");
        println!("Mod file: {mod_file:#?}");
        println!("Request file: {request_file:#?}");

        create_dir(directory).unwrap();

        let mut file1 = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(&mod_file)
            .unwrap();
        let mut mod_buf = BytesMut::new();
        mod_buf.put("mod request;\npub use request::*;".as_bytes());

        file1.write_all(mod_buf.as_mut()).unwrap();
        file1.sync_data().unwrap();

        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&request_file)
            .unwrap()
    }

    fn get_impl_bytes(&self) -> anyhow::Result<BytesMut> {
        let mut v = BTreeSet::new();
        let mut buf = BytesMut::new();
        let mut imports: HashSet<String> = HashSet::new();
        let path_metadata_map = self.path_metadata_map();
        let keys: Vec<_> = path_metadata_map.keys().cloned().collect();
        let values: Vec<_> = path_metadata_map.values().cloned().collect();

        dbg!(&keys);

        for path_metadata_queue in values.iter() {
            let current_imports: Vec<String> = path_metadata_queue
                .iter()
                .flat_map(|m| m.imports())
                .collect();
            imports.extend(current_imports);
        }

        let ris: VecDeque<(String, ResourceIdentity)> = keys
            .iter()
            .map(|key| {
                (
                    key.to_pascal_case(),
                    ResourceIdentity::from_str(&key.to_camel_case().replace("Id", "")).unwrap(),
                )
            })
            .collect();

        let settings: VecDeque<ResourceSettings> = ris
            .iter()
            .map(|(name, ri)| ResourceSettings::new(name.as_str(), *ri))
            .collect();

        for resource_setting in settings.iter() {
            imports.extend(resource_setting.imports.clone());
        }

        buf.put("// GENERATED CODE\n\nuse crate::api_default_imports::*;\n".as_bytes());
        for import in imports.iter() {
            buf.put(format!("use {import};\n").as_bytes());
        }

        buf.put("\n".as_bytes());

        let client_names: Vec<String> = keys
            .iter()
            .map(|name| format!("{}ApiClient", name.to_pascal_case()))
            .collect();

        // Build ApiClientLink enum to add the client being generated as a method link from one client
        // to another. This is for ease of use and doesnt always work for ever client name.
        for (name, _) in ris.iter() {
            let client_name = format!("{}ApiClient", name.to_pascal_case());
            if client_name.contains("Id") {
                let mut method_name = name.to_snake_case();
                if method_name.ends_with("s_id") {
                    method_name = method_name.replacen("s_id", "", 1);
                }
                v.insert(format!(
                    "ApiClientLink::StructId(\"{}\".into(), \"{}\".into()),",
                    method_name, client_name
                ));
            } else {
                v.insert(format!(
                    "ApiClientLink::Struct(\"{}\".into(), \"{}\".into()),",
                    name.to_snake_case(),
                    client_name
                ));
            }
        }

        let client_impl_string = {
            if keys.len() > 1 {
                client_names.join(", ")
            } else {
                client_names.join("")
            }
        };

        //println!("client_impl_string? {:#?}", client_impl_string);

        if keys.is_empty() || client_impl_string.is_empty() {
            dbg!("Missing keys for client impl.");
            println!("{ris:#?}\n{client_names:#?}\n{client_impl_string:#?}",);
            return Err(anyhow!(
                "Missing keys for client impl. keys: {:#?} client_impl_string: {:#?}",
                keys,
                client_impl_string
            ));
        }

        let resource_api_client_impl = format!(
            "resource_api_client!({}, {});\n",
            client_impl_string,
            settings[0].ri.enum_string()
        );
        buf.put(resource_api_client_impl.as_bytes());

        for (name, path_metadata_queue) in path_metadata_map.iter() {
            let api_client_name = format!("{}ApiClient", name);
            buf.put(format!("\nimpl {} {{", api_client_name).as_bytes());

            let mut set = HashSet::new();
            for setting in settings.iter() {
                for link in setting.api_client_links.iter() {
                    if let Some(api_client) = link.0.as_ref() {
                        if api_client.eq(&api_client_name) {
                            for api_client_link in link.1.iter() {
                                let s = api_client_link.format();
                                set.insert(s);
                            }
                        }
                    } else {
                        for api_client_link in link.1.iter() {
                            let s = api_client_link.format();
                            set.insert(s);
                        }
                    }
                }
            }

            for value in set {
                buf.put(value.as_bytes());
                buf.put("\n".as_bytes());
            }

            for path_metadata in path_metadata_queue.iter() {
                let method_macros = path_metadata.write_async_default_method_macros();
                buf.put(method_macros.as_bytes());
            }

            buf.put("\n}\n".as_bytes());
        }

        for api_client_link in v.iter() {
            println!("Suggested Client Links To Use:\n{}", api_client_link);
        }

        Ok(buf)
    }

    fn get_impl_metadata(resource_parsing_info: WriteConfiguration) -> PathMetadataQueue {
        PathMetadataQueue::from(resource_parsing_info)
    }

    fn get_metadata_method_macros(
        &self,
        resource_parsing_info: WriteConfiguration,
    ) -> BTreeSet<MethodMacro> {
        let path_metadata_map = self.path_metadata_map();
        let mut set = BTreeSet::new();

        for (_name, path_metadata_queue) in path_metadata_map.iter() {
            for metadata in path_metadata_queue.iter() {
                let mut method_macro_set =
                    metadata.list_method_macros(resource_parsing_info.resource_identity);
                set.append(&mut method_macro_set);
            }
        }
        set
    }

    /// Writes the rust file for a single resource. Resources can contain multiple secondary resources.
    fn write_impl(&self, src_dir: &str) {
        if let Ok(mut buf) = self.get_impl_bytes() {
            let mut request_file = self.create_impl_dir(src_dir);
            request_file.write_all(buf.as_mut()).unwrap();
            request_file.sync_data().unwrap();
        }
    }

    fn write_impl_override(&self, mod_write_configuration: ModWriteConfiguration) {
        if let Ok(mut buf) = self.get_impl_bytes() {
            let mut request_file = self.create_impl_dir_override(mod_write_configuration);
            request_file.write_all(buf.as_mut()).unwrap();
            request_file.sync_data().unwrap();
        }
    }

    fn write_mod_file(&self, mod_file: &ModFile) {
        mod_file.write();

        match mod_file {
            ModFile::Declarations { file, declarations } => {
                if let Ok(mut buf) = self.get_impl_bytes() {
                    let request_file = file.replace("mod.rs", "request.rs");
                    //println!("Request file: {:#?}", request_file);
                    let mut f = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .create(true)
                        .open(&request_file)
                        .unwrap();

                    f.write_all(buf.as_mut()).unwrap();
                    f.sync_data().unwrap();
                    //println!("\nDone")
                }
            }
            _ => {}
        }
    }
}

struct ParIterWrite {
    open_api: OpenApi,
    write_configuration: WriteConfiguration,
}

pub trait OpenApiParser {
    fn write(mut write_configuration: WriteConfiguration) {
        let name = {
            if let Some(name) = write_configuration.modifier_name.as_ref() {
                name.to_string()
            } else {
                write_configuration.resource_identity.to_string()
            }
        };

        write_configuration.implement_children_mods();

        let open_api = OpenApi::default();

        let mut par_iter_writes: Vec<ParIterWrite> = Vec::new();
        for write_config in write_configuration.children.iter() {
            let mut open_api2 = open_api.clone();

            if let Some(start_path) = write_config.trim_path_start.as_ref() {
                open_api2.paths = open_api2.filter_path(start_path);
                open_api2.paths = open_api2.filter_path_contains(&write_configuration.path);
            }
            OpenApi::write_using(write_config.clone(), &open_api2);
        }

        let mut open_api2 = open_api.clone();
        open_api2.paths = open_api2.filter_path_contains(&write_configuration.path);
        let metadata_queue = PathMetadataQueue::from((write_configuration.clone(), &open_api2));
        dbg!(&metadata_queue);

        if let Some(mod_write_configuration) = write_configuration.mod_write_override.as_ref() {
            metadata_queue.write_impl_override(mod_write_configuration.clone());
            // dbg!(mod_write_configuration);
        } else if let Some(mod_file) = write_configuration.mod_file.as_ref() {
            metadata_queue.write_mod_file(mod_file);
        } else {
            metadata_queue.write_impl(name.as_str());
        }

        if let Some(mod_file_writer) = write_configuration.mod_file_writer.as_ref() {
            mod_file_writer.write();
        }

        let metadata_file = format!(
            "./graph-codegen/src/parsed_metadata/{}.json",
            name.to_snake_case()
        );

        metadata_queue.as_file_pretty(&metadata_file).unwrap();

        let resource_parsing_info_file = format!(
            "./graph-codegen/src/parsed_metadata/{}_parsing_info.json",
            name.to_snake_case()
        );

        write_configuration
            .as_file_pretty(&resource_parsing_info_file)
            .unwrap();
    }

    fn write_using(mut write_configuration: WriteConfiguration, open_api: &OpenApi) {
        let name = {
            if let Some(name) = write_configuration.modifier_name.as_ref() {
                name.to_string()
            } else {
                write_configuration.resource_identity.to_string()
            }
        };

        write_configuration.implement_children_mods();

        let metadata_queue = PathMetadataQueue::from((write_configuration.clone(), open_api));
        dbg!(&metadata_queue);

        if let Some(mod_write_configuration) = write_configuration.mod_write_override.as_ref() {
            metadata_queue.write_impl_override(mod_write_configuration.clone());
            // dbg!(mod_write_configuration);
        } else if let Some(mod_file) = write_configuration.mod_file.as_ref() {
            metadata_queue.write_mod_file(mod_file);
        } else {
            metadata_queue.write_impl(name.as_str());
        }

        if let Some(mod_file_writer) = write_configuration.mod_file_writer.as_ref() {
            mod_file_writer.write();
        }

        let metadata_file = format!(
            "./graph-codegen/src/parsed_metadata/{}.json",
            name.to_snake_case()
        );

        metadata_queue.as_file_pretty(&metadata_file).unwrap();

        let resource_parsing_info_file = format!(
            "./graph-codegen/src/parsed_metadata/{}_parsing_info.json",
            name.to_snake_case()
        );

        write_configuration
            .as_file_pretty(&resource_parsing_info_file)
            .unwrap();
    }

    fn write_all(write_configurations: Vec<WriteConfiguration>) {
        let open_api = OpenApi::default();
        for write_configuration in write_configurations {
            let mut open_api2 = open_api.clone();
            if let Some(start_path) = write_configuration.trim_path_start.as_ref() {
                open_api2.paths = open_api2.filter_path(start_path);
                open_api2.paths = open_api2.filter_path_contains(&write_configuration.path);
            }
            OpenApi::write_using(write_configuration, &open_api2);
        }
    }

    /// Use only for top-level resources. Otherwise use `write`.
    fn write_resource(resource_identity: ResourceIdentity) {
        let name = resource_identity.to_string();
        let metadata_queue = PathMetadataQueue::from(resource_identity);
        metadata_queue.debug_print();
        metadata_queue.write_impl(name.as_str());

        let metadata_file = format!(
            "./graph-codegen/src/parsed_metadata/{}.json",
            name.to_snake_case()
        );
        metadata_queue.as_file_pretty(&metadata_file).unwrap();
    }

    fn write_metadata<P: AsRef<Path>>(
        resource_parsing_info: WriteConfiguration,
        path: &P,
    ) -> Result<(), FromAsError> {
        let metadata_queue = PathMetadataQueue::from(resource_parsing_info);
        metadata_queue.debug_print();
        let path_buf = path.as_ref().to_path_buf();
        metadata_queue.as_file_pretty(&path_buf)
    }

    fn get_metadata_method_macros(
        resource_parsing_info: WriteConfiguration,
    ) -> BTreeSet<MethodMacro> {
        let metadata_queue = PathMetadataQueue::from(resource_parsing_info.clone());
        metadata_queue.get_metadata_method_macros(resource_parsing_info)
    }
}
