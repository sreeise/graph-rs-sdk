use from_as::*;
use graph_core::resource::ResourceIdentity;
use std::convert::TryFrom;
use std::io::{Read, Write};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromFile, AsFile)]
pub struct ResourceParsingInfo {
    /// The name, in camel case, of the Api that is used in a request path. These are usually
    /// the same as the ResourceIdentity name and therefore this is an optional field.
    pub modifier_name: Option<String>,

    /// The starting point of the path relating to a single API endpoint. As an example,
    /// if you want to generate the user APIs then pass /user. For secondary level resources
    /// pass the whole path up to and including the secondary level resource name. For example,
    /// if you want to generate the calendar APIs then pass /me/calendar
    pub path: String,

    /// Represents the Api being parsed. If there is not a ResourceIdentity for the
    /// Api then you can add the Api to the enum.
    pub resource_identity: ResourceIdentity,

    /// If this is a secondary level resource in the path then pass the first part of
    /// the path that need to be trimmed. For instance, given /me/activities and
    /// you want to generate the activities api, then pass /me here so that it will be
    /// trimmed from the path.
    pub trim_path_start: Option<String>,

    /// If the resource is secondary and the top level resource has an id then
    /// the original parameters will still have the top level resource id which
    /// we don't want to include. Allow filtering for these parameters.
    pub parameter_filter: Vec<String>,
}
