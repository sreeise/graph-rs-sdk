use crate::api_types::RequestTask;
use crate::parser::HttpMethod;
use crate::settings::{MacroModifierType, MethodMacroModifier};
use from_as::*;
use inflector::Inflector;
use std::convert::TryFrom;
use std::io::{Read, Write};

/// Represents the macro used for describing requests. This is the outer
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
        let mut is_match = true;
        for method_macro_type in method_macro_modifier.matching.iter() {
            match method_macro_type {
                MacroModifierType::FnName(name) => {
                    if self.fn_name.to_snake_case() != name.to_snake_case() {
                        return false;
                    }
                }
                MacroModifierType::Path(path) => {
                    if self.path.ne(path.as_str()) {
                        return false;
                    }
                }
                MacroModifierType::ParamSize(param_size) => {
                    if self.param_size.ne(param_size) {
                        return false;
                    }
                }
                MacroModifierType::RequestTask(request_task) => {
                    if self.request_task.ne(request_task) {
                        return false;
                    }
                }
                MacroModifierType::FnNameAndPath(name, path) => {
                    if self.fn_name.to_snake_case().ne(&name.to_snake_case())
                        || self.path.ne(path.as_str())
                    {
                        return false;
                    }
                }
                MacroModifierType::Method(http_method) => {
                    if self.http_method.ne(http_method) {
                        return false;
                    }
                }
            }
        }
        is_match
    }

    pub fn update(&mut self, method_macro_modifier: &MethodMacroModifier) {
        match method_macro_modifier.update.clone() {
            MacroModifierType::FnName(name) => {
                self.fn_name = name;
            }
            MacroModifierType::Path(path) => {
                self.path = path;
            }
            MacroModifierType::ParamSize(param_size) => {
                self.param_size = param_size;
            }
            MacroModifierType::RequestTask(request_task) => {
                self.request_task = request_task;
            }
            MacroModifierType::FnNameAndPath(name, path) => {
                self.fn_name = name;
                self.path = path;
            }
            MacroModifierType::Method(http_method) => {
                self.http_method = http_method;
            }
        }
    }
}
