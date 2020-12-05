use crate::parser::filter::{Filter, FilterIgnore};
use graph_core::resource::ResourceIdentity;

pub fn get_path_filters(resource_identity: ResourceIdentity) -> Vec<Filter<'static>> {
    match resource_identity {
        ResourceIdentity::Buckets => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "buckets/{plannerBucket-id}/tasks/",
            ]))]
        },
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
        ResourceIdentity::Conversations => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "/threads/",
            ]))]
        },
        ResourceIdentity::Drives | ResourceIdentity::Drive => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "/list/", "versions", "items",
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
        ResourceIdentity::MailFolders => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "/move", "messages",
            ]))]
        },
        ResourceIdentity::Messages => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "/move",
            ]))]
        },
        ResourceIdentity::Onenote => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "sections/{onenoteSection-id}",
                "sectionGroups/{sectionGroup-id}",
                "pages/{onenotePage-id}",
                "notebooks/{notebook-id}",
                "getNotebookFromWebUrl",
            ]))]
        },
        ResourceIdentity::Pages => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "sections/{onenoteSection-id}",
                "sectionGroups/{sectionGroup-id}",
                "notebooks/{notebook-id}",
                "/parentNotebook/",
                "/parentSection/",
            ]))]
        },
        ResourceIdentity::Notebooks => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "sections/{onenoteSection-id}",
                "sectionGroups/{sectionGroup-id}",
                "pages/{onenotePage-id}",
            ]))]
        },
        ResourceIdentity::SectionGroups => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "sections/{onenoteSection-id}",
                "pages/{onenotePage-id}",
                "notebooks/{notebook-id}",
                "/sectionGroups/{sectionGroup-id}/sectionGroups/{sectionGroup-id}",
            ]))]
        },
        ResourceIdentity::Sections => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "pages/{onenotePage-id}",
                "sectionGroups/{sectionGroup-id}",
                "notebooks/{notebook-id}",
                "/parentSectionGroup/",
                "/parentNotebook/",
            ]))]
        },
        ResourceIdentity::ParentNotebook => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "/parentNotebook/sectionGroups/{sectionGroup-id}",
                "/parentNotebook/sections/{onenoteSection-id}",
            ]))]
        },
        ResourceIdentity::ParentSectionGroup => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "/parentSectionGroup/parentNotebook/",
                "/parentSectionGroup/sectionGroups/",
                "/parentSectionGroup/sections/",
                "/parentSectionGroup/parentSectionGroup",
            ]))]
        },
        ResourceIdentity::ParentSection => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "/parentSection/pages/",
                "/parentSectionGroup/",
                "/parentNotebook/",
            ]))]
        },
        ResourceIdentity::Plans => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "/buckets/",
                "/tasks/",
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
            "planner",
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
                "planner",
            ]))]
        },
        ResourceIdentity::Groups => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "/calendarGroup/",
                "/calendars/",
                "/calendar/",
                "/calendarView/",
                "/events/",
                "/onenote/",
                "/planner/",
                "/conversations/",
                "/threads/",
                "/conversations/",
            ]))]
        },
        ResourceIdentity::Threads => {
            vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
                "/posts/",
            ]))]
        },
        _ => vec![Filter::IgnoreIf(FilterIgnore::PathContainsMulti(vec![
            "singleValueExtendedProperties",
            "multiValueExtendedProperties",
            // These are basically like OData queries and look like getByPath(path={path})
            // but we dont currently handle these so they are ignored. The get activities
            // by interval is used the most in these situations.
            "={",
            "getActivitiesByInterval",
        ]))],
    }
}
