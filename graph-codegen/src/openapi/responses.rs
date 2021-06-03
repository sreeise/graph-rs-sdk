use crate::openapi::{EitherT, Reference, Response};
use from_as::*;
use std::{
    collections::HashMap,
    convert::TryFrom,
    io::{Read, Write},
};

/// [Responses Object](hhttps://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#responsesObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Responses {
    /// The documentation of responses other than the ones declared for specific
    /// HTTP response codes. Use this field to cover undeclared responses. A
    /// Reference Object can link to a response that the OpenAPI Object's
    /// components/responses section defines.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<EitherT<Response, Reference>>,

    /// Any HTTP status code can be used as the property name, but only one
    /// property per code, to describe the expected response for that HTTP
    /// status code. A Reference Object can link to a response that is defined
    /// in the OpenAPI Object's components/responses section. This field MUST be
    /// enclosed in quotation marks (for example, "200") for compatibility
    /// between JSON and YAML. To define a range of response codes, this field
    /// MAY contain the uppercase wildcard character X. For example, 2XX
    /// represents all response codes between [200-299]. Only the following
    /// range definitions are allowed: 1XX, 2XX, 3XX, 4XX, and 5XX. If a
    /// response is defined using an explicit code, the explicit code definition
    /// takes precedence over the range definition for that code.
    #[serde(flatten)]
    #[serde(default)]
    pub status_codes: HashMap<String, EitherT<Response, Reference>>,
}
