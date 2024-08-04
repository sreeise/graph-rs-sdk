use crate::api_types::{PathMetadata, RequestMetadata};
use crate::openapi::{EitherT, Operation, Parameter, Reference, Server};
use crate::parser::HttpMethod;
use crate::traits::{PathMatcher, RequestParser};
use from_as::*;
use inflector::Inflector;
use std::collections::{HashMap, HashSet};
use std::{
    collections::VecDeque,
    convert::TryFrom,
    io::{Read, Write},
};

/// [Path Item Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#pathItemObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct PathItem {
    /// Allows for a referenced definition of this path item. The referenced
    /// structure MUST be in the form of a Path Item Object. In case a Path
    /// Item Object field appears both in the defined object and the
    /// referenced object, the behavior is undefined.
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,

    /// An optional, string summary, intended to apply to all operations in this
    /// path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// An optional, string description, intended to apply to all operations in
    /// this path. CommonMark syntax MAY be used for rich text
    /// representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A definition of a GET operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,

    /// A definition of a PUT operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,

    /// A definition of a POST operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,

    /// A definition of a DELETE operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,

    /// A definition of a OPTIONS operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,

    /// A definition of a HEAD operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,

    /// A definition of a PATCH operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,

    /// A definition of a TRACE operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Operation>,

    /// An alternative server array to service all operations in this path.
    #[serde(default)]
    #[serde(skip_serializing_if = "VecDeque::is_empty")]
    pub servers: VecDeque<Server>,

    /// A list of parameters that are applicable for all the operations
    /// described under this path. These parameters can be overridden at the
    /// operation level, but cannot be removed there. The list MUST NOT
    /// include duplicated parameters. A unique parameter is defined by a
    /// combination of a name and location. The list can use the Reference
    /// Object to link to parameters that are defined at the OpenAPI
    /// Object's components/parameters.
    #[serde(default)]
    #[serde(skip_serializing_if = "VecDeque::is_empty")]
    pub parameters: VecDeque<EitherT<Parameter, Reference>>,
}

impl PathItem {
    pub fn parameters(&self) -> VecDeque<Parameter> {
        self.parameters
            .iter()
            .filter(|either| either.is_left())
            .flat_map(|either| either.either_as_ref().left())
            .cloned()
            .collect()
    }

    pub fn path_parameter_size(&self) -> usize {
        self.parameters()
            .iter()
            .filter(|parameter| parameter.is_path())
            .count()
    }

    pub fn path_parameters(&self) -> VecDeque<String> {
        self.parameters()
            .iter()
            .filter(|p| p.is_path())
            .flat_map(|p| p.name.clone())
            .collect()
    }

    pub fn path_parameters_and_size(&self) -> (usize, VecDeque<String>) {
        let parameters = self.parameters();
        let path_param_size = parameters
            .iter()
            .filter(|parameter| parameter.is_path())
            .count();
        let parameters = parameters
            .iter()
            .filter(|p| p.is_path())
            .flat_map(|p| p.name.clone())
            .collect();
        (path_param_size, parameters)
    }

    pub fn query_parameters_and_size(&self) -> (usize, VecDeque<String>) {
        let parameters = self.parameters();
        let path_param_size = parameters
            .iter()
            .filter(|parameter| parameter.is_query())
            .count();
        let parameters = parameters
            .iter()
            .filter(|p| p.is_query())
            .flat_map(|p| p.name.clone())
            .collect();
        (path_param_size, parameters)
    }

    pub fn operations(&self) -> VecDeque<Operation> {
        [
            self.get.as_ref(),
            self.put.as_ref(),
            self.post.as_ref(),
            self.patch.as_ref(),
            self.delete.as_ref(),
        ]
        .iter()
        .cloned()
        .flatten()
        .cloned()
        .collect()
    }

    pub fn operation_http_map(&self) -> HashMap<HttpMethod, Operation> {
        let mut map: HashMap<HttpMethod, Operation> = HashMap::new();
        if let Some(get) = self.get.as_ref() {
            map.insert(HttpMethod::GET, get.clone());
        }

        if let Some(put) = self.put.as_ref() {
            map.insert(HttpMethod::PUT, put.clone());
        }

        if let Some(post) = self.post.as_ref() {
            map.insert(HttpMethod::POST, post.clone());
        }

        if let Some(patch) = self.patch.as_ref() {
            map.insert(HttpMethod::PATCH, patch.clone());
        }

        if let Some(delete) = self.delete.as_ref() {
            map.insert(HttpMethod::DELETE, delete.clone());
        }

        map
    }

    fn map_to_request_metadata(&self) -> VecDeque<RequestMetadata> {
        self.operation_http_map()
            .iter()
            .map(|(http_method, operation)| operation.request_metadata(*http_method))
            .collect()
    }

