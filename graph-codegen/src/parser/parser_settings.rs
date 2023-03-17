use crate::api_types::ModifierMap;
use crate::filter::{Filter, FilterIgnore};
use crate::parser::{DirectoryModFile, RequestSet};
use crate::settings::{get_custom_requests, get_target_map_modifier};
use graph_core::resource::ResourceIdentity;
use std::collections::HashMap;

pub struct ParserSettings;

impl ParserSettings {
    pub fn default_path_filters() -> Vec<Filter> {
        vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
            "singleValueExtendedProperties".into(),
            "multiValueExtendedProperties".into(),
        ]))]
    }

    pub fn custom_register_clients(resource_identity: ResourceIdentity) -> Option<String> {
        match resource_identity {
            ResourceIdentity::Drives => Some(
                "register_client!(
                    () DrivesRequest,
                    drive_item => \"drive/items\", \"items\", ResourceIdentity::Drives,
                    drive_root => \"drive\", \"\", ResourceIdentity::Drives,
                    drive_root_path => \"drive/root\", \"root\", ResourceIdentity::Drives,
                );"
                .to_string(),
            ),
            _ => None,
        }
    }

    pub fn custom_methods(
        resource_identity: ResourceIdentity,
    ) -> Option<HashMap<String, RequestSet>> {
        get_custom_requests(resource_identity)
    }

    pub fn directory_mod(resource_identity: ResourceIdentity) -> Option<DirectoryModFile> {
        match resource_identity {
            ResourceIdentity::Drive | ResourceIdentity::Drives => Some(DirectoryModFile {
                resource_identity,
                mod_name: "manual_request".into(),
                use_all: true,
            }),
            ResourceIdentity::OnenotePages => Some(DirectoryModFile {
                resource_identity,
                mod_name: "manual_request".into(),
                use_all: true,
            }),
            ResourceIdentity::ChildFolders => Some(DirectoryModFile {
                resource_identity,
                mod_name: "manual_request".into(),
                use_all: true,
            }),
            _ => None,
        }
    }

    // Modifiers that need to be explicitly declared.
    // The struct names for clients are generated based on the operation id
    // which is also modified when the clients are generated. This can result
    // in naming conflicts that is fixed by these modifiers.
    pub fn target_modifiers(resource_identity: ResourceIdentity) -> ModifierMap {
        get_target_map_modifier(resource_identity)
    }
}
