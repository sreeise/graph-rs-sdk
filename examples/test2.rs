extern crate core;

use from_as::*;
use graph_codegen::openapi::OpenApiRaw;
use graph_codegen::traits::API_METHOD_MACRO;
use graph_rs_sdk::client::Graph;
use openapiv3::OpenAPI;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Read;
/*
use graph_http::api_impl::*;
use graph_rs_sdk::client::GraphV2;
use handlebars::Handlebars;

#[derive(Debug, Clone, Eq, PartialEq)]
struct NewClientTest {
    client: graph_http::client::Client,
    resource_config: graph_http::ResourceConfig,
    registry: Handlebars,
}
    let mut client = GraphV2::new("").admin();
    let handler = client
        .get_admin("admin_id".to_string())
        .select(&["id", "name"]);
    println!("{:#?}", handler.url());
 */

fn use_openapi_v3() {
    let open_api_raw = OpenApiRaw::default();
    let data = include_str!("example_files/agreements.json");
    let openapi: OpenAPI = serde_json::from_value(open_api_raw.open_api).unwrap();
    //println!("{:?}", openapi);

    for (path, reference_or_path_item) in openapi.paths.paths.iter() {
        if path.starts_with("/agreements") {
            println!("{:#?}", path);

            if let Some(item) = reference_or_path_item.as_item() {
                println!("{:#?}", item);
                for (path, operation) in item.iter() {
                    for (status, reference_or_response) in operation.responses.responses.iter() {
                        println!(
                            "Status: {:#?}\nReference Or Response: {:#?}",
                            status, reference_or_response
                        );
                    }
                }
            }
        }
    }
}

fn use_openapi_v3_components() {
    let open_api_raw = OpenApiRaw::default();
    let data = include_str!("example_files/agreements.json");
    let openapi: OpenAPI = serde_json::from_value(open_api_raw.open_api).unwrap();
    //println!("{:?}", openapi);

    println!(
        "{:#?}",
        openapi
            .components
            .unwrap()
            .schemas
            .entry("microsoft.graph.agreement".to_string())
    );
}

fn test_macro() {
    let client = Graph::new("token");

    let request = client.admin().get_admin();

    println!("{:#?}", request.url());
}

#[derive(Debug)]
struct ParsedApiMethod {
    method: Option<reqwest::Method>,
    doc: Option<String>,
    name: String,
    path: String,
    body: bool,
    params: Option<String>,
}

impl ParsedApiMethod {
    pub fn write_out_method(&self) -> String {
        if self.method.is_none() {
            panic!("Method is none");
        }

        let method = self.method.clone().unwrap();
        let mut doc = self.doc.clone().unwrap_or(Default::default());
        if !doc.is_empty() {
            doc = doc.clone();
        }
        let name = self.name.clone();
        let mut path = self.path.clone().replace(",", "");
        let mut body = {
            if self.body == true {
                "\nbody: true\n".to_string()
            } else {
                "".to_string()
            }
        };
        let mut params = self.params.clone().unwrap_or(Default::default());

        if !params.is_empty() || !body.is_empty() {
            path = format!("{},", path);
        }

        if !body.is_empty() && !params.is_empty() {
            body = format!("{},", body);
        }

        if !params.is_empty() {
            params = format!("\n{}\n", params);
        }

        format!(
            "{}\n{}\n{}\n{}{}{}\n);",
            format_method(method),
            doc,
            name,
            path,
            body,
            params
        )
    }
}

impl Default for ParsedApiMethod {
    fn default() -> Self {
        ParsedApiMethod {
            method: None,
            doc: None,
            name: Default::default(),
            path: Default::default(),
            body: false,
            params: None,
        }
    }
}

fn update_api_methods() {
    let mut s = r#"
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/accessPackages/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        response: serde_json::Value,
        path: "/accessPackages/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: [ on ],
        has_body: false
    });
    "#
    .to_string();

    s = s
        .replace("get!({", "get(")
        .replace("put!({", "put(")
        .replace("post!({", "post(")
        .replace("patch!({", "patch(")
        .replace("delete!({", "delete(")
        .replace("has_body: false", "")
        .replace("response: serde_json::Value,", "")
        .replace("response: NoContent,", "")
        .replace("[", "")
        .replace("],", "")
        .replace("]", "")
        .replace(",});", ");");
    println!("{}", s);
}

fn test_api() {
    let client = Graph::new("token");

    let request = client.admin().update_admin(&serde_json::json!({}));
    println!("{:#?}", request.url());

    let schema_names = [
        "",
        "microsoft.graph.agreementFileVersion",
        "microsoft.graph.agreementFileLocalizationCollectionResponse",
        "microsoft.graph.agreementFile",
        "microsoft.graph.agreementAcceptance",
        "microsoft.graph.agreement",
        "error",
        "top",
        "skip",
        "search",
        "filter",
        "count",
        "ODataCountResponse",
    ];
}

