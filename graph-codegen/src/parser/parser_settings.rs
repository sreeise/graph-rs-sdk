use crate::builder::ClientLinkSettings;
use crate::parser::filter::{
    Filter, FilterIgnore, MatchTarget, ModifierMap, SecondaryModifierMap, UrlMatchTarget,
};
use crate::parser::settings::{
    get_client_link_settings, get_custom_requests, get_imports, get_path_filters,
    get_target_map_modifier,
};
use crate::parser::{DirectoryModFile, RequestSet};
use graph_core::resource::ResourceIdentity;
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub struct ParserSettings;

impl ParserSettings {
    /// Imports that won't be added from parsing and need to be manually added.
    pub fn imports(resource_identity: ResourceIdentity) -> Vec<&'static str> {
        get_imports(resource_identity)
    }

    pub fn default_path_filters() -> Vec<Filter<'static>> {
        vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
            "singleValueExtendedProperties",
            "multiValueExtendedProperties",
            // These are basically like OData queries and look like getByPath(path={path})
            // but we dont currently handle these so they are ignored. The get activities
            // by interval is used the most in these situations.
            "={",
            "getActivitiesByInterval",
        ]))]
    }

    // Filters for clients when the parsing and generation happens. Some clients,
    // such as Users and Groups use the same path for resources like calendars, and
    // so we generate a separate module for calendars. In cases like these, Users and
    // Groups will use the same calendar module. This cuts down on the size of the crate
    // and makes it easier to generate clients that use the same resources.
    pub fn path_filters(resource_identity: ResourceIdentity) -> Vec<Filter<'static>> {
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

    // Secondary links and modifiers. These are api's that are used multiple times
    // such as calendars and calendar groups where we might have two resources
    // such as groups and users with the same ending path:
    // groups/{group-id}/calendars/{calendar-id}
    // users/{user-id}/calendars/{calendar-id}
    // We do not want each api implementation to have its own calendar struct
    // and methods to prevent repeated code. So we separate these out here
    // and add a link between them. We change the operation map of the
    // requests so they are generated within the correct client.
    pub fn secondary_modifier_map(resource_identity: ResourceIdentity) -> SecondaryModifierMap {
        let mut map = SecondaryModifierMap::with_capacity(15);

        match resource_identity {
            ResourceIdentity::Activities => {
                map.insert_operation_mapping("me.activities", "activities");
            },
            ResourceIdentity::Buckets => {
                map.insert_operation_mapping("planner.buckets", "buckets");
            },
            ResourceIdentity::Calendar => {
                map.insert_operation_mapping("users.calendar", "calendar");
                map.insert_operation_mapping("users.calendars", "calendars");
            },
            ResourceIdentity::CalendarGroups => {
                map.insert(
                    "users.calendarGroups",
                    MatchTarget::OperationMap("calendarGroups".to_string()),
                );
                map.insert(
                    "users.calendarGroups",
                    MatchTarget::OperationId("calendarGroups".to_string()),
                );
            },
            ResourceIdentity::CalendarView => {
                map.insert(
                    "me.calendarView",
                    MatchTarget::OperationId("calendarViews".to_string()),
                );
                map.insert(
                    "me.calendarView",
                    MatchTarget::OperationMap("calendarViews".to_string()),
                );
            },
            ResourceIdentity::Calls => {
                map.insert(
                    "communications.calls",
                    MatchTarget::OperationMap("calls".to_string()),
                );
            },
            ResourceIdentity::CallRecords => {
                map.insert(
                    "communications.callRecords",
                    MatchTarget::OperationMap("callRecords".to_string()),
                );
            },
            ResourceIdentity::ContactFolders => {
                map.insert(
                    "me.contactFolders",
                    MatchTarget::OperationMap("contactFolders".to_string()),
                );
            },
            ResourceIdentity::Contacts => {
                map.insert(
                    "me.contacts",
                    MatchTarget::OperationMap("contacts".to_string()),
                );
            },
            ResourceIdentity::ContentTypes => {
                map.insert(
                    "sites.contentTypes",
                    MatchTarget::OperationMap("contentTypes".to_string()),
                );
            },
            ResourceIdentity::Conversations => {
                map.insert(
                    "groups.conversations",
                    MatchTarget::OperationMap("conversations".to_string()),
                );
            },
            ResourceIdentity::Events => {
                map.insert(
                    "users.events",
                    MatchTarget::OperationId("events".to_string()),
                );
            },
            ResourceIdentity::InferenceClassification => {
                map.insert(
                    "me.inferenceClassification",
                    MatchTarget::OperationMap("inferenceClassification".to_string()),
                );
            },
            ResourceIdentity::Instances => {
                map.insert(
                    "me.calendarView.instances",
                    MatchTarget::OperationMap("instances".to_string()),
                );
            },
            ResourceIdentity::Items => {
                map.insert(
                    "sites.lists.items",
                    MatchTarget::OperationMap("items".to_string()),
                );
            },
            ResourceIdentity::Lists => {
                map.insert(
                    "sites.lists",
                    MatchTarget::OperationMap("lists".to_string()),
                );
            },
            ResourceIdentity::MailFolders => {
                map.insert(
                    "me.mailFolders",
                    MatchTarget::OperationMap("mailFolders".to_string()),
                );
            },
            ResourceIdentity::Messages => {
                map.insert(
                    "me.messages",
                    MatchTarget::OperationMap("messages".to_string()),
                );
            },
            ResourceIdentity::Me => {
                map.insert("me.user", MatchTarget::OperationMap("me".to_string()));
            },
            ResourceIdentity::Outlook => {
                map.insert(
                    "me.outlook",
                    MatchTarget::OperationMap("outlook".to_string()),
                );
            },
            ResourceIdentity::Plans => {
                map.insert(
                    "planner.plans",
                    MatchTarget::OperationMap("plans".to_string()),
                );
            },
            ResourceIdentity::Settings => {
                map.insert(
                    "me.settings",
                    MatchTarget::OperationMap("settings".to_string()),
                );
            },
            ResourceIdentity::Notebooks => {
                map.insert(
                    "me.onenote.notebooks",
                    MatchTarget::OperationMap("notebooks".to_string()),
                );
            },
            ResourceIdentity::SectionGroups => {
                map.insert(
                    "me.onenote.sectionGroups",
                    MatchTarget::OperationMap("sectionGroups".to_string()),
                );
            },
            ResourceIdentity::Sections => {
                map.insert(
                    "me.onenote.sections",
                    MatchTarget::OperationMap("sections".to_string()),
                );
            },
            ResourceIdentity::Sessions => {
                map.insert(
                    "communications.callRecords.sessions",
                    MatchTarget::OperationMap("sessions".to_string()),
                );
            },
            ResourceIdentity::Pages => {
                map.insert(
                    "me.onenote.pages",
                    MatchTarget::OperationMap("pages".to_string()),
                );
            },
            ResourceIdentity::ParentSection => {
                map.insert(
                    "me.onenote.pages.parentSection",
                    MatchTarget::OperationMap("parentSection".to_string()),
                );
            },
            ResourceIdentity::Posts => {
                map.insert(
                    "groups.threads.posts",
                    MatchTarget::OperationMap("posts".to_string()),
                );
            },
            ResourceIdentity::Tasks => {
                map.insert(
                    "planner.tasks",
                    MatchTarget::OperationMap("tasks".to_string()),
                );
            },
            ResourceIdentity::Threads => {
                map.insert(
                    "groups.threads",
                    MatchTarget::OperationMap("threads".to_string()),
                );
            },
            _ => {},
        }

        map
    }

    pub fn get_directory_mod_files(
        resource_identity: ResourceIdentity,
    ) -> Option<Vec<DirectoryModFile>> {
        match resource_identity {
            ResourceIdentity::Drive | ResourceIdentity::Drives => Some(vec![DirectoryModFile {
                resource_identity,
                mod_name: "manual_request".to_string(),
                use_all: true,
            }]),
            ResourceIdentity::Pages => Some(vec![DirectoryModFile {
                resource_identity,
                mod_name: "manual_request".to_string(),
                use_all: true,
            }]),
            _ => None,
        }
    }

    // Modify that paths that have a resource id. See UrlMatchTarget
    // for more info.
    pub fn url_target_modifiers(resource_identity: ResourceIdentity) -> Vec<UrlMatchTarget> {
        match resource_identity {
            ResourceIdentity::Applications => {
                vec![UrlMatchTarget::resource_id("applications", "application")]
            },
            ResourceIdentity::Buckets => vec![UrlMatchTarget::resource_id("buckets", "bucket")],
            ResourceIdentity::Calendar => {
                vec![UrlMatchTarget::resource_id("calendars", "calendar")]
            },
            ResourceIdentity::CalendarGroups => vec![UrlMatchTarget::resource_id(
                "calendarGroups",
                "calendarGroup",
            )],
            ResourceIdentity::CalendarView => {
                vec![UrlMatchTarget::resource_id("calendarView", "calendarViews")]
            },
            ResourceIdentity::Calls => vec![UrlMatchTarget::resource_id("calls", "call")],
            ResourceIdentity::CallRecords => {
                vec![UrlMatchTarget::resource_id("callRecords", "callRecord")]
            },
            ResourceIdentity::ContactFolders => vec![UrlMatchTarget::resource_id(
                "contactFolders",
                "contactFolder",
            )],
            ResourceIdentity::Contacts => vec![UrlMatchTarget::resource_id("contacts", "contact")],
            ResourceIdentity::ContentTypes => {
                vec![UrlMatchTarget::resource_id("contentTypes", "contentType")]
            },
            ResourceIdentity::Conversations => {
                vec![UrlMatchTarget::resource_id("conversations", "conversation")]
            },
            ResourceIdentity::Drives => vec![UrlMatchTarget::resource_id("drives", "drive")],
            ResourceIdentity::Domains => vec![UrlMatchTarget::resource_id("domains", "domain")],
            ResourceIdentity::Events => vec![UrlMatchTarget::resource_id("events", "event")],
            ResourceIdentity::Groups => vec![UrlMatchTarget::resource_id("groups", "group")],
            ResourceIdentity::Instances => {
                vec![UrlMatchTarget::resource_id("instances", "instance")]
            },
            ResourceIdentity::Items => vec![UrlMatchTarget::resource_id("items", "item")],
            ResourceIdentity::Lists => vec![UrlMatchTarget::resource_id("lists", "list")],
            ResourceIdentity::MailFolders => {
                vec![UrlMatchTarget::resource_id("mailFolders", "mailFolder")]
            },
            ResourceIdentity::Messages => vec![UrlMatchTarget::resource_id("messages", "message")],
            ResourceIdentity::ManagedDevices => vec![UrlMatchTarget::resource_id(
                "managedDevices",
                "managedDevice",
            )],
            ResourceIdentity::Notebooks => {
                vec![UrlMatchTarget::resource_id("notebooks", "notebook")]
            },
            ResourceIdentity::Onenote => vec![UrlMatchTarget::resource_id("notebooks", "notebook")],
            ResourceIdentity::Pages => vec![UrlMatchTarget::resource_id("pages", "page")],
            ResourceIdentity::Posts => vec![UrlMatchTarget::resource_id("posts", "post")],
            ResourceIdentity::Sections => vec![UrlMatchTarget::resource_id("sections", "section")],
            ResourceIdentity::Plans => vec![UrlMatchTarget::resource_id("plans", "plan")],
            ResourceIdentity::SectionGroups => {
                vec![UrlMatchTarget::resource_id("sectionGroups", "sectionGroup")]
            },
            ResourceIdentity::Sessions => vec![UrlMatchTarget::resource_id("sessions", "session")],
            ResourceIdentity::Sites => vec![UrlMatchTarget::resource_id("sites", "site")],
            ResourceIdentity::Teams => vec![UrlMatchTarget::resource_id("teams", "team")],
            ResourceIdentity::Threads => vec![UrlMatchTarget::resource_id("threads", "thread")],
            ResourceIdentity::Users => vec![UrlMatchTarget::resource_id("users", "user")],
            ResourceIdentity::Tasks => vec![UrlMatchTarget::resource_id("tasks", "task")],
            ResourceIdentity::Workbooks => {
                vec![UrlMatchTarget::resource_id("workbooks", "workbook")]
            },
            _ => vec![],
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
}
