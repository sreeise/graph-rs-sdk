use crate::openapi::{Discriminator, ExternalDocumentation, XML};
use from_as::*;
use std::convert::TryFrom;
use std::io::{Read, Write};

/// [Schema Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#schemaObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Schema {
    /// Adds support for polymorphism. The discriminator is an object name that is used
    /// to differentiate between other schemas which may satisfy the payload description.
    discriminator: Discriminator,

    /// This MAY be used only on properties schemas. It has no effect on root schemas.
    /// Adds additional metadata to describe the XML representation of this property.
    xml: XML,

    /// Additional external documentation for this schema.
    #[serde(rename = "externalDocs")]
    external_docs: ExternalDocumentation,

    /// Deprecated: The example property has been deprecated in favor of the JSON
    /// Schema examples keyword. Use of example is discouraged, and later versions
    /// of this specification may remove it.
    #[serde(skip_serializing_if = "Option::is_none")]
    example: Option<serde_json::Value>,
}
