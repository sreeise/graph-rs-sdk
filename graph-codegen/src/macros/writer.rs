use crate::parser::HttpMethod;

/// Helper struct to write out the macros that create the request methods.
/// The macros are located at ./src/client/macros.rs
pub struct MacroWriter;

impl MacroWriter {
    pub fn method_macro(params: usize, has_body: bool, method: HttpMethod) -> String {
        let method_str = {
            match method {
                HttpMethod::GET => "Method::GET",
                HttpMethod::PUT => "Method::PUT",
                HttpMethod::PATCH => "Method::PATCH",
                HttpMethod::POST => "Method::POST",
                HttpMethod::DELETE => "Method::DELETE",
                HttpMethod::TRACE => panic!("Error, illegal trace method"),
            }
        };

        format!("({{ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: {}, has_body: {} }}) => {{
            register_method!(
                {{ doc: $doc, name: $name, response: $T, path: $template, method: {}, params: {}, has_body: {} }}
            );
        }};", params, has_body, method_str, params, has_body)
    }
}
