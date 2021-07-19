use crate::api_types::{Metadata, RequestTask};
use bytes::{BufMut, BytesMut};
use std::collections::VecDeque;

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

    /// The URL path.
    /// Macro type: expr
    fn path(&self) -> String;

    /// A list of parameter names that will be used for the request method.
    fn params(&self) -> &VecDeque<String>;

    fn parameter_bounds(param_size: usize, has_body: bool) -> &'static str {
        if param_size > 0 {
            match has_body {
                false => "<S: AsRef<str>>",
                true => "<S: AsRef<str>, B: serde::Serialize>",
            }
        } else if has_body {
            "<B: serde::Serialize>"
        } else {
            ""
        }
    }

    fn parameters(param_size: usize, has_body: bool) -> String {
        let mut s = String::from("(&'a self");

        for i in 0..param_size {
            if i == 0 {
                s.push_str(", $p: S");
            } else {
                s.push_str(&format!(", $p{}: S", i));
            }
        }

        if has_body {
            s.push_str(", body: &B");
        }
        s.push(')');
        s
    }

    fn macro_params(&self) -> String {
        let mut parameter_str = String::new();
        for param in self.params().iter() {
            parameter_str.push_str(&format!(" {} ", param));
        }
        parameter_str
    }

    fn write(&self) -> String {
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
            buf.put(format!(",\n\t\tparams: [{}]", params).as_bytes());
            if has_body {
                buf.put(format!(",\n\t\thas_body: {}", has_body).as_bytes());
            }
            if is_upload {
                buf.put(",\n\t\tupload: true".as_bytes());
            }
            if is_upload_session {
                buf.put(",\n\t\tupload_session: true".as_bytes());
            }
            buf.put("\n\t}});".as_bytes());
        }
        let s = std::str::from_utf8(buf.as_ref()).unwrap();
        s.to_string()
    }
}
