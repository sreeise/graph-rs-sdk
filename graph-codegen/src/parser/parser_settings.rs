use crate::api_types::ModifierMap;
use crate::filter::{Filter, FilterIgnore};
use crate::parser::settings::get_doc_comment_replace_filter;
use crate::settings::{
    get_client_link_settings, get_custom_requests, get_imports, get_path_filters,
    get_target_map_modifier,
};
use crate::{
    builder::ClientLinkSettings,
    parser::{DirectoryModFile, RequestSet},
};
use graph_core::resource::ResourceIdentity;
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub struct ParserSettings;

impl ParserSettings {
    /// Imports that won't be added from parsing and need to be manually added.
    pub fn imports(resource_identity: ResourceIdentity) -> BTreeSet<String> {
        let mut vec: Vec<&'static str> = Vec::new();
        vec.extend(get_imports(resource_identity));
        vec.sort_unstable();
        let mut set: BTreeSet<String> = BTreeSet::new();
        set.extend(vec.into_iter().map(|s| s.to_string()));
        set
    }

    pub fn default_path_filters() -> Vec<Filter> {
        vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
            "singleValueExtendedProperties".into(),
            "multiValueExtendedProperties".into(),
        ]))]
    }

    // Filters for clients when the parsing and generation happens. Some clients,
    // such as Users and Groups use the same path for resources like calendars, and
    // so we generate a separate module for calendars. In cases like these, Users
    // and Groups will use the same calendar module. This cuts down on the size
    // of the crate and makes it easier to generate clients that use the same
    // resources.
    pub fn path_filters(resource_identity: ResourceIdentity) -> Vec<Filter> {
        get_path_filters(resource_identity)
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
            ResourceIdentity::Pages => Some(DirectoryModFile {
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

    pub fn client_link_settings(
        resource_identity: ResourceIdentity,
    ) -> BTreeMap<String, BTreeSet<ClientLinkSettings>> {
        get_client_link_settings(resource_identity)
    }

    // Modifiers that need to be explicitly declared.
    // The struct names for clients are generated based on the operation id
    // which is also modified when the clients are generated. This can result
    // in naming conflicts that is fixed by these modifiers.
    pub fn target_modifiers(resource_identity: ResourceIdentity) -> ModifierMap {
        get_target_map_modifier(resource_identity)
    }

    pub fn doc_comment_filters(resource_identity: ResourceIdentity) -> Vec<String> {
        get_doc_comment_replace_filter(resource_identity)
    }
}
