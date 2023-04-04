use crate::api_types::RequestTask;
use crate::parser::HttpMethod;
use crate::settings::{GeneratedMacroType, MethodMacroModifier};
use from_as::*;
use inflector::Inflector;
use std::convert::TryFrom;
use std::io::{Read, Write};

/// Represents the macro used for describing requests. This is the outer
/// most macro and is used to describe all requests.
///
/// # Example Macro
/// ```rust,ignore
/// get!(
///     doc: "Get historyItems from me",
///     name: get_activity_history,
///     path: "/activities/{{id}}/historyItems/{{id1}}}",
///     params: user_activity_id, history_items_id
/// );
/// ```
#[derive(
    Default, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, FromFile, AsFile,
)]
pub struct MethodMacro {
    pub doc_comment: Option<String>,
    pub fn_name: String,
    pub macro_fn_name: String,
    pub path: String,
    pub params: String,
    pub param_size: usize,
    pub request_task: RequestTask,
    pub has_body: bool,
    pub is_upload: bool,
    pub is_upload_session: bool,
    pub http_method: HttpMethod,
}

impl MethodMacro {
    pub fn response_type_name(&self) -> String {
        self.request_task.type_name().into()
    }

    pub fn matches(&mut self, method_macro_modifier: &MethodMacroModifier) -> bool {
        for method_macro_type in method_macro_modifier.matching.iter() {
            match method_macro_type {
                GeneratedMacroType::FnName(name) => {
                    if self.fn_name.to_snake_case() != name.to_snake_case() {
                        return false;
                    }
                }
                GeneratedMacroType::Path(path) => {
                    if self.path.ne(path) {
                        return false;
                    }
                }
                GeneratedMacroType::ParamSize(param_size) => {
                    if self.param_size.ne(param_size) {
                        return false;
                    }
                }
                GeneratedMacroType::RequestTask(request_task) => {
                    if self.request_task.ne(request_task) {
                        return false;
                    }
                }
                GeneratedMacroType::FnNameAndPath(name, path) => {
                    if self.fn_name.to_snake_case().ne(&name.to_snake_case()) || self.path.ne(path)
                    {
                        return false;
                    }
                }
                GeneratedMacroType::Method(http_method) => {
                    if self.http_method.ne(http_method) {
                        return false;
                    }
                }
                GeneratedMacroType::Default => return false,
            }
        }
        true
    }

    pub fn update(&mut self, method_macro_modifier: &MethodMacroModifier) {
        match method_macro_modifier.update.clone() {
            GeneratedMacroType::FnName(name) => {
                self.fn_name = name.to_string();
            }
            GeneratedMacroType::Path(path) => {
                self.path = path.to_string();
            }
            GeneratedMacroType::ParamSize(param_size) => {
                self.param_size = param_size;
            }
            GeneratedMacroType::RequestTask(request_task) => {
                self.request_task = request_task;
            }
            GeneratedMacroType::FnNameAndPath(name, path) => {
                self.fn_name = name.to_string();
                self.path = path.to_string();
            }
            GeneratedMacroType::Method(http_method) => {
                self.http_method = http_method;
            }
            GeneratedMacroType::Default => {}
        }
    }
}
