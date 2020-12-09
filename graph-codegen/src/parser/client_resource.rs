use crate::parser::filter::Filter;
use graph_core::resource::ResourceIdentity;
use std::convert::TryFrom;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ClientResource<'a> {
    Main {
        modifier: String,
    },
    Secondary {
        start_filter: Filter<'a>,
        modifier: String,
    },
}

impl Default for ClientResource<'_> {
    fn default() -> Self {
        ClientResource::Main {
            modifier: Default::default(),
        }
    }
}

impl TryFrom<ResourceIdentity> for ClientResource<'_> {
    type Error = ();

    fn try_from(resource_identity: ResourceIdentity) -> Result<Self, Self::Error> {
        match resource_identity {
            ResourceIdentity::Activities => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/activities"),
                modifier: "activities".to_string(),
            }),
            ResourceIdentity::AppCatalogs => Ok(ClientResource::Main {
                modifier: "appCatalogs".to_string(),
            }),
            ResourceIdentity::Applications => Ok(ClientResource::Main {
                modifier: "applications".to_string(),
            }),
            ResourceIdentity::AuditLogs => Ok(ClientResource::Main {
                modifier: "auditLogs".to_string(),
            }),
            ResourceIdentity::Attachments => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith(
                    "/groups/{group-id}/calendar/events/{event-id}/attachments",
                ),
                modifier: "attachments".to_string(),
            }),
            ResourceIdentity::Buckets => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/planner/buckets"),
                modifier: "buckets".to_string(),
            }),
            ResourceIdentity::Calendar | ResourceIdentity::Calendars => {
                Ok(ClientResource::Secondary {
                    start_filter: Filter::PathStartsWith("/users/{user-id}/calendar"),
                    modifier: "calendar".to_string(),
                })
            },
            ResourceIdentity::CalendarGroups => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/users"),
                modifier: "calendarGroups".to_string(),
            }),
            ResourceIdentity::CalendarView | ResourceIdentity::CalendarViews => {
                Ok(ClientResource::Secondary {
                    start_filter: Filter::PathStartsWith("/me/calendarView"),
                    modifier: "calendarView".to_string(),
                })
            },
            ResourceIdentity::CallRecords => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/communications/callRecords"),
                modifier: "callRecords".to_string(),
            }),
            ResourceIdentity::CertificateBasedAuthConfiguration => Ok(ClientResource::Main {
                modifier: "certificateBasedAuthConfiguration".to_string(),
            }),
            ResourceIdentity::ContactFolders => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/contactFolders"),
                modifier: "contactFolders".to_string(),
            }),
            ResourceIdentity::Contacts => Ok(ClientResource::Main {
                modifier: "contacts".to_string(),
            }),
            ResourceIdentity::ContentTypes => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/sites/{site-id}/contentTypes"),
                modifier: "contentTypes".to_string(),
            }),
            ResourceIdentity::Conversations => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/groups/{group-id}/conversations"),
                modifier: "conversations".to_string(),
            }),
            ResourceIdentity::Calls => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/communications/calls"),
                modifier: "calls".to_string(),
            }),
            ResourceIdentity::Communications => Ok(ClientResource::Main {
                modifier: "communications".to_string(),
            }),
            ResourceIdentity::DataPolicyOperations => Ok(ClientResource::Main {
                modifier: "dataPolicyOperations".to_string(),
            }),
            ResourceIdentity::Drive | ResourceIdentity::Drives => Ok(ClientResource::Main {
                modifier: "drive".to_string(),
            }),
            ResourceIdentity::Domains => Ok(ClientResource::Main {
                modifier: "domains".to_string(),
            }),
            ResourceIdentity::Education => Ok(ClientResource::Main {
                modifier: ResourceIdentity::Education.to_string(),
            }),
            ResourceIdentity::Events => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/users"),
                modifier: "events".to_string(),
            }),
            ResourceIdentity::Groups => Ok(ClientResource::Main {
                modifier: "groups".to_string(),
            }),
            ResourceIdentity::InferenceClassification => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/inferenceClassification"),
                modifier: "inferenceClassification".to_string(),
            }),
            ResourceIdentity::Insights => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/insights"),
                modifier: "insights".to_string(),
            }),
            ResourceIdentity::Instances => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/calendarView/{event-id}/instances"),
                modifier: "instances".to_string(),
            }),
            ResourceIdentity::Items => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/sites/{site-id}/lists/{list-id}/items"),
                modifier: "items".to_string(),
            }),
            ResourceIdentity::Lists => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/sites"),
                modifier: "lists".to_string(),
            }),
            ResourceIdentity::ManagedDevices => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/managedDevices"),
                modifier: "managedDevices".to_string(),
            }),
            ResourceIdentity::MailFolders => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/mailFolders"),
                modifier: "mailFolders".to_string(),
            }),
            ResourceIdentity::Messages => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/messages"),
                modifier: "messages".to_string(),
            }),
            ResourceIdentity::Me => Ok(ClientResource::Main {
                modifier: "me".to_string(),
            }),
            ResourceIdentity::Outlook => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/outlook"),
                modifier: "outlook".to_string(),
            }),
            ResourceIdentity::Onenote => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/onenote"),
                modifier: "onenote".to_string(),
            }),
            ResourceIdentity::Notebooks => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/onenote/notebook"),
                modifier: "notebooks".to_string(),
            }),
            ResourceIdentity::Sections => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/onenote/sections"),
                modifier: "sections".to_string(),
            }),
            ResourceIdentity::SectionGroups => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/onenote/sectionGroups"),
                modifier: "sectionGroups".to_string(),
            }),
            ResourceIdentity::Pages => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/onenote/pages"),
                modifier: "pages".to_string(),
            }),
            ResourceIdentity::ParentNotebook => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith(
                    "/me/onenote/sections/{onenoteSection-id}/parentNotebook",
                ),
                modifier: "parentNotebook".to_string(),
            }),
            ResourceIdentity::ParentSectionGroup => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith(
                    "/me/onenote/sections/{onenoteSection-id}/parentSectionGroup",
                ),
                modifier: "parentSectionGroup".to_string(),
            }),
            ResourceIdentity::ParentSection => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith(
                    "/me/onenote/pages/{onenotePage-id}/parentSection",
                ),
                modifier: "parentSection".to_string(),
            }),
            ResourceIdentity::Planner => Ok(ClientResource::Main {
                modifier: "planner".to_string(),
            }),
            ResourceIdentity::Posts => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith(
                    "/groups/{group-id}/threads/{conversationThread-id}/posts",
                ),
                modifier: "posts".to_string(),
            }),
            ResourceIdentity::Plans => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/planner/plans"),
                modifier: "plans".to_string(),
            }),
            ResourceIdentity::Sites => Ok(ClientResource::Main {
                modifier: "sites".to_string(),
            }),
            ResourceIdentity::Sessions => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith(
                    "/communications/callRecords/{callRecord-id}/sessions",
                ),
                modifier: "sessions".to_string(),
            }),
            ResourceIdentity::Settings => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/settings"),
                modifier: "settings".to_string(),
            }),
            ResourceIdentity::Users => Ok(ClientResource::Main {
                modifier: "users".to_string(),
            }),
            ResourceIdentity::Tasks => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/planner/tasks"),
                modifier: "tasks".to_string(),
            }),
            ResourceIdentity::Threads => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/groups/{group-id}/threads"),
                modifier: "threads".to_string(),
            }),
            _ => Err(()),
        }
    }
}
