use crate::api_types::{Metadata, RequestTask, RequestClientList, PathMetadata, RequestMetadata, PathMetadataQueue};
use bytes::{BufMut, BytesMut};
use std::collections::{VecDeque, BTreeMap, HashSet, HashMap};
use crate::inflector::Inflector;
use crate::traits::RequestParser;
use std::fmt::Debug;
use crate::builder::RegisterClient;
use graph_core::resource::ResourceIdentity;
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

    fn macro_params(&self) -> String {
        let mut parameter_str = String::new();
        for param in self.params().iter() {
            parameter_str.push_str(&format!(" {} ", param));
        }
        parameter_str
    }

    fn write_method_macros(&self) -> String {
        let metadata = self.request_metadata();
        let mut buf = BytesMut::new();

        for m in metadata.iter() {
            let mut doc = String::new();
            if let Some(doc_string) = m.doc() {
                doc = format!("\n\t\tdoc: \"{}\", ", doc_string);
            }

            let name = m.fn_name();
            let has_body = m.has_body();
            let macro_fn_name = m.macro_fn_name();
            let request_task = m.request_task();
            let is_upload = request_task == RequestTask::Upload;
            let is_upload_session = request_task == RequestTask::UploadSession;
            let type_name = request_task.type_name();

            let path = self.path();
            let params = self.macro_params();

            buf.put(format!("\n\t{}!({{", macro_fn_name).as_bytes());
            buf.put(doc.as_bytes());
            buf.put(format!("\n\t\tname: {}", name).as_bytes());
            buf.put(format!(",\n\t\tresponse: {}", type_name).as_bytes());
            buf.put(format!(",\n\t\tpath: {}", path).as_bytes());

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
        let s = std::str::from_utf8(buf.as_ref()).unwrap();
        s.to_string()
    }

    fn write_impl_macros(&self) {

    }
}

pub trait MacroImplWriter {
    type Metadata: Debug + Clone + MacroQueueWriter;

    fn path_metadata_queue(&self) -> VecDeque<Self::Metadata>;

    fn request_metadata_queue(&self) -> VecDeque<RequestMetadata>;

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

    /// Writes the rust file for a single resource. Resources can contain
    /// multiple secondary resources.
    // TODO
    fn write_impl(&self) {

        let mut path_metadata_map = self.path_metadata_map();
        println!("{:#?}", path_metadata_map);

        let mut buf = BytesMut::new();

        for (name, path_metadata_queue) in path_metadata_map.iter() {
            /*
            let resource_identity: ResourceIdentity = ResourceIdentity::from_str(name.to_camel_case().as_str()).unwrap();
            let client_struct = RegisterClient::from_resource_identity(resource_identity);
            buf.put(client_struct.as_bytes());
             */
            let client_struct = RegisterClient::BaseClient.format(name.as_str());
            buf.put(client_struct);

            let impl_start = format!(
                "\nimpl<'a, Client> {}Request<'a, Client> where Client: graph_http::RequestClient {{",
                name
            );

            buf.put(impl_start.as_bytes());

            for path_metadata in path_metadata_queue.iter() {
                let method_macros = path_metadata.write_method_macros();
                buf.put(method_macros.as_bytes());
            }

            buf.put("\n}".as_bytes());
        }
        let s = std::str::from_utf8(buf.as_ref()).unwrap();
        println!("{}", s);

    }
}