    /// There are some requests with paths that have a key value type parameters we need to get
    /// but the parameters and operations object treat these as queries in the metadata.
    ///
    /// In these cases, get the names for the parameters by parsing the path using a Regex,
    /// and match them against the queries in the parameters or operations objects.
    fn map_path_parameters_labeled_in_query(&self, request_path_item: &mut PathMetadata) {
        let operations = self.operations();
        let path = request_path_item.path.clone();
        if PathMatcher::KeyValuePairQuoted.matches(path.as_str()) {
            let (_path, names) = path.transform_key_value_pair_query();
            let names: HashSet<String> = names.iter().map(|n| n.to_snake_case()).collect();

            if let Some(operation) = operations.front() {
                let operation_query_size = operation.query_parameter_size();

                let query_names: VecDeque<String> = {
                    if operation_query_size == 0 {
                        let (_query_item_parameter_size, query_item_parameters) =
                            self.query_parameters_and_size();
                        query_item_parameters
                            .iter()
                            .map(|query| query.to_snake_case())
                            .collect()
                    } else {
                        operation
                            .query_parameters()
                            .iter()
                            .map(|query| query.to_snake_case())
                            .collect()
                    }
                };

                for n in query_names.iter() {
                    let n = n.to_snake_case();
                    if names.contains(&n) {
                        request_path_item.param_size += 1;
                        request_path_item.parameters.push_back(n);
                    }
                }
            }
        }
    }

    pub fn request_metadata_secondary(
        &self,
        path: &str,
        trim_pat: &str,
        parameter_filter: &[String],
    ) -> PathMetadata {
        let path = path.trim_start_matches(trim_pat);

        let operations = self.operations();
        let mut request_path_item = PathMetadata {
            path: path.to_string(),
            ..Default::default()
        };

        // The parameter names used in the request macros are parsed using the parameter field of
        // an operation object. However, the operation object parameters may not include the path
        // parameters due to how the parameters field is an EitherT and can only be either a
        // Parameter object or a Reference object but not both. When path parameters are missing in
        // the operations object despite params showing in the path it usually means that the
        // Operations object's Parameter field has an EitherT with a Reference object in it instead
        // of a Parameter object.
        //
        // In this case the PathItem object also has a parameters field that represents all of the
        // current operations and it may have the correct path parameters.
        let (path_item_parameter_size, path_item_parameters) = self.path_parameters_and_size();

        if let Some(operation) = operations.front() {
            let operation_parameter_size = operation.path_parameter_size();

            if operation_parameter_size == 0 {
                request_path_item.parameters = path_item_parameters;
                request_path_item.param_size = path_item_parameter_size;
            } else {
                request_path_item.parameters = operation.path_parameters();
                request_path_item.param_size = operation_parameter_size;
            }
        }

        for param in parameter_filter {
            request_path_item.parameters.retain(|p| p.ne(param));
        }

        request_path_item.param_size = request_path_item.parameters.len();

        self.map_path_parameters_labeled_in_query(&mut request_path_item);

        if request_path_item.param_size > 0 {
            request_path_item.format_path_parameters();
        }

        request_path_item.metadata = self.map_to_request_metadata();
        request_path_item
    }

    pub fn request_metadata(&self, path: &str) -> PathMetadata {
        let operations = self.operations();
        let mut request_path_item = PathMetadata {
            path: path.to_string(),
            ..Default::default()
        };

        // The parameter names used in the request macros are parsed using the parameter field of
        // an operation object. However, the operation object parameters may not include the path
        // parameters due to how the parameters field is an EitherT and can only be either a
        // Parameter object or a Reference object but not both. When path parameters are missing in
        // the operations object despite params showing in the path it usually means that the
        // Operations object's Parameter field has an EitherT with a Reference object in it instead
        // of a Parameter object.
        //
        // In this case the PathItem object also has a parameters field that represents all of the
        // current operations and it may have the correct path parameters.
        let (path_item_parameter_size, path_item_parameters) = self.path_parameters_and_size();

        if let Some(operation) = operations.front() {
            let operation_parameter_size = operation.path_parameter_size();

            if operation_parameter_size == 0 {
                request_path_item.parameters = path_item_parameters;
                request_path_item.param_size = path_item_parameter_size;
            } else {
                request_path_item.parameters = operation.path_parameters();
                request_path_item.param_size = operation_parameter_size;
            }
        }

        self.map_path_parameters_labeled_in_query(&mut request_path_item);

        if request_path_item.param_size > 0 {
            request_path_item.format_path_parameters();
        }

        request_path_item.metadata = self.map_to_request_metadata();
        request_path_item
    }
}