fn parse_method(line: &str) -> Option<reqwest::Method> {
    match line.as_bytes() {
        b"get!({" => Some(Method::GET),
        b"put!({" => Some(Method::PUT),
        b"patch!({" => Some(Method::PATCH),
        b"post!({" => Some(Method::POST),
        b"delete!({" => Some(Method::DELETE),
        _ => None,
    }
}

fn format_method(method: reqwest::Method) -> String {
    match method {
        Method::GET => "get!(".to_string(),
        Method::PUT => "put!(".to_string(),
        Method::POST => "post!(".to_string(),
        Method::PATCH => "patch!(".to_string(),
        Method::DELETE => "delete!(".to_string(),
        _ => panic!("Not a method"),
    }
}

fn parse_macro_method() {
    let mut s = r#"
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/accessPackages/$count",
        has_body: true
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        response: serde_json::Value,
        path: "/accessPackages/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: [ on id ],
        has_body: false
    });
    "#
    .to_string();

    let mut api_method = ParsedApiMethod::default();
    let mut methods: Vec<ParsedApiMethod> = Vec::new();

    for line in s.lines() {
        if line.trim().starts_with("});") {
            methods.push(api_method);
            api_method = ParsedApiMethod::default();
        }

        let l = line.trim();
        if api_method.method.is_none() {
            let m = parse_method(l);
            if m.is_some() {
                api_method.method = m;
            }
        }

        if l.starts_with("doc") {
            api_method.doc = Some(l.to_string());
        }

        if l.starts_with("name") {
            api_method.name = l.to_string();
        }

        if l.starts_with("path") {
            api_method.path = l.to_string();
        }

        if l.starts_with("has_body: true") {
            api_method.body = true;
        }

        if l.starts_with("params") {
            api_method.params = Some(
                l.replace("[", "")
                    .replace("]", "")
                    .replace(" ,", "")
                    .replace(",", "")
                    .replace("  ", " "),
            );
        }
    }

    println!("{:?}", methods);

    for m in methods.iter() {
        println!("{}", m.write_out_method());
    }
}

fn somehow() {
    let mut file = OpenOptions::new()
        .read(true)
        .open("./src/calendar/request.rs")
        .unwrap();

    let mut buffer = String::new();
    let s = file.read_to_string(&mut buffer).unwrap();

    //let captures = API_METHOD_MACRO.captures_iter(buffer.as_str());

    for cap in API_METHOD_MACRO.captures_iter(buffer.as_str()) {
        println!("{:#?}", cap);
    }

    let m = API_METHOD_MACRO.find(buffer.as_str());
    if let Some(mat) = m {
        println!("{}", mat.as_str());
    }
}

