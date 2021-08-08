use crate::api_types::{Metadata, RequestMetadata, RequestTask};
use crate::parser::HttpMethod;
use bytes::{BufMut, BytesMut};
use graph_http::RequestAttribute::RequestType;
use std::collections::{HashSet, VecDeque};

/*
TODO:

 The goal is to make parameter names for request methods the same as they are in the path:

 Given the list of parameter names that were originally available in the path of the request
 we can set the parameter names on each method.

 The two macros we need are the register_method macro:

     ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident $p1:ident ], has_body: false } ) => {
        #[doc = $doc]
        pub fn $name<S: AsRef<str>>(&'a self, $p: S, $p1: S)-> IntoResponse<'a, $T, Client>
        {
            self.client.request()
                .set_method($m);


            render_path!(
                    self.client,
                    $template,
                    &serde_json::json!({ "id": $p.as_ref(), "id1": $p1.as_ref() }));
            IntoResponse::new(&self.client.request)
        }
    };


 and the http method macros such as:

     ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p $p1 ], has_body: false }
        );
    };

 This will allow us to write the request macro with the parameter names:

    get!({
        doc: "# Get historyItems from me",
        name: get_activity_history,
        response: serde_json::Value,
        path: "/activities/{{id}}/historyItems/{{id1}}}",
        params: [ user_activity_id history_items_id ],
        has_body: false
    });

 When syntax highlighting takes place the user will see the parameter names
 instead of id and id1.
 */

/// Helper struct to write out the macros that create the request methods.
/// The macros are located at ./src/client/macros.rs
pub struct MacroFormatter;

impl MacroFormatter {
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

