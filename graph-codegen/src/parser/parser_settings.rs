use crate::builder::ClientLinkSettings;
use crate::parser::filter::{
    Filter, FilterIgnore, MatchTarget, ModifierMap, SecondaryModifierMap, UrlMatchTarget,
};
use graph_core::resource::ResourceIdentity;
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

pub struct ParserSettings;

impl ParserSettings {
    /// Imports that won't be added from parsing and need to be manually
    /// returned here.
    pub fn imports(resource_identity: ResourceIdentity) -> Vec<&'static str> {
        match resource_identity {
            ResourceIdentity::Calendar => vec![
                //"crate::calendars::CalendarsRequest",
                "crate::events::EventsRequest",
                "crate::core::ResourceIdentity",
            ],
            ResourceIdentity::Calendars => vec![
                "crate::events::EventsRequest",
                "crate::core::ResourceIdentity",
            ],
            ResourceIdentity::Users => vec![
                "crate::calendar_groups::CalendarGroupsRequest",
                "crate::calendar::CalendarsRequest",
                "crate::calendar::CalendarRequest",
                "crate::core::ResourceIdentity",
                "crate::events::EventsRequest",
                "crate::events::EventRequest",
            ],
            ResourceIdentity::Me => vec![
                "crate::calendar_groups::CalendarGroupsRequest",
                "crate::calendar::CalendarsRequest",
                "crate::calendar::CalendarRequest",
                "crate::core::ResourceIdentity",
                "crate::events::EventsRequest",
                "crate::events::EventRequest",
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

    // normal calendar only ignore:
    /*
                       "calendarGroup",
                   "calendars/{calendar-id}",
                   "calendars/{{RID}}",
                   "users/{user-id}/calendarView",
                   "users/{{RID}}/calendarView",
                   "{{RID}}/calendarView",
                   "{user-id}/calendarView",
                   "users/{user-id}/calendars",
                   "users/{{RID}}/calendars",
                   "events",
                   "instances",
    */

    pub fn path_filters(resource_identity: ResourceIdentity) -> Vec<Filter<'static>> {
        match resource_identity {
            ResourceIdentity::Calendar => {
                vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                    "calendarGroup",
                    "calendarView",
                    "events",
                    "instances",
                ]))]
            },
            ResourceIdentity::Calendars => {
                vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                    "calendarGroup",
                    "users/{user-id}/calendarView",
                    "users/{{RID}}/calendarView",
                    "instances",
                    "events",
                ]))]
            },
            ResourceIdentity::CalendarGroups => {
                vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                    "/calendar/",
                ]))]
            },
            ResourceIdentity::Me => vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "calendarGroup",
                "calendars",
                "calendar",
                "calendarView",
                "events",
                "instances",
                "mailFolders",
                "messages",
                "onenote",
            ]))],
            ResourceIdentity::Users => {
                vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                    "calendarGroup",
                    "calendars",
                    "calendar",
                    "calendarView",
                    "events",
                    "instances",
                    "mailFolders",
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
    // and add a link between them.
    pub fn secondary_modifier_map(resource_identity: ResourceIdentity) -> SecondaryModifierMap {
        let mut map = SecondaryModifierMap::with_capacity(15);

        match resource_identity {
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
            ResourceIdentity::Calendars => {
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
            ResourceIdentity::Events => {
                map.insert(
                    "users.events",
                    MatchTarget::OperationId("events".to_string()),
                );
            },
            ResourceIdentity::Me => {
                map.insert("me.user", MatchTarget::OperationMap("me".to_string()));
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
            ResourceIdentity::Users => vec![UrlMatchTarget::resource_id("users", "user")],
            ResourceIdentity::Sites => vec![UrlMatchTarget::resource_id("sites", "site")],
            ResourceIdentity::Groups => vec![UrlMatchTarget::resource_id("groups", "group")],
            ResourceIdentity::Drives => vec![UrlMatchTarget::resource_id("drives", "drive")],
            ResourceIdentity::Teams => vec![UrlMatchTarget::resource_id("teams", "team")],
            ResourceIdentity::Workbooks => {
                vec![UrlMatchTarget::resource_id("workbooks", "workbook")]
            },
            ResourceIdentity::Calendar => {
                vec![UrlMatchTarget::resource_id("calendars", "calendar")]
            },
            ResourceIdentity::Calendars => {
                vec![UrlMatchTarget::resource_id("calendars", "calendar")]
            },
            ResourceIdentity::Events => vec![UrlMatchTarget::resource_id("events", "event")],
            _ => vec![],
        }
    }

    pub fn client_link_settings(
        resource_identity: ResourceIdentity,
    ) -> BTreeMap<String, BTreeSet<ClientLinkSettings>> {
        let mut map = BTreeMap::new();
        match resource_identity {
            ResourceIdentity::Calendar => {
                let mut settings = ClientLinkSettings::new("events");
                settings
                    .use_method_name("event")
                    .with_extend_path_ident()
                    .with_set_resource_identity()
                    .with_id_param();
                let mut settings2 = ClientLinkSettings::new("calendars");
                settings2.as_id_method_link();

                let mut set = BTreeSet::new();
                set.insert(settings);
                set.insert(settings2);
                map.insert("calendar".to_string(), set);
            },
            ResourceIdentity::Calendars => {
                let mut settings = ClientLinkSettings::new("events");
                settings
                    .use_method_name("event")
                    .with_id_param()
                    .with_extend_path_id()
                    .with_extend_path_ident();
                let mut set = BTreeSet::new();
                set.insert(settings);
                map.insert("calendars".to_string(), set);
            },
            ResourceIdentity::Me => {
                let mut settings = ClientLinkSettings::new("calendarGroups");
                settings.with_extend_path_ident();

                let mut settings2 = ClientLinkSettings::new("calendars");
                settings2
                    .use_method_name("calendar")
                    .with_id_param()
                    .with_set_resource_identity()
                    .with_extend_path_ident();

                let mut settings3 = ClientLinkSettings::new("calendar");
                settings3
                    .use_method_name("calendars")
                    .with_set_resource_identity()
                    .with_extend_path_ident();

                let mut settings4 = ClientLinkSettings::new("event");
                settings4
                    .use_method_name("events")
                    .with_set_resource_identity()
                    .with_extend_path_ident();

                let mut settings5 = ClientLinkSettings::new("events");
                settings5
                    .use_method_name("event")
                    .with_id_param()
                    .with_extend_path_ident();

                let mut set = BTreeSet::new();
                set.extend(vec![settings, settings2, settings3, settings4, settings5]);
                map.insert("me".to_string(), set);
            },
            ResourceIdentity::Users => {
                let mut settings = ClientLinkSettings::new("calendarGroups");
                settings.with_extend_path_id().with_extend_path_ident();

                let mut settings2 = ClientLinkSettings::new("calendars");
                settings2
                    .use_method_name("calendar")
                    .with_id_param()
                    .with_set_resource_identity()
                    .with_extend_path_id()
                    .with_extend_path_ident();

                let mut settings3 = ClientLinkSettings::new("calendar");
                settings3
                    .use_method_name("calendars")
                    .with_set_resource_identity()
                    .with_extend_path_id()
                    .with_extend_path_ident();

                let mut settings4 = ClientLinkSettings::new("event");
                settings4
                    .use_method_name("events")
                    .with_set_resource_identity()
                    .with_extend_path_id()
                    .with_extend_path_ident();

                let mut settings5 = ClientLinkSettings::new("events");
                settings5
                    .use_method_name("event")
                    .with_id_param()
                    .with_extend_path_id()
                    .with_extend_path_ident();

                let mut set = BTreeSet::new();
                set.extend(vec![settings, settings2, settings3, settings4, settings5]);
                map.insert("users".to_string(), set);
            },
            _ => {},
        }

        map
    }

    pub fn target_modifiers(resource_identity: ResourceIdentity) -> ModifierMap {
        let mut modify_target = ModifierMap::default();
        match resource_identity {
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
            ResourceIdentity::Calendars => {
                modify_target.map.insert(
                    MatchTarget::OperationId("users.UpdateCalendars".to_string()),
                    vec![
                        MatchTarget::OperationId("users.calendars.UpdateCalendars".to_string()),
                        MatchTarget::OperationMap("users.calendars".to_string()),
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
                    MatchTarget::OperationId(
                        "users.calendars.events.calendar.getSchedule".to_string(),
                    ),
                    vec![
                        MatchTarget::OperationId("users.calendars.events.getSchedule".to_string()),
                        MatchTarget::OperationMap("users.calendars.events".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId(
                        "users.calendars.calendarView.calendar.getSchedule".to_string(),
                    ),
                    vec![
                        MatchTarget::OperationId(
                            "users.calendars.calendarView.getSchedule".to_string(),
                        ),
                        MatchTarget::OperationMap("users.calendars.calendarView".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId(
                        "users.calendars.calendarView.calendar.getSchedule".to_string(),
                    ),
                    vec![
                        MatchTarget::OperationId(
                            "users.calendars.calendarView.getSchedule".to_string(),
                        ),
                        MatchTarget::OperationMap("users.calendars.calendarView".to_string()),
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
            ResourceIdentity::Users => {
                modify_target.operation_map(
                    "users.contactFolders.contacts",
                    "users.contactFolders.contactFolderContact",
                );
            },
            _ => {},
        }
        modify_target
    }
}
