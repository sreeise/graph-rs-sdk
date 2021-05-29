use crate::openapi::{either_t_map_right_or_reference, Encoding, Example, Reference, Schema};
use either::Either;
use from_as::*;
use std::{
    collections::HashMap,
    convert::TryFrom,
    io::{Read, Write},
};

/// [Media Type Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#media-type-object)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct MediaType {
    // The schema defining the content of the request, response, or parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,

    /// Example of the media type. The example object SHOULD be in the correct
    /// format as specified by the media type. The example field is mutually
    /// exclusive of the examples field. Furthermore, if referencing a
    /// schema which contains an example, the example value SHALL override
    /// the example provided by the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,

    /// Examples of the media type. Each example object SHOULD match the media
    /// type and specified schema if present. The examples field is mutually
    /// exclusive of the example field. Furthermore, if referencing a schema
    /// which contains an example, the examples value SHALL override the
    /// example provided by the schema.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "either_t_map_right_or_reference")]
    pub examples: Option<HashMap<String, Either<Example, Reference>>>,

    /// A map between a property name and its encoding information. The key,
    /// being the property name, MUST exist in the schema as a property. The
    /// encoding object SHALL only apply to requestBody objects when the
    /// media type is multipart or application/x-www-form-urlencoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<HashMap<String, Encoding>>,
}
