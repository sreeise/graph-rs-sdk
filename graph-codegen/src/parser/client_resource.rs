use crate::parser::filter::Filter;
use from_as::TryFrom;
use graph_core::resource::ResourceIdentity;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ClientResource<'a> {
    Main {
        modifier: String,
    },
    Secondary {
        start_filter: Filter<'a>,
        secondary_name: String,
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
                secondary_name: "activities".to_string(),
                modifier: "activities".to_string(),
            }),
            ResourceIdentity::Calendar | ResourceIdentity::Calendars => {
                Ok(ClientResource::Secondary {
                    start_filter: Filter::PathStartsWith("/users/{user-id}/calendar"),
                    secondary_name: "calendar".to_string(),
                    modifier: "calendar".to_string(),
                })
            },
            ResourceIdentity::CalendarGroups => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/users"),
                secondary_name: "calendarGroups".to_string(),
                modifier: "calendarGroups".to_string(),
            }),
            ResourceIdentity::CalendarView | ResourceIdentity::CalendarViews => {
                Ok(ClientResource::Secondary {
                    start_filter: Filter::PathStartsWith("/me/calendarView"),
                    secondary_name: "calendarView".to_string(),
                    modifier: "calendarView".to_string(),
                })
            },
            ResourceIdentity::ContactFolders => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/contactFolders"),
                secondary_name: "contactFolders".to_string(),
                modifier: "contactFolders".to_string(),
            }),
            ResourceIdentity::Contacts => Ok(ClientResource::Main {
                modifier: "contacts".to_string(),
            }),
            ResourceIdentity::ContentTypes => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/sites/{site-id}/contentTypes"),
                secondary_name: "contentTypes".to_string(),
                modifier: "contentTypes".to_string(),
            }),
            ResourceIdentity::Drive | ResourceIdentity::Drives => Ok(ClientResource::Main {
                modifier: "drive".to_string(),
            }),
            ResourceIdentity::Education => Ok(ClientResource::Main {
                modifier: ResourceIdentity::Education.to_string(),
            }),
            ResourceIdentity::Events => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/users"),
                secondary_name: "events".to_string(),
                modifier: "events".to_string(),
            }),
            ResourceIdentity::InferenceClassification => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/inferenceClassification"),
                secondary_name: "inferenceClassification".to_string(),
                modifier: "inferenceClassification".to_string(),
            }),
            ResourceIdentity::Insights => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/insights"),
                secondary_name: "insights".to_string(),
                modifier: "insights".to_string(),
            }),
            ResourceIdentity::Instances => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/calendarView/{event-id}/instances"),
                secondary_name: "instances".to_string(),
                modifier: "instances".to_string(),
            }),
            ResourceIdentity::Items => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/sites/{site-id}/lists/{list-id}/items"),
                secondary_name: "items".to_string(),
                modifier: "items".to_string(),
            }),
            ResourceIdentity::Lists => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/sites"),
                secondary_name: "lists".to_string(),
                modifier: "lists".to_string(),
            }),
            ResourceIdentity::ManagedDevices => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/managedDevices"),
                secondary_name: "managedDevices".to_string(),
                modifier: "managedDevices".to_string(),
            }),
            ResourceIdentity::Me => Ok(ClientResource::Main {
                modifier: "me".to_string(),
            }),
            ResourceIdentity::Outlook => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/outlook"),
                secondary_name: "outlook".to_string(),
                modifier: "outlook".to_string(),
            }),
            ResourceIdentity::Planner => Ok(ClientResource::Main {
                modifier: "planner".to_string(),
            }),
            ResourceIdentity::Sites => Ok(ClientResource::Main {
                modifier: "sites".to_string(),
            }),
            ResourceIdentity::Settings => Ok(ClientResource::Secondary {
                start_filter: Filter::PathStartsWith("/me/settings"),
                secondary_name: "settings".to_string(),
                modifier: "settings".to_string(),
            }),
            ResourceIdentity::Users => Ok(ClientResource::Main {
                modifier: "users".to_string(),
            }),
            _ => Err(()),
        }
    }
}
