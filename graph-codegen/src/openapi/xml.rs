use from_as::*;
use std::convert::TryFrom;
use std::io::{Read, Write};

/// [XML Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#xmlObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct XML {
    /// Replaces the name of the element/attribute used for the described schema property.
    /// When defined within items, it will affect the name of the individual XML elements
    /// within the list. When defined alongside type being array (outside the items), it
    /// will affect the wrapping element and only if wrapped is true. If wrapped is false,
    /// it will be ignored.
    name: String,

    /// The URI of the namespace definition. This MUST be in the form of an absolute URI.
    namespace: String,

    /// The prefix to be used for the name.
    prefix: String,

    /// Declares whether the property definition translates to an attribute instead of an element.
    /// Default value is false.
    #[serde(default)]
    attribute: bool,

    /// MAY be used only for an array definition. Signifies whether the array is
    /// wrapped (for example, <books><book/><book/></books>) or unwrapped (<book/><book/>).
    /// Default value is false. The definition takes effect only when defined alongside type
    /// being array (outside the items).
    #[serde(default)]
    wrapped: bool,
}
