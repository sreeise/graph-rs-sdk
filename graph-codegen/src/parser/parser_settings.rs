use crate::builder::ClientLinkSettings;
use crate::parser::filter::{
    Filter, FilterIgnore, MatchTarget, ModifierMap, SecondaryModifierMap, UrlMatchTarget,
};
use crate::parser::ResponseType::Collection;
use crate::parser::{
    DirectoryModFile, HttpMethod, Request, RequestMap, RequestSet, RequestType, ResourceNames,
    ResponseType,
};
use graph_core::resource::ResourceIdentity;
use graph_http::GraphResponse;
use inflector::Inflector;
use std::collections::{BTreeMap, BTreeSet, HashMap};

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
            ResourceIdentity::Drive | ResourceIdentity::Drives => vec![
                "std::path::Path",
                "crate::core::ResourceIdentity",
                "crate::items::{ItemRequest, ItemsRequest}",
                "crate::lists::{ListRequest, ListsRequest}",
                "graph_http::types::DeltaPhantom",
                // TODO: Handlebars should be imported by the builder. Figure out why this is not happening.
                "handlebars::*",
            ],
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
                "crate::drive::DrivesRequest",
                "crate::onenote::OnenoteRequest",
            ],
            ResourceIdentity::Onenote => vec![
                "crate::core::ResourceIdentity",
                "crate::notebooks::{NotebookRequest, NotebooksRequest}",
                "crate::pages::{PagesRequest, PageRequest}",
                "crate::sections::{SectionRequest, SectionsRequest}",
                "crate::section_groups::{SectionGroupRequest, SectionGroupsRequest}",
            ],
            ResourceIdentity::Pages => vec![
                "crate::core::ResourceIdentity",
                "crate::parent_notebook::ParentNotebookRequest",
                "crate::parent_section::ParentSectionRequest",
                "graph_http::{BlockingDownload, AsyncDownload, BlockingHttpClient, AsyncHttpClient, RequestClient}",
                "std::path::Path",
            ],
            ResourceIdentity::Notebooks => vec![
                "crate::core::ResourceIdentity",
                "crate::sections::SectionsRequest",
                "crate::section_groups::SectionGroupsRequest",
            ],
            ResourceIdentity::SectionGroups => vec![
                "crate::core::ResourceIdentity",
                "crate::sections::SectionsRequest",
            ],
            ResourceIdentity::Sections => vec![
                "crate::core::ResourceIdentity",
                "crate::pages::PagesRequest",
                "crate::section_groups::SectionGroupsRequest",
                "crate::parent_notebook::ParentNotebookRequest",
                "crate::parent_section_group::ParentSectionGroupRequest",
            ],
            ResourceIdentity::ParentNotebook => vec![
                "crate::core::ResourceIdentity",
                "crate::sections::SectionsRequest",
                "crate::section_groups::SectionGroupsRequest",
            ],
            ResourceIdentity::ParentSectionGroup => vec![
                "crate::core::ResourceIdentity",
                "crate::sections::SectionsRequest",
                "crate::section_groups::SectionGroupsRequest",
                "crate::parent_notebook::ParentNotebookRequest",
            ],
            ResourceIdentity::ParentSection => vec![
                "crate::core::ResourceIdentity",
                "crate::pages::PagesRequest",
                "crate::parent_section_group::ParentSectionGroupRequest",
                "crate::parent_notebook::ParentNotebookRequest",
            ],
            ResourceIdentity::ManagedDevices => vec!["crate::core::ResourceIdentity"],
            ResourceIdentity::MailFolders => vec!["crate::core::ResourceIdentity"],
            ResourceIdentity::Messages => vec!["crate::core::ResourceIdentity"],
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
                "crate::drive::DrivesRequest",
                "crate::onenote::OnenoteRequest",
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
                "crate::drive::DrivesRequest",
                "crate::onenote::OnenoteRequest",
                "crate::core::ResourceIdentity",
            ],
            _ => vec![],
        }
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
            _ => ParserSettings::default_path_filters(),
        }
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
        let vec = {
            match resource_identity {
                ResourceIdentity::Drive | ResourceIdentity::Drives => vec![
                    Request {
                        path: "/{{drive_item_id}}/children".to_string(),
                        method: HttpMethod::GET,
                        method_name: "list_children".to_string(),
                        param_size: 1,
                        has_body: false,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::Collection,
                        tag: Default::default(),
                        operation_id: "drives.ListChildren".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "/{{drive_item_id}}/activities".to_string(),
                        method: HttpMethod::GET,
                        method_name: "get_item_activities".to_string(),
                        param_size: 1,
                        has_body: false,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::Collection,
                        tag: Default::default(),
                        operation_id: "drives.GetActivities".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "/{{drive_item_id}}".to_string(),
                        method: HttpMethod::PATCH,
                        method_name: "update_items".to_string(),
                        param_size: 1,
                        has_body: true,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::SerdeJson,
                        tag: Default::default(),
                        operation_id: "drives.UpdateItems".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "/{{drive_item_id}}".to_string(),
                        method: HttpMethod::DELETE,
                        method_name: "delete_items".to_string(),
                        param_size: 1,
                        has_body: false,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::NoContent,
                        tag: Default::default(),
                        operation_id: "drives.DeleteItems".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "/{{drive_item_id}}".to_string(),
                        method: HttpMethod::GET,
                        method_name: "get_items".to_string(),
                        param_size: 1,
                        has_body: false,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::SerdeJson,
                        tag: Default::default(),
                        operation_id: "drives.GetItems".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "/{{drive_item_id}}/children".to_string(),
                        method: HttpMethod::POST,
                        method_name: "create_folder".to_string(),
                        param_size: 1,
                        has_body: true,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::SerdeJson,
                        tag: Default::default(),
                        operation_id: "drives.CreateFolder".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "/{{drive_root}}/root/children".to_string(),
                        method: HttpMethod::POST,
                        method_name: "create_root_folder".to_string(),
                        param_size: 0,
                        has_body: true,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::SerdeJson,
                        tag: Default::default(),
                        operation_id: "drives.CreateRootFolder".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "/{{drive_item_id}}/copy".to_string(),
                        method: HttpMethod::POST,
                        method_name: "copy_item".to_string(),
                        param_size: 1,
                        has_body: true,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::NoContent,
                        tag: Default::default(),
                        operation_id: "drives.CopyItem".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "/{{drive_item_id}}/versions".to_string(),
                        method: HttpMethod::GET,
                        method_name: "list_item_versions".to_string(),
                        param_size: 1,
                        has_body: false,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::Collection,
                        tag: Default::default(),
                        operation_id: "drives.ListItemVersions".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "/{{drive_item_id}}/versions/{{id2}}/restoreVersion".to_string(),
                        method: HttpMethod::POST,
                        method_name: "restore_item_versions".to_string(),
                        param_size: 2,
                        has_body: false,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::NoContent,
                        tag: Default::default(),
                        operation_id: "drives.RestoreItemVersions".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "/{{drive_item}}/thumbnails".to_string(),
                        method: HttpMethod::GET,
                        method_name: "list_thumbnails".to_string(),
                        param_size: 0,
                        has_body: false,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::NoContent,
                        tag: Default::default(),
                        operation_id: "drives.ListThumbnails".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "/{{drive_item_id}}/thumbnails/{{id2}}/{{id3}}".to_string(),
                        method: HttpMethod::GET,
                        method_name: "get_thumbnail".to_string(),
                        param_size: 3,
                        has_body: false,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::NoContent,
                        tag: Default::default(),
                        operation_id: "drives.GetThumbnail".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "/{{drive_item_id}}/thumbnails/{{id2}}/{{id3}}/content".to_string(),
                        method: HttpMethod::GET,
                        method_name: "get_thumbnail_binary".to_string(),
                        param_size: 3,
                        has_body: false,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::NoContent,
                        tag: Default::default(),
                        operation_id: "drives.GetThumbnailBinary".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    // TODO: Setting files as the body in a request macro
                    Request {
                        path: "/{{drive_item_id}}/content".to_string(),
                        method: HttpMethod::PUT,
                        method_name: "upload_replace".to_string(),
                        param_size: 1,
                        has_body: false,
                        request_type: RequestType::Upload,
                        has_rid: false,
                        response: ResponseType::SerdeJson,
                        tag: Default::default(),
                        operation_id: "drives.UploadReplace".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "/{{drive_item_id}}/content".to_string(),
                        method: HttpMethod::GET,
                        method_name: "get_item_content".to_string(),
                        param_size: 1,
                        has_body: false,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::SerdeJson,
                        tag: Default::default(),
                        operation_id: "drives.GetItemContent".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "/{{drive_item_id}}".to_string(),
                        method: HttpMethod::POST,
                        method_name: "move_items".to_string(),
                        param_size: 1,
                        has_body: true,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::SerdeJson,
                        tag: Default::default(),
                        operation_id: "drives.MoveItems".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "/{{drive_root}}/root/children".to_string(),
                        method: HttpMethod::GET,
                        method_name: "list_root_children".to_string(),
                        param_size: 0,
                        has_body: false,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::Collection,
                        tag: Default::default(),
                        operation_id: "drives.ListRootChildren".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "{{drive_root}}/activities".to_string(),
                        method: HttpMethod::GET,
                        method_name: "list_root_activities".to_string(),
                        param_size: 0,
                        has_body: false,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::Collection,
                        tag: Default::default(),
                        operation_id: "drives.ListRootActivities".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "{{drive_root}}/root/delta".to_string(),
                        method: HttpMethod::GET,
                        method_name: "delta".to_string(),
                        param_size: 0,
                        has_body: false,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::Delta,
                        tag: Default::default(),
                        operation_id: "drives.Delta".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                    Request {
                        path: "/{{drive_item_id}}/checkin".to_string(),
                        method: HttpMethod::POST,
                        method_name: "check_in_item".to_string(),
                        param_size: 1,
                        has_body: true,
                        request_type: RequestType::Normal,
                        has_rid: false,
                        response: ResponseType::NoContent,
                        tag: Default::default(),
                        operation_id: "drives.CheckIn".to_string(),
                        operation_mapping: "drives".to_string(),
                        doc: None,
                    },
                ],
                ResourceIdentity::Items => vec![Request {
                    path: "/items/{{RID}}".into(),
                    method: HttpMethod::DELETE,
                    method_name: "delete_items".into(),
                    param_size: 1,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: true,
                    response: ResponseType::NoContent,
                    tag: "drives.driveItem".into(),
                    operation_id: "items.DeleteItems".into(),
                    operation_mapping: "items".into(),
                    doc: Some("# Delete navigation property items".into()),
                }],
                ResourceIdentity::Pages => vec![
                    Request {
                        path: "/pages/{{RID}}".into(),
                        method: HttpMethod::DELETE,
                        method_name: "delete_pages".into(),
                        param_size: 0,
                        has_body: false,
                        request_type: RequestType::Normal,
                        has_rid: true,
                        response: ResponseType::NoContent,
                        tag: "pages".into(),
                        operation_id: "me.onenote.pages.DeletePages".into(),
                        operation_mapping: "me.onenote.pages".into(),
                        doc: None,
                    },
                    Request {
                        path: "/pages/{{RID}}/content".into(),
                        method: HttpMethod::PATCH,
                        method_name: "update_page_content".into(),
                        param_size: 0,
                        has_body: true,
                        request_type: RequestType::Normal,
                        has_rid: true,
                        response: ResponseType::SerdeJson,
                        tag: "pages".into(),
                        operation_id: "me.onenote.pages.UpdatePageContent".into(),
                        operation_mapping: "me.onenote.pages".into(),
                        doc: None,
                    },
                    Request {
                        path: "/pages/{{RID}}/content".into(),
                        method: HttpMethod::GET,
                        method_name: "download_page".into(),
                        param_size: 0,
                        has_body: false,
                        request_type: RequestType::Download,
                        has_rid: true,
                        response: ResponseType::Download,
                        tag: "pages".into(),
                        operation_id: "me.onenote.pages.DownloadPage".into(),
                        operation_mapping: "me.onenote.pages".into(),
                        doc: None,
                    },
                    Request {
                        path: "/pages/{{RID}}/content".into(),
                        method: HttpMethod::GET,
                        method_name: "download_page".into(),
                        param_size: 0,
                        has_body: false,
                        request_type: RequestType::AsyncDownload,
                        has_rid: true,
                        response: ResponseType::AsyncDownload,
                        tag: "pages".into(),
                        operation_id: "me.onenote.pages.DownloadPage".into(),
                        operation_mapping: "me.onenote.pages".into(),
                        doc: None,
                    },
                ],
                _ => vec![],
            }
        };

        if vec.is_empty() {
            None
        } else {
            let mut request_set = RequestSet::default();

            for request in vec {
                let mut request_map = RequestMap::default();
                request_map.path = request.path.clone();
                request_map.requests.push_back(request);
                request_set.join_inner_insert(request_map);
            }

            let mut map = HashMap::new();
            match resource_identity {
                ResourceIdentity::Drive | ResourceIdentity::Drives => {
                    map.insert(
                        ResourceIdentity::Drives.to_string().to_camel_case(),
                        request_set,
                    );
                },
                _ => {
                    map.insert(resource_identity.to_string().to_camel_case(), request_set);
                },
            }

            Some(map)
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
                map.insert(
                    "me.calendarView",
                    MatchTarget::OperationMap("calendarViews".to_string()),
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
            ResourceIdentity::Sections => vec![UrlMatchTarget::resource_id("sections", "section")],
            ResourceIdentity::SectionGroups => {
                vec![UrlMatchTarget::resource_id("sectionGroups", "sectionGroup")]
            },
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

                let mut settings4 = ClientLinkSettings::new("calendarView");
                settings4
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings5 = ClientLinkSettings::new("calendarViews");
                settings5
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![settings, settings2, settings3, settings4, settings5]);
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
            ResourceIdentity::CalendarView | ResourceIdentity::CalendarViews => {
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
            ResourceIdentity::Drive | ResourceIdentity::Drives => {
                let mut settings = ClientLinkSettings::new("items");
                settings
                    .use_method_name("item")
                    .use_custom("self.transfer_identity();\n")
                    .with_id_param()
                    .with_set_resource_identity();

                let mut settings2 = ClientLinkSettings::new("item");
                settings2
                    .use_method_name("items")
                    .use_custom("self.transfer_identity();\n");

                let mut settings3 = ClientLinkSettings::new("list");
                settings3
                    .use_method_name("lists")
                    .use_custom("self.transfer_identity();\n")
                    .with_set_resource_identity();

                let mut settings4 = ClientLinkSettings::new("lists");
                settings4
                    .use_method_name("list")
                    .use_custom("self.transfer_identity();\n")
                    .with_id_param()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![settings, settings2, settings3, settings4]);
                map.insert("drives".to_string(), set);

                let mut settings5 = ClientLinkSettings::new("item");
                settings5.use_method_name("items").with_extend_path_ident();

                let mut settings6 = ClientLinkSettings::new("list");
                settings6.use_method_name("lists").with_extend_path_ident();

                let mut set2 = BTreeSet::new();
                set2.extend(vec![settings5, settings6]);
                map.insert("drive".to_string(), set2);
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
            ResourceIdentity::Onenote => {
                let mut settings = ClientLinkSettings::new("pages");
                settings
                    .use_method_name("page")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings2 = ClientLinkSettings::new("page");
                settings2
                    .use_method_name("pages")
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings3 = ClientLinkSettings::new("notebooks");
                settings3
                    .use_method_name("notebook")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings4 = ClientLinkSettings::new("notebook");
                settings4
                    .use_method_name("notebooks")
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings5 = ClientLinkSettings::new("sectionGroups");
                settings5
                    .use_method_name("sectionGroup")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings6 = ClientLinkSettings::new("sectionGroup");
                settings6
                    .use_method_name("sectionGroups")
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings7 = ClientLinkSettings::new("sections");
                settings7
                    .use_method_name("section")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings8 = ClientLinkSettings::new("section");
                settings8
                    .use_method_name("sections")
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![
                    settings, settings2, settings3, settings4, settings5, settings6, settings7,
                    settings8,
                ]);
                map.insert("onenote".to_string(), set);
            },
            ResourceIdentity::Notebooks => {
                let mut settings = ClientLinkSettings::new("sectionGroups");
                settings
                    .use_method_name("sectionGroup")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings2 = ClientLinkSettings::new("sections");
                settings2
                    .use_method_name("section")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![settings, settings2]);
                map.insert("notebooks".to_string(), set);
            },
            ResourceIdentity::Sections => {
                let mut settings = ClientLinkSettings::new("pages");
                settings
                    .use_method_name("page")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings2 = ClientLinkSettings::new("parentNotebook");
                settings2
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings3 = ClientLinkSettings::new("parentSectionGroup");
                settings3
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![settings, settings2, settings3]);
                map.insert("sections".to_string(), set);
            },
            ResourceIdentity::Pages => {
                let mut settings = ClientLinkSettings::new("parentNotebook");
                settings
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut settings2 = ClientLinkSettings::new("parentSection");
                settings2
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![settings, settings2]);
                map.insert("pages".to_string(), set);
            },
            ResourceIdentity::ParentSection => {
                let mut settings = ClientLinkSettings::new("pages");
                settings
                    .use_method_name("page")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings2 = ClientLinkSettings::new("parentSectionGroup");
                settings2
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings3 = ClientLinkSettings::new("parentNotebook");
                settings3
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![settings, settings2, settings3]);
                map.insert("parentSection".to_string(), set);
            },
            ResourceIdentity::SectionGroups => {
                let mut settings = ClientLinkSettings::new("sections");
                settings
                    .use_method_name("section")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![settings]);
                map.insert("sectionGroups".to_string(), set);
            },
            ResourceIdentity::ParentNotebook => {
                let mut settings = ClientLinkSettings::new("sections");
                settings
                    .use_method_name("section")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings2 = ClientLinkSettings::new("sectionGroups");
                settings2
                    .use_method_name("sectionGroup")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![settings, settings2]);
                map.insert("parentNotebook".to_string(), set);
            },
            ResourceIdentity::ParentSectionGroup => {
                let mut settings = ClientLinkSettings::new("sections");
                settings
                    .use_method_name("section")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings2 = ClientLinkSettings::new("sectionGroups");
                settings2
                    .use_method_name("sectionGroup")
                    .with_id_param()
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut settings3 = ClientLinkSettings::new("parentNotebook");
                settings3
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                // Parent section group has requests that have the path /parentSectionGroup/parentSectionGroup
                // and these requests have the same method name, such as get_parent_section_group, which
                // is a conflict because parent section group already has that method name. So we remove
                // those requests entirely and provide a link to the same parent section group which
                // will extend the path correctly when calling these methods.
                let mut settings4 = ClientLinkSettings::new("parentSectionGroup");
                settings4.with_extend_path_ident();

                let mut set = BTreeSet::new();
                set.extend(vec![settings, settings2, settings3, settings4]);
                map.insert("parentSectionGroup".to_string(), set);
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

                let mut settings19 = ClientLinkSettings::new("drives");
                settings19
                    .use_method_name("drive")
                    .with_extend_path_ident()
                    .with_new_method_empty_id()
                    .with_new_method_empty_id();

                let mut settings20 = ClientLinkSettings::new("onenote");
                settings20
                    .with_extend_path_ident()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![
                    settings, settings2, settings3, settings4, settings5, settings6, settings7,
                    settings8, settings9, settings10, settings11, settings12, settings13,
                    settings14, settings15, settings16, settings17, settings18, settings19,
                    settings20,
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

                let mut settings5 = ClientLinkSettings::new("drives");
                settings5
                    .use_method_name("drive")
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_new_method_empty_id();

                let mut settings6 = ClientLinkSettings::new("onenote");
                settings6
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![
                    settings, settings2, settings3, settings4, settings5, settings6,
                ]);
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

                let mut settings18 = ClientLinkSettings::new("drives");
                settings18
                    .use_method_name("drive")
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_new_method_empty_id();

                let mut settings19 = ClientLinkSettings::new("onenote");
                settings19
                    .with_extend_path_ident()
                    .with_extend_path_id()
                    .with_set_resource_identity();

                let mut set = BTreeSet::new();
                set.extend(vec![
                    settings, settings2, settings3, settings4, settings5, settings6, settings7,
                    settings8, settings9, settings10, settings11, settings12, settings13,
                    settings14, settings15, settings16, settings17, settings18, settings19,
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
            ResourceIdentity::Messages => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.ListMessages".to_string()),
                    vec![
                        MatchTarget::OperationMap("messages".to_string()),
                        MatchTarget::OperationId("messages.ListMessages".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.GetMessages".to_string()),
                    vec![
                        MatchTarget::OperationMap("messages".to_string()),
                        MatchTarget::OperationId("messages.GetMessages".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.CreateMessages".to_string()),
                    vec![
                        MatchTarget::OperationMap("messages".to_string()),
                        MatchTarget::OperationId("messages.CreateMessages".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.UpdateMessages".to_string()),
                    vec![
                        MatchTarget::OperationMap("messages".to_string()),
                        MatchTarget::OperationId("messages.UpdateMessages".to_string()),
                    ],
                );
            },
            ResourceIdentity::MailFolders => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.ListMailFolders".to_string()),
                    vec![
                        MatchTarget::OperationMap("mailFolders".to_string()),
                        MatchTarget::OperationId("mailFolders.ListMailFolders".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.GetMailFolders".to_string()),
                    vec![
                        MatchTarget::OperationMap("mailFolders".to_string()),
                        MatchTarget::OperationId("mailFolders.GetMailFolders".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.CreateMailFolders".to_string()),
                    vec![
                        MatchTarget::OperationMap("mailFolders".to_string()),
                        MatchTarget::OperationId("mailFolders.CreateMailFolders".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.UpdateMailFolders".to_string()),
                    vec![
                        MatchTarget::OperationMap("mailFolders".to_string()),
                        MatchTarget::OperationId("mailFolders.UpdateMailFolders".to_string()),
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
            ResourceIdentity::Notebooks => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.ListNotebooks".to_string()),
                    vec![
                        MatchTarget::OperationMap("notebooks".to_string()),
                        MatchTarget::OperationId("notebooks.ListNotebooks".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.GetNotebooks".to_string()),
                    vec![
                        MatchTarget::OperationMap("notebooks".to_string()),
                        MatchTarget::OperationId("notebooks.GetNotebooks".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.CreateNotebooks".to_string()),
                    vec![
                        MatchTarget::OperationMap("notebooks".to_string()),
                        MatchTarget::OperationId("notebooks.CreateNotebooks".to_string()),
                    ],
                );
            },
            ResourceIdentity::Onenote => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.GetOnenote".to_string()),
                    vec![
                        MatchTarget::OperationMap("onenote".to_string()),
                        MatchTarget::OperationId("onenote.GetOnenote".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.UpdateOnenote".to_string()),
                    vec![
                        MatchTarget::OperationMap("onenote".to_string()),
                        MatchTarget::OperationId("onenote.UpdateOnenote".to_string()),
                    ],
                );
            },
            ResourceIdentity::SectionGroups => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.CreateSectionGroups".to_string()),
                    vec![
                        MatchTarget::OperationMap("sectionGroups".to_string()),
                        MatchTarget::OperationId("sectionGroups.CreateSectionGroups".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.GetSectionGroups".to_string()),
                    vec![
                        MatchTarget::OperationMap("sectionGroups".to_string()),
                        MatchTarget::OperationId("sectionGroups.GetSectionGroup".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.ListSectionGroups".to_string()),
                    vec![
                        MatchTarget::OperationMap("sectionGroups".to_string()),
                        MatchTarget::OperationId("sectionGroups.ListSectionGroups".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.UpdateSectionGroups".to_string()),
                    vec![
                        MatchTarget::OperationMap("sectionGroups".to_string()),
                        MatchTarget::OperationId("sectionGroups.UpdateSectionGroup".to_string()),
                    ],
                );
            },
            ResourceIdentity::Pages => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.GetPages".to_string()),
                    vec![
                        MatchTarget::OperationMap("pages".to_string()),
                        MatchTarget::OperationId("pages.GetPages".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.UpdatePages".to_string()),
                    vec![
                        MatchTarget::OperationMap("pages".to_string()),
                        MatchTarget::OperationId("pages.UpdatePages".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.ListPages".to_string()),
                    vec![
                        MatchTarget::OperationMap("pages".to_string()),
                        MatchTarget::OperationId("pages.ListPages".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.CreatePages".to_string()),
                    vec![
                        MatchTarget::OperationMap("pages".to_string()),
                        MatchTarget::OperationId("pages.CreatePages".to_string()),
                    ],
                );
            },
            ResourceIdentity::Sections => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.ListSections".to_string()),
                    vec![
                        MatchTarget::OperationMap("sections".to_string()),
                        MatchTarget::OperationId("sections.ListSections".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.CreateSections".to_string()),
                    vec![
                        MatchTarget::OperationMap("sections".to_string()),
                        MatchTarget::OperationId("sections.CreateSections".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.UpdateSections".to_string()),
                    vec![
                        MatchTarget::OperationMap("sections".to_string()),
                        MatchTarget::OperationId("sections.UpdateSections".to_string()),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.GetSections".to_string()),
                    vec![
                        MatchTarget::OperationMap("sections".to_string()),
                        MatchTarget::OperationId("sections.GetSections".to_string()),
                    ],
                );
            },
            ResourceIdentity::ParentSection => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.pages.GetParentSection".to_string()),
                    vec![
                        MatchTarget::OperationMap("me.onenote.pages.parentSection".to_string()),
                        MatchTarget::OperationId(
                            "me.onenote.pages.parentSection.GetParentNotebook".to_string(),
                        ),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.pages.UpdateParentSection".to_string()),
                    vec![
                        MatchTarget::OperationMap("me.onenote.pages.parentSection".to_string()),
                        MatchTarget::OperationId(
                            "me.onenote.pages.parentSection.UpdateParentNotebook".to_string(),
                        ),
                    ],
                );
            },
            ResourceIdentity::ParentNotebook => {
                modify_target.map.insert(
                    MatchTarget::OperationId("me.onenote.sections.GetParentNotebook".to_string()),
                    vec![
                        MatchTarget::OperationMap("me.onenote.sections.parentNotebook".to_string()),
                        MatchTarget::OperationId(
                            "me.onenote.sections.parentNotebook.GetParentNotebook".to_string(),
                        ),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId(
                        "me.onenote.sections.UpdateParentNotebook".to_string(),
                    ),
                    vec![
                        MatchTarget::OperationMap("me.onenote.sections.parentNotebook".to_string()),
                        MatchTarget::OperationId(
                            "me.onenote.sections.parentNotebook.UpdateParentNotebook".to_string(),
                        ),
                    ],
                );
            },
            ResourceIdentity::ParentSectionGroup => {
                modify_target.map.insert(
                    MatchTarget::OperationId(
                        "me.onenote.sections.GetParentSectionGroup".to_string(),
                    ),
                    vec![
                        MatchTarget::OperationMap(
                            "me.onenote.sections.parentSectionGroup".to_string(),
                        ),
                        MatchTarget::OperationId(
                            "me.onenote.sections.parentSectionGroup.GetParentSectionGroup"
                                .to_string(),
                        ),
                    ],
                );
                modify_target.map.insert(
                    MatchTarget::OperationId(
                        "me.onenote.sections.UpdateParentSectionGroup".to_string(),
                    ),
                    vec![
                        MatchTarget::OperationMap(
                            "me.onenote.sections.parentSectionGroup".to_string(),
                        ),
                        MatchTarget::OperationId(
                            "me.onenote.sections.parentSectionGroup.UpdateParentSectionGroup"
                                .to_string(),
                        ),
                    ],
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
