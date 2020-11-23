use crate::builder::ClientLinkSettings;
use crate::parser::filter::{
    Filter, FilterIgnore, MatchTarget, ModifierMap, SecondaryModifierMap, UrlMatchTarget,
};
use graph_core::resource::ResourceIdentity;
use std::collections::{BTreeMap, BTreeSet};

pub struct ParserSettings;

impl ParserSettings {
    /// Imports that won't be added from parsing and need to be manually added.
    pub fn imports(resource_identity: ResourceIdentity) -> Vec<&'static str> {
        match resource_identity {
            ResourceIdentity::Calendar | ResourceIdentity::Calendars => vec![
                "crate::calendar_view::{CalendarViewRequest, CalendarViewsRequest}",
                "crate::events::{EventsRequest, EventRequest}",
                "crate::core::ResourceIdentity",
                // TODO: Handlebars should be imported by the builder. Figure out why this is not happening.
                "handlebars::*",
            ],
            ResourceIdentity::CalendarGroup | ResourceIdentity::CalendarGroups => vec![
                "crate::calendar::{CalendarRequest, CalendarsRequest}",
                "crate::events::{EventsRequest, EventRequest}",
                "crate::core::ResourceIdentity",
            ],
            ResourceIdentity::CalendarView => vec![
                "crate::instances::{InstanceRequest, InstancesRequest}",
                "crate::calendar::CalendarRequest",
                "crate::core::ResourceIdentity",
            ],
            ResourceIdentity::ContactFolders => vec!["crate::core::ResourceIdentity"],
            ResourceIdentity::Lists => vec![
                "crate::content_types::{ContentTypeRequest, ContentTypesRequest}",
                "crate::items::{ItemRequest, ItemsRequest}",
            ],
            ResourceIdentity::Events => vec![
                "crate::calendar::CalendarRequest",
                "crate::instances::{InstanceRequest, InstancesRequest}",
                "crate::core::ResourceIdentity",
            ],
            ResourceIdentity::Sites => vec![
                "crate::core::ResourceIdentity",
                "crate::content_types::{ContentTypeRequest, ContentTypesRequest}",
                "crate::lists::{ListRequest, ListsRequest}",
            ],
            ResourceIdentity::ManagedDevices => vec!["crate::core::ResourceIdentity"],
            ResourceIdentity::Me => vec![
                "crate::calendar_groups::{CalendarGroupRequest, CalendarGroupsRequest}",
                "crate::calendar_view::{CalendarViewRequest, CalendarViewsRequest}",
                "crate::calendar::{CalendarRequest, CalendarsRequest}",
                "crate::education::{MeRequest as EducationMeRequest}",
                "crate::events::{EventsRequest, EventRequest}",
                "crate::managed_devices::{ManagedDeviceRequest, ManagedDevicesRequest}",
                "crate::contact_folders::{ContactFolderRequest, ContactFoldersRequest}",
                "crate::insights::InsightsRequest",
                "crate::inference_classification::InferenceClassificationRequest",
                "crate::activities::ActivitiesRequest",
                "crate::settings::SettingsRequest",
                "crate::outlook::OutlookRequest",
                "crate::core::ResourceIdentity",
            ],
            ResourceIdentity::Users => vec![
                "crate::calendar_groups::{CalendarGroupRequest, CalendarGroupsRequest}",
                "crate::calendar_view::{CalendarViewRequest, CalendarViewsRequest}",
                "crate::calendar::{CalendarRequest, CalendarsRequest}",
                "crate::education::{UsersRequest as EducationUsersRequest}",
                "crate::events::{EventsRequest, EventRequest}",
                "crate::managed_devices::{ManagedDeviceRequest, ManagedDevicesRequest}",
                "crate::contact_folders::{ContactFolderRequest, ContactFoldersRequest}",
                "crate::insights::InsightsRequest",
                "crate::inference_classification::InferenceClassificationRequest",
                "crate::activities::ActivitiesRequest",
                "crate::settings::SettingsRequest",
                "crate::outlook::OutlookRequest",
                "crate::core::ResourceIdentity",
            ],
            _ => vec![],
        }
    }

    pub fn default_path_filters() -> Vec<Filter<'static>> {
        vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
            "singleValueExtendedProperties",
            "multiValueExtendedProperties",
            "planner",
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
        match resource_identity {
            ResourceIdentity::Calendar | ResourceIdentity::Calendars => {
                vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                    "calendarGroup",
                    "instances",
                    "calendarView",
                    "events",
                    "attachments",
                ]))]
            },
            ResourceIdentity::CalendarGroups => {
                vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                    "/calendar/",
                    "events",
                    "attachments",
                    "instances",
                    "calendarView",
                    "calendarPermissions",
                    "getSchedule",
                ]))]
            },
            ResourceIdentity::CalendarView => {
                vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                    "/calendar/calendarView",
                    "events",
                    "/calendar/calendarPermissions",
                    "/calendar/getSchedule",
                    "instances",
                ]))]
            },
            ResourceIdentity::Events => {
                vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                    "/calendar/calendarView",
                    "instances",
                    "calendar/events",
                    "/calendar/getSchedule",
                    "calendarPermissions",
                ]))]
            },
            ResourceIdentity::Lists => {
                vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                    "contentTypes",
                    "items",
                ]))]
            },
            ResourceIdentity::Me => vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "activities",
                "historyItems",
                "contacts",
                "onlineMeetings",
                "outlook",
                "/settings/",
                "calendarGroup",
                "calendars",
                "calendar",
                "calendarView",
                "contactFolder",
                "events",
                "inferenceClassification",
                "insights",
                "instances",
                "mailFolders",
                "managedDevices",
                "messages",
                "onenote",
            ]))],
            ResourceIdentity::Sites => {
                vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                    "onenote",
                    "contentTypes",
                    "lists",
                ]))]
            },
            ResourceIdentity::Users => {
                vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                    "activities",
                    "historyItems",
                    "contacts",
                    "onlineMeetings",
                    "outlook",
                    "/settings/",
                    "calendarGroup",
                    "calendars",
                    "calendar",
                    "calendarView",
                    "contactFolder",
                    "events",
                    "inferenceClassification",
                    "insights",
                    "instances",
                    "mailFolders",
                    "managedDevices",
                    "messages",
                    "onenote",
                ]))]
            },
            _ => ParserSettings::default_path_filters(),
        }
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
                map.insert(
                    "me.activities",
                    MatchTarget::OperationMap("activities".to_string()),
                );
            },
            ResourceIdentity::Calendar => {
                map.insert(
                    "users.calendar",
                    MatchTarget::OperationMap("calendar".to_string()),
                );
                map.insert(
                    "users.calendar",
                    MatchTarget::OperationId("calendar".to_string()),
                );
                map.insert(
                    "users.calendars",
                    MatchTarget::OperationMap("calendars".to_string()),
                );
                map.insert(
                    "users.calendars",
                    MatchTarget::OperationId("calendars".to_string()),
                );
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
            ResourceIdentity::Me => {
                map.insert("me.user", MatchTarget::OperationMap("me".to_string()));
            },
            ResourceIdentity::Outlook => {
                map.insert(
                    "me.outlook",
                    MatchTarget::OperationMap("outlook".to_string()),
                );
            },
            ResourceIdentity::Settings => {
                map.insert(
                    "me.settings",
                    MatchTarget::OperationMap("settings".to_string()),
                );
            },
            _ => {},
        }

        map
    }

    // Modify that paths that have a resource id. See UrlMatchTarget
    // for more info.
    pub fn url_target_modifiers(resource_identity: ResourceIdentity) -> Vec<UrlMatchTarget> {
        match resource_identity {
            ResourceIdentity::Applications => {
                vec![UrlMatchTarget::resource_id("applications", "application")]
            },
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
            ResourceIdentity::ContactFolders => vec![UrlMatchTarget::resource_id(
                "contactFolders",
                "contactFolder",
            )],
            ResourceIdentity::ContentTypes => {
                vec![UrlMatchTarget::resource_id("contentTypes", "contentType")]
            },
            ResourceIdentity::Drives => vec![UrlMatchTarget::resource_id("drives", "drive")],
            ResourceIdentity::Events => vec![UrlMatchTarget::resource_id("events", "event")],
            ResourceIdentity::Groups => vec![UrlMatchTarget::resource_id("groups", "group")],
            ResourceIdentity::Instances => {
                vec![UrlMatchTarget::resource_id("instances", "instance")]
            },
            ResourceIdentity::Items => vec![UrlMatchTarget::resource_id("items", "item")],
            ResourceIdentity::Lists => vec![UrlMatchTarget::resource_id("lists", "list")],
            ResourceIdentity::ManagedDevices => vec![UrlMatchTarget::resource_id(
                "managedDevices",
                "managedDevice",
            )],
            ResourceIdentity::Sites => vec![UrlMatchTarget::resource_id("sites", "site")],
            ResourceIdentity::Teams => vec![UrlMatchTarget::resource_id("teams", "team")],
            ResourceIdentity::Users => vec![UrlMatchTarget::resource_id("users", "user")],
            ResourceIdentity::Workbooks => {
                vec![UrlMatchTarget::resource_id("workbooks", "workbook")]
            },
            _ => vec![],
        }
    }

    pub fn client_link_settings(
        resource_identity: ResourceIdentity,
    ) -> BTreeMap<String, BTreeSet<ClientLinkSettings>> {
        let mut map = BTreeMap::new();
        match resource_identity {
            ResourceIdentity::Calendar | ResourceIdentity::Calendars => {
                let mut settings = ClientLinkSettings::new("events");
                settings
                    .use_method_name("event")
                    .with_extend_path_ident()
                    .with_set_resource_identity()
                    .with_id_param();

                let mut settings2 = ClientLinkSettings::new("event");
                settings2
                    .use_method_name("events")
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings3 = ClientLinkSettings::new("calendars");
                settings3.as_id_method_link();

                let mut set = BTreeSet::new();
                set.extend(vec![settings, settings2, settings3]);
                map.insert("calendar".to_string(), set);

                let mut settings4 = ClientLinkSettings::new("calendarView");
                settings4
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings5 = ClientLinkSettings::new("calendarViews");
                settings5
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings6 = ClientLinkSettings::new("events");
                settings6
                    .use_method_name("event")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings7 = ClientLinkSettings::new("event");
                settings7
                    .use_method_name("events")
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut set2 = BTreeSet::new();
                set2.extend(vec![settings4, settings5, settings6, settings7]);
                map.insert("calendars".to_string(), set2);
            },
            ResourceIdentity::CalendarGroup | ResourceIdentity::CalendarGroups => {
                let mut settings = ClientLinkSettings::new("calendars");
                settings
                    .use_method_name("calendar")
                    .with_id_param()
                    .with_extend_path_id()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings2 = ClientLinkSettings::new("calendar");
                settings2
                    .use_method_name("calendars")
                    .with_extend_path_id()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings3 = ClientLinkSettings::new("events");
                settings3
                    .use_method_name("event")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings4 = ClientLinkSettings::new("event");
                settings4
                    .use_method_name("events")
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![settings, settings2, settings3, settings4]);
                map.insert("calendarGroups".to_string(), set);

                let mut settings5 = ClientLinkSettings::new("events");
                settings5
                    .use_method_name("event")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings6 = ClientLinkSettings::new("event");
                settings6
                    .use_method_name("events")
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings7 = ClientLinkSettings::new("calendars");
                settings7
                    .use_method_name("calendar")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings8 = ClientLinkSettings::new("calendar");
                settings8
                    .use_method_name("calendars")
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![settings5, settings6, settings7, settings8]);
                map.insert("calendarGroup".to_string(), set);
            },
            ResourceIdentity::CalendarView => {
                let mut settings = ClientLinkSettings::new("instances");
                settings
                    .use_method_name("instance")
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity()
                    .with_id_param();

                let mut settings2 = ClientLinkSettings::new("instance");
                settings2
                    .use_method_name("instances")
                    .with_extend_path_ident()
                    .with_extend_path_id();

                let mut settings3 = ClientLinkSettings::new("calendar");
                settings3
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![settings, settings2, settings3]);
                map.insert("calendarView".to_string(), set);
            },
            ResourceIdentity::Events => {
                let mut settings = ClientLinkSettings::new("calendar");
                settings
                    .with_extend_path_id()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings2 = ClientLinkSettings::new("instances");
                settings2
                    .use_method_name("instance")
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity()
                    .with_id_param();

                let mut settings3 = ClientLinkSettings::new("instance");
                settings3
                    .use_method_name("instances")
                    .with_extend_path_ident()
                    .with_extend_path_id();

                let mut set = BTreeSet::new();
                set.extend(vec![settings, settings2, settings3]);
                map.insert("events".to_string(), set);
            },
            ResourceIdentity::Lists => {
                let mut settings = ClientLinkSettings::new("items");
                settings
                    .use_method_name("item")
                    .with_id_param()
                    .with_extend_path_id()
                    .with_extend_path_ident();

                let mut settings2 = ClientLinkSettings::new("item");
                settings2
                    .use_method_name("items")
                    .with_extend_path_id()
                    .with_extend_path_ident();

                let mut settings3 = ClientLinkSettings::new("contentTypes");
                settings3
                    .use_method_name("contentType")
                    .with_id_param()
                    .with_extend_path_id()
                    .with_extend_path_ident();

                let mut settings4 = ClientLinkSettings::new("contentType");
                settings4
                    .use_method_name("contentTypes")
                    .with_extend_path_id()
                    .with_extend_path_ident();

                let mut set = BTreeSet::new();
                set.extend(vec![settings, settings2, settings3, settings4]);
                map.insert("lists".to_string(), set);
            },
            ResourceIdentity::Me => {
                let mut settings = ClientLinkSettings::new("calendarGroups");
                settings
                    .use_method_name("calendarGroup")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings2 = ClientLinkSettings::new("calendarGroup");
                settings2
                    .use_method_name("calendarGroups")
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings3 = ClientLinkSettings::new("calendars");
                settings3
                    .use_method_name("calendar")
                    .with_id_param()
                    .with_set_resource_identity()
                    .with_extend_path_ident();

                let mut settings4 = ClientLinkSettings::new("calendar");
                settings4
                    .use_method_name("calendars")
                    .with_set_resource_identity()
                    .with_extend_path_ident();

                let mut settings5 = ClientLinkSettings::new("event");
                settings5
                    .use_method_name("events")
                    .with_set_resource_identity()
                    .with_extend_path_ident();

                let mut settings6 = ClientLinkSettings::new("events");
                settings6
                    .use_method_name("event")
                    .with_id_param()
                    .with_set_resource_identity()
                    .with_extend_path_ident();

                let mut settings7 = ClientLinkSettings::new("calendarView");
                settings7
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings8 = ClientLinkSettings::new("calendarViews");
                settings8
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings9 = ClientLinkSettings::new("educationMe");
                settings9.use_method_name("education");

                let mut settings10 = ClientLinkSettings::new("insights");
                settings10
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings11 = ClientLinkSettings::new("managedDevices");
                settings11
                    .use_method_name("managed_device")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings12 = ClientLinkSettings::new("managedDevice");
                settings12
                    .use_method_name("managed_devices")
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings13 = ClientLinkSettings::new("inferenceClassification");
                settings13
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings14 = ClientLinkSettings::new("contactFolders");
                settings14
                    .use_method_name("contact_folder")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings15 = ClientLinkSettings::new("contactFolder");
                settings15
                    .use_method_name("contact_folders")
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings16 = ClientLinkSettings::new("activities");
                settings16
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings17 = ClientLinkSettings::new("settings");
                settings17
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings18 = ClientLinkSettings::new("outlook");
                settings18
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![
                    settings, settings2, settings3, settings4, settings5, settings6, settings7,
                    settings8, settings9, settings10, settings11, settings12, settings13,
                    settings14, settings15, settings16, settings17, settings18,
                ]);
                map.insert("me".to_string(), set);
            },
            ResourceIdentity::Sites => {
                let mut settings = ClientLinkSettings::new("contentTypes");
                settings
                    .use_method_name("contentType")
                    .with_id_param()
                    .with_extend_path_id()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings2 = ClientLinkSettings::new("contentType");
                settings2
                    .use_method_name("contentTypes")
                    .with_extend_path_id()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings3 = ClientLinkSettings::new("list");
                settings3
                    .use_method_name("lists")
                    .with_extend_path_id()
                    .with_extend_path_ident();

                let mut settings4 = ClientLinkSettings::new("lists");
                settings4
                    .use_method_name("list")
                    .with_id_param()
                    .with_extend_path_id()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![settings, settings2, settings3, settings4]);
                map.insert("sites".to_string(), set);
            },
            ResourceIdentity::Users => {
                let mut settings = ClientLinkSettings::new("calendarGroups");
                settings
                    .use_method_name("calendarGroup")
                    .with_id_param()
                    .with_extend_path_id()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings2 = ClientLinkSettings::new("calendarGroup");
                settings2
                    .use_method_name("calendarGroups")
                    .with_extend_path_id()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings3 = ClientLinkSettings::new("calendars");
                settings3
                    .use_method_name("calendar")
                    .with_id_param()
                    .with_set_resource_identity()
                    .with_extend_path_id()
                    .with_extend_path_ident();

                let mut settings4 = ClientLinkSettings::new("calendar");
                settings4
                    .use_method_name("calendars")
                    .with_set_resource_identity()
                    .with_extend_path_id()
                    .with_extend_path_ident();

                let mut settings5 = ClientLinkSettings::new("event");
                settings5
                    .use_method_name("events")
                    .with_set_resource_identity()
                    .with_extend_path_id()
                    .with_extend_path_ident();

                let mut settings6 = ClientLinkSettings::new("events");
                settings6
                    .use_method_name("event")
                    .with_id_param()
                    .with_set_resource_identity()
                    .with_extend_path_id()
                    .with_extend_path_ident();

                let mut settings7 = ClientLinkSettings::new("calendarView");
                settings7
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings8 = ClientLinkSettings::new("calendarViews");
                settings8
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings9 = ClientLinkSettings::new("insights");
                settings9
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings10 = ClientLinkSettings::new("managedDevices");
                settings10
                    .use_method_name("managed_device")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings11 = ClientLinkSettings::new("managedDevice");
                settings11
                    .use_method_name("managed_devices")
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings12 = ClientLinkSettings::new("inferenceClassification");
                settings12
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings13 = ClientLinkSettings::new("contactFolders");
                settings13
                    .use_method_name("contact_folder")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings14 = ClientLinkSettings::new("contactFolder");
                settings14
                    .use_method_name("contact_folders")
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings15 = ClientLinkSettings::new("activities");
                settings15
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings16 = ClientLinkSettings::new("settings");
                settings16
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings17 = ClientLinkSettings::new("outlook");
                settings17
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![
                    settings, settings2, settings3, settings4, settings5, settings6, settings7,
                    settings8, settings9, settings10, settings11, settings12, settings13,
                    settings14, settings15, settings16, settings17,
                ]);
                map.insert("users".to_string(), set);

                let mut user_setting = ClientLinkSettings::new("educationUsers");
                user_setting.use_method_name("education");

                let mut set = BTreeSet::new();
                set.extend(vec![user_setting]);
                map.insert("user".to_string(), set);
            },
            _ => {},
        }

        map
    }

    // Modifiers that need to be explicitly declared.
    // The struct names for clients are generated based on the operation id
    // which is also modified when the clients are generated. This can result
    // in naming conflicts that is fixed by these modifiers.
    pub fn target_modifiers(resource_identity: ResourceIdentity) -> ModifierMap {
        let mut modify_target = ModifierMap::default();
        match resource_identity {
            ResourceIdentity::Activities => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.ListActivities".to_string()),
                    vec![
                        MatchTarget::OperationMap("activities".to_string()),
                        MatchTarget::OperationId("activities.ListActivities".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.GetActivities".to_string()),
                    vec![
                        MatchTarget::OperationMap("activities".to_string()),
                        MatchTarget::OperationId("activities.GetActivities".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.UpdateActivities".to_string()),
                    vec![
                        MatchTarget::OperationMap("activities".to_string()),
                        MatchTarget::OperationId("activities.UpdateActivities".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.CreateActivities".to_string()),
                    vec![
                        MatchTarget::OperationMap("activities".to_string()),
                        MatchTarget::OperationId("activities.CreateActivities".to_string()),
                    ],
                );
            },
            ResourceIdentity::AuditLogs => {
                modify_target.operation_map("auditLogs.auditLogRoot", "auditLogs");
            },
            ResourceIdentity::Calendar => {
                modify_target.map.insert(
                    MatchTarget::OperationId("users.ListCalendars".to_string()),
                    vec![
                        MatchTarget::OperationId("users.calendar.ListCalendars".to_string()),
                        MatchTarget::OperationMap("users.calendar".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("users.CreateCalendar".to_string()),
                    vec![
                        MatchTarget::OperationId("users.calendar.CreateCalendar".to_string()),
                        MatchTarget::OperationMap("users.calendar".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("users.GetCalendar".to_string()),
                    vec![
                        MatchTarget::OperationId("users.calendar.GetCalendar".to_string()),
                        MatchTarget::OperationMap("users.calendar".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("users.GetCalendars".to_string()),
                    vec![
                        MatchTarget::OperationId("users.calendars.GetCalendars".to_string()),
                        MatchTarget::OperationMap("users.calendars".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("users.UpdateCalendar".to_string()),
                    vec![
                        MatchTarget::OperationId("users.calendar.UpdateCalendar".to_string()),
                        MatchTarget::OperationMap("users.calendar".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("users.UpdateCalendars".to_string()),
                    vec![
                        MatchTarget::OperationId("users.calendars.UpdateCalendars".to_string()),
                        MatchTarget::OperationMap("users.calendars".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId(
                        "users.calendar.calendarView.calendar.getSchedule".to_string(),
                    ),
                    vec![
                        MatchTarget::OperationId(
                            "users.calendar.calendarView.getSchedule".to_string(),
                        ),
                        MatchTarget::OperationMap("users.calendar.calendarView".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId(
                        "users.calendar.events.calendar.getSchedule".to_string(),
                    ),
                    vec![
                        MatchTarget::OperationId("users.calendar.events.getSchedule".to_string()),
                        MatchTarget::OperationMap("users.calendar.events".to_string()),
                    ],
                );
            },
            ResourceIdentity::CalendarGroups => {
                modify_target.map.insert(
                    MatchTarget::OperationId("users.GetCalendarGroups".to_string()),
                    vec![
                        MatchTarget::OperationId(
                            "users.calendarGroups.GetCalendarGroups".to_string(),
                        ),
                        MatchTarget::OperationMap("users.calendarGroups".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("users.UpdateCalendarGroups".to_string()),
                    vec![
                        MatchTarget::OperationId(
                            "users.calendarGroups.UpdateCalendarGroups".to_string(),
                        ),
                        MatchTarget::OperationMap("users.calendarGroups".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("users.ListCalendarGroups".to_string()),
                    vec![
                        MatchTarget::OperationId(
                            "users.calendarGroups.ListCalendarGroups".to_string(),
                        ),
                        MatchTarget::OperationMap("users.calendarGroups".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId(
                        "users.calendarGroups.calendars.events.calendar.getSchedule".to_string(),
                    ),
                    vec![
                        MatchTarget::OperationId(
                            "users.calendarGroups.calendars.events.getSchedule".to_string(),
                        ),
                        MatchTarget::OperationMap(
                            "users.calendarGroups.calendars.events".to_string(),
                        ),
                    ],
                );
            },
            ResourceIdentity::CalendarView => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.ListCalendarView".to_string()),
                    vec![
                        MatchTarget::OperationMap("calendarViews".to_string()),
                        MatchTarget::OperationId("calendarViews.ListCalendarView".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.GetCalendarView".to_string()),
                    vec![
                        MatchTarget::OperationMap("calendarView".to_string()),
                        MatchTarget::OperationId("calendarView.GetCalendarView".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.UpdateCalendarView".to_string()),
                    vec![
                        MatchTarget::OperationMap("calendarView".to_string()),
                        MatchTarget::OperationId("calendarView.UpdateCalendarView".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.CreateCalendarView".to_string()),
                    vec![
                        MatchTarget::OperationMap("calendarViews".to_string()),
                        MatchTarget::OperationId("calendarViews.CreateCalendarView".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.calendarView.delta.fa14".to_string()),
                    vec![
                        MatchTarget::OperationMap("calendarViews".to_string()),
                        MatchTarget::OperationId("calendarViews.delta".to_string()),
                    ],
                );
            },
            ResourceIdentity::ContactFolders => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.GetContactFolders".to_string()),
                    vec![
                        MatchTarget::OperationMap("contactFolders".to_string()),
                        MatchTarget::OperationId("contactFolders.GetContactFolders".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.UpdateContactFolders".to_string()),
                    vec![
                        MatchTarget::OperationMap("contactFolders".to_string()),
                        MatchTarget::OperationId("contactFolders.UpdateContactFolders".to_string()),
                    ],
                );
            },
            ResourceIdentity::Contacts => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.GetContacts".to_string()),
                    vec![
                        MatchTarget::OperationMap("contacts".to_string()),
                        MatchTarget::OperationId("contacts.GetContacts".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.UpdateContacts".to_string()),
                    vec![
                        MatchTarget::OperationMap("contacts".to_string()),
                        MatchTarget::OperationId("contacts.UpdateContacts".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.ListContacts".to_string()),
                    vec![
                        MatchTarget::OperationMap("contacts".to_string()),
                        MatchTarget::OperationId("contacts.ListContacts".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.CreateContacts".to_string()),
                    vec![
                        MatchTarget::OperationMap("contacts".to_string()),
                        MatchTarget::OperationId("contacts.CreateContacts".to_string()),
                    ],
                );
            },
            ResourceIdentity::ContentTypes => {
                modify_target.map.insert(
                    MatchTarget::OperationId("sites.ListContentTypes".to_string()),
                    vec![
                        MatchTarget::OperationMap("sites.contentTypes".to_string()),
                        MatchTarget::OperationId("sites.contentTypes.ListContentTypes".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("sites.GetContentTypes".to_string()),
                    vec![
                        MatchTarget::OperationMap("sites.contentTypes".to_string()),
                        MatchTarget::OperationId("sites.contentTypes.GetContentTypes".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("sites.UpdateContentTypes".to_string()),
                    vec![
                        MatchTarget::OperationMap("sites.contentTypes".to_string()),
                        MatchTarget::OperationId(
                            "sites.contentTypes.UpdateContentTypes".to_string(),
                        ),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("sites.CreateContentTypes".to_string()),
                    vec![
                        MatchTarget::OperationMap("sites.contentTypes".to_string()),
                        MatchTarget::OperationId(
                            "sites.contentTypes.CreateContentTypes".to_string(),
                        ),
                    ],
                );
            },
            ResourceIdentity::DeviceManagement => {
                modify_target.operation_map(
                    "deviceManagement.detectedApps.managedDevices",
                    "deviceManagement.detectedApps.appManagedDevices",
                );
            },
            ResourceIdentity::Directory => {
                modify_target.operation_map(
                    "directoryObjects.microsoft.graph.administrativeUnit",
                    "directoryObjects.administrativeUnits",
                );
                modify_target.map.insert(
                    MatchTarget::OperationId(
                        "directory.administrativeUnits.delta.fa14".to_string(),
                    ),
                    vec![
                        MatchTarget::OperationId(
                            "directoryObjects.administrativeUnits.delta".to_string(),
                        ),
                        MatchTarget::OperationMap(
                            "directoryObjects.administrativeUnits".to_string(),
                        ),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("directoryRoles.delta.fa14".to_string()),
                    vec![
                        MatchTarget::OperationId("directoryRoles.delta".to_string()),
                        MatchTarget::OperationMap("directoryRoles".to_string()),
                    ],
                );
            },
            ResourceIdentity::Events => {
                modify_target.map.insert(
                    MatchTarget::OperationId("calendar.events.UpdateInstances".to_string()),
                    vec![MatchTarget::OperationMap(
                        "calendar.events.instances".to_string(),
                    )],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("calendar.events.GetInstances".to_string()),
                    vec![
                        MatchTarget::OperationMap("calendar.events.instances".to_string()),
                        MatchTarget::OperationId(
                            "calendar.events.instances.GetInstances".to_string(),
                        ),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("calendar.events.ListInstances".to_string()),
                    vec![
                        MatchTarget::OperationMap("calendar.events.instances".to_string()),
                        MatchTarget::OperationId(
                            "calendar.events.instances.ListInstances".to_string(),
                        ),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("calendar.events.CreateInstances".to_string()),
                    vec![
                        MatchTarget::OperationMap("calendar.events.instances".to_string()),
                        MatchTarget::OperationId(
                            "calendar.events.instances.CreateInstances".to_string(),
                        ),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("users.ListEvents".to_string()),
                    vec![
                        MatchTarget::OperationMap("users.event".to_string()),
                        MatchTarget::OperationId("users.event.ListEvents".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("users.CreateEvents".to_string()),
                    vec![
                        MatchTarget::OperationMap("users.event".to_string()),
                        MatchTarget::OperationId("users.event.CreateEvents".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("users.UpdateEvents".to_string()),
                    vec![
                        MatchTarget::OperationMap("users.events".to_string()),
                        MatchTarget::OperationId("users.events.UpdateEvents".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("users.GetEvents".to_string()),
                    vec![
                        MatchTarget::OperationMap("users.events".to_string()),
                        MatchTarget::OperationId("users.events.GetEvents".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("users.events.delta.fa14".to_string()),
                    vec![
                        MatchTarget::OperationMap("users.event".to_string()),
                        MatchTarget::OperationId("users.event.delta.fa14".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationMap("users.events.calendar.events".to_string()),
                    vec![MatchTarget::OperationMap(
                        "users.events.calendar".to_string(),
                    )],
                );
            },
            ResourceIdentity::GroupLifecyclePolicies => {
                modify_target.map.insert(
                    MatchTarget::OperationMap(
                        "groupLifecyclePolicies.groupLifecyclePolicy".to_string(),
                    ),
                    vec![MatchTarget::OperationMap(
                        "groupLifecyclePolicies".to_string(),
                    )],
                );
            },
            ResourceIdentity::Instances => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.calendarView.ListInstances".to_string()),
                    vec![
                        MatchTarget::OperationMap("instances".to_string()),
                        MatchTarget::OperationId("instances.ListInstances".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.calendarView.CreateInstances".to_string()),
                    vec![
                        MatchTarget::OperationMap("instances".to_string()),
                        MatchTarget::OperationId("instances.CreateInstances".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.calendarView.GetInstances".to_string()),
                    vec![
                        MatchTarget::OperationMap("instances".to_string()),
                        MatchTarget::OperationId("instances.GetInstances".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.calendarView.UpdateInstances".to_string()),
                    vec![
                        MatchTarget::OperationMap("instances".to_string()),
                        MatchTarget::OperationId("instances.UpdateInstances".to_string()),
                    ],
                );
            },
            ResourceIdentity::InferenceClassification => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.GetInferenceClassification".to_string()),
                    vec![
                        MatchTarget::OperationMap("inferenceClassification".to_string()),
                        MatchTarget::OperationId(
                            "inferenceClassification.GetInferenceClassification".to_string(),
                        ),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.UpdateInferenceClassification".to_string()),
                    vec![
                        MatchTarget::OperationMap("inferenceClassification".to_string()),
                        MatchTarget::OperationId(
                            "inferenceClassification.UpdateInferenceClassification".to_string(),
                        ),
                    ],
                );
            },
            ResourceIdentity::Insights => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.GetInsights".to_string()),
                    vec![
                        MatchTarget::OperationMap("insights".to_string()),
                        MatchTarget::OperationId("insights.GetInsights".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.UpdateInsights".to_string()),
                    vec![
                        MatchTarget::OperationMap("insights".to_string()),
                        MatchTarget::OperationId("insights.UpdateInsights".to_string()),
                    ],
                );
            },
            ResourceIdentity::Items => {
                modify_target.map.insert(
                    MatchTarget::OperationId("sites.lists.ListItems".to_string()),
                    vec![
                        MatchTarget::OperationMap("items".to_string()),
                        MatchTarget::OperationId("items.ListItems ".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("sites.lists.CreateItems".to_string()),
                    vec![
                        MatchTarget::OperationMap("items".to_string()),
                        MatchTarget::OperationId("items.CreateItems".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("sites.lists.GetItems".to_string()),
                    vec![
                        MatchTarget::OperationMap("items".to_string()),
                        MatchTarget::OperationId("items.GetItems".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("sites.lists.UpdateItems".to_string()),
                    vec![
                        MatchTarget::OperationMap("items".to_string()),
                        MatchTarget::OperationId("items.UpdateItems".to_string()),
                    ],
                );
            },
            ResourceIdentity::Lists => {
                modify_target.map.insert(
                    MatchTarget::OperationId("sites.GetLists".to_string()),
                    vec![
                        MatchTarget::OperationMap("lists".to_string()),
                        MatchTarget::OperationId("lists.GetLists".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("sites.UpdateLists".to_string()),
                    vec![
                        MatchTarget::OperationMap("lists".to_string()),
                        MatchTarget::OperationId("lists.UpdateLists".to_string()),
                    ],
                );
            },
            ResourceIdentity::ManagedDevices => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.GetManagedDevices".to_string()),
                    vec![
                        MatchTarget::OperationMap("managedDevices".to_string()),
                        MatchTarget::OperationId("managedDevices.GetManagedDevices".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.UpdateManagedDevices".to_string()),
                    vec![
                        MatchTarget::OperationMap("managedDevices".to_string()),
                        MatchTarget::OperationId("managedDevices.UpdateManagedDevices".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.GetManagedDevices".to_string()),
                    vec![
                        MatchTarget::OperationMap("managedDevices".to_string()),
                        MatchTarget::OperationId("managedDevices.GetManagedDevices".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.UpdateManagedDevices".to_string()),
                    vec![
                        MatchTarget::OperationMap("managedDevices".to_string()),
                        MatchTarget::OperationId("managedDevices.UpdateManagedDevices".to_string()),
                    ],
                );
            },
            ResourceIdentity::Me => {
                // me.user.GetUser
                modify_target.map.insert(
                    MatchTarget::OperationId("me.user.GetUser".to_string()),
                    vec![
                        MatchTarget::OperationMap("me".to_string()),
                        MatchTarget::OperationId("me.GetUser".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.user.UpdateUser".to_string()),
                    vec![
                        MatchTarget::OperationMap("me".to_string()),
                        MatchTarget::OperationId("me.UpdateUser".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationMap("me.user".to_string()),
                    vec![MatchTarget::OperationMap("me".to_string())],
                );
            },
            ResourceIdentity::Outlook => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.GetOutlook".to_string()),
                    vec![
                        MatchTarget::OperationMap("outlook".to_string()),
                        MatchTarget::OperationId("outlook.GetOutlook".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.UpdateOutlook".to_string()),
                    vec![
                        MatchTarget::OperationMap("outlook".to_string()),
                        MatchTarget::OperationId("outlook.UpdateOutlook".to_string()),
                    ],
                );
            },
            ResourceIdentity::Planner => {
                modify_target.map.insert(
                    MatchTarget::OperationMap("users.planner.plans.tasks".to_string()),
                    vec![MatchTarget::OperationMap(
                        "users.planner.plans.plannerTasks".to_string(),
                    )],
                );
                modify_target.map.insert(
                    MatchTarget::OperationMap("users.planner.plans.buckets.tasks".to_string()),
                    vec![MatchTarget::OperationMap(
                        "users.planner.plans.buckets.bucketTasks".to_string(),
                    )],
                );
            },
            ResourceIdentity::Policies => {
                modify_target.operation_map("policies.policyRoot", "policies");
            },
            ResourceIdentity::Teams => {
                modify_target.map.insert(
                    MatchTarget::OperationMap("teams.primaryChannel.messages".to_string()),
                    vec![MatchTarget::OperationMap(
                        "teams.primaryChannel.primaryChannelMessages".to_string(),
                    )],
                );
                modify_target.map.insert(
                    MatchTarget::OperationMap("teams.primaryChannel.tabs".to_string()),
                    vec![MatchTarget::OperationMap(
                        "teams.primaryChannel.primaryChannelTabs".to_string(),
                    )],
                );
            },
            ResourceIdentity::Settings => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.GetSettings".to_string()),
                    vec![
                        MatchTarget::OperationMap("settings".to_string()),
                        MatchTarget::OperationId("settings.GetSettings".to_string()),
                    ],
                );

                modify_target.map.insert(
                    MatchTarget::OperationId("me.UpdateSettings".to_string()),
                    vec![
                        MatchTarget::OperationMap("settings".to_string()),
                        MatchTarget::OperationId("settings.UpdateSettings".to_string()),
                    ],
                );
            },
            _ => {},
        }
        modify_target
    }
}
