use crate::generator::Generator;
use crate::parser::filter::Filter;
use graph_core::resource::ResourceIdentity;

static API_V1_METADATA: &str = "https://raw.githubusercontent.com/microsoftgraph/msgraph-metadata/master/openapi/v1.0/openapi.yaml";

pub struct GeneratorBuilders;

impl GeneratorBuilders {
    pub fn get_generator(resource_identity: ResourceIdentity) -> Option<Generator> {
        match resource_identity {
            ResourceIdentity::Calendar | ResourceIdentity::Calendars => {
                Some(Generator::from_url_secondary(
                    API_V1_METADATA,
                    Filter::PathStartsWith("/users/{user-id}/calendar"),
                    "calendar",
                    Some(&["calendar"]),
                ))
            },
            ResourceIdentity::CalendarGroups => Some(Generator::from_url_secondary(
                API_V1_METADATA,
                Filter::PathStartsWith("/users"),
                "calendarGroups",
                Some(&["calendarGroups"]),
            )),
            ResourceIdentity::CalendarView => Some(Generator::from_url_secondary(
                API_V1_METADATA,
                Filter::PathStartsWith("/me/calendarView"),
                "calendarView",
                Some(&["calendarView"]),
            )),
            ResourceIdentity::ContentTypes => Some(Generator::from_url_secondary(
                API_V1_METADATA,
                Filter::PathStartsWith("/sites/{site-id}/contentTypes"),
                "contentTypes",
                Some(&["contentTypes"]),
            )),
            ResourceIdentity::Events => Some(Generator::from_url_secondary(
                API_V1_METADATA,
                Filter::PathStartsWith("/users"),
                "events",
                Some(&["events"]),
            )),
            ResourceIdentity::Instances => Some(Generator::from_url_secondary(
                API_V1_METADATA,
                Filter::PathStartsWith("/me/calendarView/{event-id}/instances"),
                "instances",
                Some(&["instances"]),
            )),
            ResourceIdentity::Items => Some(Generator::from_url_secondary(
                API_V1_METADATA,
                Filter::PathStartsWith("/sites/{site-id}/lists/{list-id}/items"),
                "items",
                Some(&["items"]),
            )),
            ResourceIdentity::Lists => Some(Generator::from_url_secondary(
                API_V1_METADATA,
                Filter::PathStartsWith("/sites"),
                "lists",
                Some(&["lists"]),
            )),
            ResourceIdentity::Me => Some(Generator::from_url(API_V1_METADATA, Some(&["me"]))),
            ResourceIdentity::Sites => Some(Generator::from_url(API_V1_METADATA, Some(&["sites"]))),
            ResourceIdentity::Users => Some(Generator::from_url(API_V1_METADATA, Some(&["users"]))),
            _ => None,
        }
    }
}