fn main() {
    // open_api.filter_replace_path("/agreements");
    //    open_api.as_file_pretty("./examples/example_files/agreements.json").unwrap();
    // microsoft.graph.agreementFileLocalizationCollectionResponse

    //  OpenApi

    // let mut open_api_raw = OpenApiRaw::default();

    // open_api_raw.path_filter("/agreements", schema_names.iter().map(|s| s.to_string()).collect());
    // open_api_raw.as_file_pretty("./examples/example_files/agreements.json").expect("error");

    /*let client = GraphV2::new("").admin();
    let handler = client
        .select(&["name", "value"])
        .top("3")
        .update_admin(&serde_json::json!({}));
    println!("{:#?}", handler.url());*/

    //let mut json = serde_json::json!({});

    //json["id"] = serde_json::Value::String("name".to_string());

    //fn get_x(&self, bar: &str, baz: impl FnOnce()) -> u32 { 0 }

    /*visit_members! {
        call_via_deref; fn get_x(&self, foo: usize) -> u32;
    }*/
}
/*
    (
        $name:ident, $template:expr, $method:expr, body
    ) => {
        pub fn $name(&mut self) -> ResponseHandler {
            //(**self).$name( $($arg_name),* )
            let url = self.build_url($template, &serde_json::json!({})).unwrap();
            let request_builder = self.client.default_builder($method, url);
            let body_result = serde_json::to_string(body).map_err(GraphFailure::from);

            if let Ok(body) = body_result {
                let request_builder = self.client.default_builder_with_body($method, url, body);
                ResponseHandler::new(request_builder, None)
            } else if let Err(err) = body_result {
                let request_builder = self.client.default_builder($method, url);
                ResponseHandler::new(request_builder, Some(err));
            }
        }
    };



macro_rules! request {

    ({ $name:ident, $template:expr, $method:expr }) => {
        pub fn $name(&mut self)-> ResponseHandler
        {
            let url = self.build_url($template, &serde_json::json!({})).unwrap();
            let request_builder = self.client.default_builder($method, url);
            ResponseHandler::new(request_builder, None)
        }
    };

    ({ $name:ident, $template:expr, $method:expr, [ $p:ident ] }) => {
        pub fn $name<ID: AsRef<str>>(&mut self, $p: ID)-> ResponseHandler
        {
            let url = self.build_url($template, &serde_json::json!({ "id": $p.as_ref() })).unwrap();
            let request_builder = self.client.default_builder($method, url);
            ResponseHandler::new(request_builder, None)
        }
    };

    ({ $name:ident, $template:expr, $method:expr, [ $p:ident, $p1:ident ] }) => {
        pub fn $name<ID: AsRef<str>>(&mut self, $p: ID, $p1: ID)-> ResponseHandler
        {
            let url = self.build_url($template, &serde_json::json!({
                "id": $p.as_ref(), "id1": $p1.as_ref()
            })).unwrap();
            let request_builder = self.client.default_builder($method, url);
            ResponseHandler::new(request_builder, None)
        }
    };

    ({ $name:ident, $template:expr, $method:expr, [ $p:ident, $p1:ident $p2:ident ] }) => {
        pub fn $name<ID: AsRef<str>>(&mut self, $p: ID, $p1: ID, $p2: ID)-> ResponseHandler
        {
            let url = self.build_url($template, &serde_json::json!({
                "id": $p.as_ref(),"id1": $p1.as_ref(), "id2": $p2.as_ref()
            })).unwrap();
            let request_builder = self.client.default_builder($method, url);
            ResponseHandler::new(request_builder, None)
        }
    };

    ({ $name:ident, $template:expr, $method:expr, [ $p:ident, $p1:ident $p2:ident $p3:ident ] }) => {
        pub fn $name<ID: AsRef<str>>(&mut self, $p: ID, $p1: ID, $p2: ID)-> ResponseHandler
        {
            let url = self.build_url($template, &serde_json::json!({
                "id": $p.as_ref(),"id1": $p1.as_ref(), "id2": $p2.as_ref(), "id3": $p3:ident
            })).unwrap();
            let request_builder = self.client.default_builder($method, url);
            ResponseHandler::new(request_builder, None)
        }
    };

    ({ $name:ident, $template:expr, $method:expr, [ body ] }) => {
        pub fn $name<ID: AsRef<str>, B: serde::Serialize>(&mut self, body: &B)-> ResponseHandler
        {
            let url = self.build_url($template, &serde_json::json!({})).unwrap();
            let request_builder = self.client.default_builder($method, url)
                .body(serde_json::to_string(body));
            ResponseHandler::new(request_builder, None)
        }
    };

    ({ $name:ident, $template:expr, $method:expr, [ $p:ident, $p1:ident $p2:ident body ] }) => {
        pub fn $name<ID: AsRef<str>, B: serde::Serialize>(&mut self, $p: ID, $p1: ID, $p2: ID, body: &B)-> ResponseHandler
        {
            let url = self.build_url($template, &serde_json::json!({
                "id": $p.as_ref(),"id1": $p1.as_ref(), "id2": $p2.as_ref(), "id3": $p3:ident
            })).unwrap();
            let request_builder = self.client.default_builder($method, url)
                .body(serde_json::to_string(body));
            ResponseHandler::new(request_builder, None)
        }
    };
    // $(, $arg_name:ident : $arg_ty:ty )*)


    ( get ( $name:ident, $path:expr ) ) => {
        request!({ $name, $path, Method::GET });
    };

    ( get ( $name:ident, $path:expr, [ $p:ident ] ) ) => {
        request!({ $name, $path, Method::GET, [ $p ] });
    };

    ( get ( $name:ident, $path:expr, [ $p:ident, $p1:ident ] ) ) => {
        request!({ $name, $path, Method::GET, [ $p $p1 ] });
    };

    ( get ( $name:ident, $path:expr, [ $p:ident $p1:ident $p2:ident ] ) ) => {
        request!({ $name, $path, Method::GET, [ $p $p1 $p2 ] });
    };

    ( get ( $name:ident, $path:expr, [ $p:ident $p1:ident $p2:ident $p3:ident ] ) ) => {
        request!({ $name, $path, Method::GET, [ $p $p1 $p2 $p3 ] });
    };

    ( get ( $name:ident, $path:expr, [ body ] ) ) => {
        request!({ $name, $path, Method::GET, [ body ] });
    };

    ( get ( $name:ident, $path:expr, [ $p:ident $p1:ident $p2:ident body ] ) ) => {
        request!({ $name, $path, Method::GET, [ $p $p1 $p2 body ] });
    };

    // [ $p:ident $p1:ident ]


   /*
    (post{ name: $name:ident, path: $template:expr }) => {
        request!({ $name, $path, Method::POST });
    };

    (put{ name: $name:ident, path: $template:expr }) => {
        request!({$name, $path, Method::PUT});
    };

    (patch{ name: $name:ident, path: $template:expr }) => {
        request!({$name, $path, Method::PATCH});
    };

    (delete{ name: $name:ident, path: $template:expr }) => {
        request!({$name, $path, Method::DELETE});
    }
    */
}

/*
        pub fn $name(&mut self)-> ResponseHandler
        {
            let url = self.build_url($template, &serde_json::json!({})).unwrap();
            let request_builder = self.client.default_builder($m, url);
            ResponseHandler::new(request_builder)
        }
 */

 */
