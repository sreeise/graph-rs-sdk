use from_as::*;
use std::{
    convert::TryFrom,
    io::{Read, Write},
};

/// [Header Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#header-object)
///
/// The Header Object follows the structure of the Parameter Object with the
/// following changes:
///
/// * name MUST NOT be specified, it is given in the corresponding headers map.
/// * in MUST NOT be specified, it is implicitly in header.
/// * All traits that are affected by the location MUST be applicable to a
///   location of header (for example, style).
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    /*
       /// REQUIRED. The name of the parameter. Parameter names are case sensitive.
       /// * If in is "path", the name field MUST correspond to a template expression occurring
       /// within the path field in the Paths Object. See Path Templating for further information.
       /// * If in is "header" and the name field is "Accept", "Content-Type" or "Authorization",
       /// the parameter definition SHALL be ignored.
       /// * For all other cases, the name corresponds to the parameter name used by the in property.
       #[serde(skip_serializing_if = "Option::is_none")]
       pub name: Option<String>,

       /// REQUIRED. The location of the parameter. Possible values are "query", "header", "path" or "cookie".
       #[serde(rename = "in")]
       #[serde(skip_serializing_if = "Option::is_none")]
       pub in_: Option<String>,
    */
    /// A brief description of the parameter. This could contain examples of
    /// use. CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Determines whether this parameter is mandatory. If the parameter
    /// location is "path", this property is REQUIRED and its value MUST be
    /// true. Otherwise, the property MAY be included and its default value
    /// is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// Specifies that a parameter is deprecated and SHOULD be transitioned out
    /// of usage. Default value is false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    /// Sets the ability to pass empty-valued parameters. This is valid only for
    /// query parameters and allows sending a parameter with an empty value.
    /// Default value is false. If style is used, and if behavior is n/a
    /// (cannot be serialized), the value of allowEmptyValue SHALL be ignored.
    /// Use of this property is NOT RECOMMENDED, as it is likely to be removed
    /// in a later revision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty_value: Option<bool>,
}
