use from_as::*;
use std::convert::TryFrom;
use std::io::{Read, Write};

/// [External Documentation Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#externalDocumentationObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct ExternalDocumentation {
    /// A description of the target documentation. CommonMark syntax MAY be used for rich text representation.
    pub description: String,

    /// REQUIRED. The URL for the target documentation. This MUST be in the form of a URL.
    pub url: String,
}
