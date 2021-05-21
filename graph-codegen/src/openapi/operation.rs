use crate::openapi::{ExternalDocumentation, Parameter, Reference, RequestBody};
use from_as::*;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::io::{Read, Write};

/// [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#operation-object)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Operation {
    /// A list of tags for API documentation control. Tags can be used for logical grouping
    /// of operations by resources or any other qualifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<VecDeque<String>>,

    /// A short summary of what the operation does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// A verbose explanation of the operation behavior. CommonMark syntax MAY be used for
    /// rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Additional external documentation for this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,

    /// Unique string used to identify the operation. The id MUST be unique among all
    /// operations described in the API. The operationId value is case-sensitive.
    /// Tools and libraries MAY use the operationId to uniquely identify an operation,
    /// therefore, it is RECOMMENDED to follow common programming naming conventions.
    #[serde(rename = "operationId")]
    pub operation_id: String,

    /// A list of parameters that are applicable for this operation. If a parameter is
    /// already defined at the Path Item, the new definition will override it but can
    /// never remove it. The list MUST NOT include duplicated parameters. A unique parameter
    /// is defined by a combination of a name and location. The list can use the Reference
    /// Object to link to parameters that are defined at the OpenAPI Object's components/parameters.
    #[serde(default)]
    #[serde(skip_serializing_if = "VecDeque::is_empty")]
    pub parameters: VecDeque<Parameter>,

    /// The request body applicable for this operation. The requestBody is fully supported in
    /// HTTP methods where the HTTP 1.1 specification RFC7231 has explicitly defined semantics
    /// for request bodies. In other cases where the HTTP spec is vague (such as GET, HEAD and
    /// DELETE), requestBody is permitted but does not have well-defined semantics and SHOULD
    /// be avoided if possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestBody")]
    pub request_body: Option<RequestBody>,

    /// The list of possible responses as they are returned from executing this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<serde_json::Value>,
}