        format!(
            "({{ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, \
             params: {}, has_body: {} }}) => {{
            register_method!(
                {{ doc: $doc, name: $name, response: $T, path: $template, method: {}, params: {}, \
             has_body: {} }}
            );
        }};",
            params, has_body, method_str, params, has_body
        )
    }

    fn param_bounds(param_size: usize, has_body: bool) -> &'static str {
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

    fn render_path(param_size: usize) -> String {
        let mut s = String::new();
        for i in 0..param_size {
            if i == 0 {
                s.push_str(" \"id\": $p.as_ref()");
            } else {
                s.push_str(&format!(", \"id{}\": $p{}.as_ref()", i, i));
            }
        }

        format!(
            "render_path!(
                self.client,
                $template,
                &serde_json::json!({{{ } }})\
            );",
            s
        )
    }

    fn http_method_params(param_size: usize, doc: bool, has_body: bool) -> String {
        let mut s = String::new();
        for i in 0..param_size {
            if i == 0 {
                s.push_str(" $p:ident ");
            } else {
                s.push_str(&format!("$p{}:ident ", i));
            }
        }

        if doc {
            if param_size > 0 {
                format!("( {{ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [{}], has_body: {} }} )", s, has_body)
            } else {
                format!("( {{ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, has_body: {} }} )", has_body)
            }
        } else {
            if param_size > 0 {
                format!("( {{ name: $name:ident, response: $T:ty, path: $template:expr, params: [{}], has_body: {} }} )", s, has_body)
            } else {
                format!("( {{ name: $name:ident, response: $T:ty, path: $template:expr, has_body: {} }} )", has_body)
            }
        }
    }

    fn register_method_call(macro_name: &str, param_size: usize, method: HttpMethod, has_body: bool) -> String {
        let mut inner_params = String::new();
        for i in 0..param_size {
            if i == 0 {
                inner_params.push_str(" $p ");
            } else {
                inner_params.push_str(&format!("$p{} ", i));
            }
        }

        if param_size > 0 {
            format!(
                "{}!(
                        {{ doc: $doc, name: $name, response: $T, path: $template, method: {}, params: [{}], has_body: {} }}
                    );", macro_name, method.enum_name(), inner_params, has_body
            )
        } else {
            format!(
                "{}!(
                    {{ doc: $doc, name: $name, response: $T, path: $template, method: {}, has_body: {} }}
                );", macro_name, method.enum_name(), has_body
            )
        }
    }

    fn http_method_macro(
        macro_name: &str,
        param_size: usize,
        method: HttpMethod,
        doc: bool,
        has_body: bool,
    ) -> String {
        let params = MacroFormatter::http_method_params(param_size, doc, has_body);
        let register_method_call =
            MacroFormatter::register_method_call(macro_name, param_size, method, has_body);
        format!(
            "{} => {{
                {}
            }};",
            params, register_method_call
        )
    }

    fn macro_parameters(param_size: usize, doc: bool, has_body: bool) -> String {
        let mut s = String::new();
        for i in 0..param_size {
            if i == 0 {
                s.push_str(" $p:ident ");
            } else {
                s.push_str(&format!("$p{}:ident ", i));
            }
        }

        if doc {
            if param_size > 0 {
                format!("( {{ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [{}], has_body: {} }} )", s, has_body)
            } else {
                format!("( {{ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, has_body: {} }} )", has_body)
            }
        } else {
            if param_size > 0 {
                format!("( {{ name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [{}], has_body: {} }} )", s, has_body)
            } else {
                format!("( {{ name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, has_body: {} }} )", has_body)
            }
        }
    }

    pub fn register_all_http_macros(param_size: usize, method: HttpMethod) -> String {
        let mut buf = BytesMut::new();

        for i in 0..param_size {
            let r1 = MacroFormatter::http_method_macro("api_method", i, method, true, false);
            let r2 = MacroFormatter::http_method_macro("api_method", i, method, true, true);
            let r3 = MacroFormatter::http_method_macro("api_method", i, method, false, true);
            let r4 = MacroFormatter::http_method_macro("api_method", i, method, false, false);

            buf.put(r1.as_bytes());
            buf.put("\n\n".as_bytes());
            buf.put(r2.as_bytes());
            buf.put("\n\n".as_bytes());
            buf.put(r3.as_bytes());
            buf.put("\n\n".as_bytes());
            buf.put(r4.as_bytes());
            buf.put("\n\n".as_bytes());
        }

        let s = std::str::from_utf8(buf.as_ref()).unwrap();
        s.to_string()
    }

    pub fn register_method_macros(param_size: usize, doc: bool, has_body: bool) -> String {
        let macro_params = MacroFormatter::macro_parameters(param_size, doc, has_body);
        let bounds_with_body = MacroFormatter::param_bounds(param_size, has_body);
        let params = MacroFormatter::parameters(param_size, has_body);
        let render_path = MacroFormatter::render_path(param_size);

        let doc_comment = {
            if doc {
                "#[doc = $doc]"
            } else {
                ""
            }
        };

        let serialize_body = {
            if has_body {
                "if let Err(err) = client.set_body_with_serialize(body) {
                return IntoResponse::new_error(self.client.request(), err);
            }"
            } else {
                ""
            }
        };

        format!(
            "{} => {{
    {}
    pub fn $name{}{}-> IntoResponse<'a, $T, Client>
    {{
        self.client.request()
            .set_method($m);
        {}

        {}
        IntoResponse::new(&self.client.request)
    }}
}};",
            macro_params, doc_comment, bounds_with_body, params, serialize_body, render_path
        )
    }

    pub fn register_all_methods(param_size: usize) -> String {
        let mut buf = BytesMut::new();

        for i in 0..param_size {
            let r1 = MacroFormatter::register_method_macros(i, true, false);
            let r2 = MacroFormatter::register_method_macros(i, true, true);
            let r3 = MacroFormatter::register_method_macros(i, false, true);
            let r4 = MacroFormatter::register_method_macros(i, false, false);

            buf.put(r1.as_bytes());
            buf.put("\n\n".as_bytes());
            buf.put(r2.as_bytes());
            buf.put("\n\n".as_bytes());
            buf.put(r3.as_bytes());
            buf.put("\n\n".as_bytes());
            buf.put(r4.as_bytes());
            buf.put("\n\n".as_bytes());
        }

        let s = std::str::from_utf8(buf.as_ref()).unwrap();
        s.to_string()
    }
}
