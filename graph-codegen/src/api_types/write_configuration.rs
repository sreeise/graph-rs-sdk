use crate::api_types::{ModFile, ModFileWriter};
use from_as::*;
use graph_core::resource::ResourceIdentity;
use inflector::Inflector;
use std::convert::TryFrom;
use std::io::{Read, Write};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromFile, AsFile)]
pub struct ModWriteConfiguration {
    pub folder_path: String,
    pub folder_name: String,
    pub mod_name: String,
}

#[derive(
    Builder, Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, FromFile, AsFile,
)]
#[builder(
    pattern = "mutable",
    derive(Debug, Eq, PartialEq, Serialize, Deserialize),
    setter(into, strip_option),
    default
)]
pub struct WriteConfiguration {
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

    pub filter_path: Vec<String>,

    /// Replace the name in the metadata. The operation name is typically how
    /// a given resource is identified and tells us what api client struct
    /// to include the request for. However, this does not match the path all of the
    /// time such as with the /me path so provide an override to just make the
    /// operation map the same value such as "me" for all paths that start with /me
    pub replace_operation_map: Option<String>,

    /// If the resource is secondary and the top level resource has an id then
    /// the original parameters will still have the top level resource id which
    /// we don't want to include. Allow filtering for these parameters.
    pub parameter_filter: Vec<String>,

    pub mod_file: Option<ModFile>,

    pub mod_file_writer: Option<ModFileWriter>,

    pub mod_write_override: Option<ModWriteConfiguration>,

    pub children: Vec<WriteConfiguration>,
}

impl WriteConfiguration {
    pub fn builder(resource_identity: ResourceIdentity) -> WriteConfigurationBuilder {
        let mut builder = WriteConfigurationBuilder::default();
        builder
            .path(resource_identity.to_path_start())
            .resource_identity(resource_identity)
            .mod_file(ModFile::base_mod_from_ri(resource_identity))
            .replace_operation_map(resource_identity.exact_camel_case());
        builder
    }

    pub fn second_level_builder(
        base_resource_identity: ResourceIdentity,
        resource_identity: ResourceIdentity,
    ) -> WriteConfigurationBuilder {
        let mut builder = WriteConfiguration::builder(resource_identity);
        builder
            .mod_file(ModFile::base_mod(&format!(
                "{}/{}",
                base_resource_identity.exact_snake_case(),
                resource_identity.exact_snake_case()
            )))
            .replace_operation_map(resource_identity.exact_camel_case())
            .trim_path_start(base_resource_identity.to_path_start());

        builder
    }

    pub fn implement_children_mods(&mut self) {
        if !self.children.is_empty() {
            if let Some(mfw) = self.mod_file_writer.as_mut() {
                let mods: Vec<String> = self
                    .children
                    .iter()
                    .map(|wc| wc.mod_write_override.clone())
                    .flatten()
                    .map(|mwc| mwc.mod_name.to_string())
                    .collect();
                mfw.extend(mods);
            }

            if let Some(mod_file) = self.mod_file.as_mut() {
                let mods: Vec<String> = self
                    .children
                    .iter()
                    .map(|wc| wc.resource_identity.exact_snake_case())
                    .collect();
                match mod_file {
                    ModFile::Declarations { file, declarations } => {
                        declarations.extend(mods);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl From<ResourceIdentity> for WriteConfiguration {
    fn from(resource_identity: ResourceIdentity) -> Self {
        WriteConfigurationBuilder::default()
            .path(resource_identity.to_path_start())
            .resource_identity(resource_identity)
            .mod_file(ModFile::base_mod_from_ri(resource_identity))
            .replace_operation_map(resource_identity.exact_camel_case())
            .build()
            .unwrap()
    }
}
