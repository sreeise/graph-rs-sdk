use graph_core::resource::ResourceIdentity;
use crate::generator::Generator;
use crate::parser::filter::Filter;

static API_V1_METADATA: &str = "/home/sreeise/src/msgraph-metadata/openapi/v1.0/openapi.yaml";

pub struct GeneratorBuilders;

impl GeneratorBuilders {
    pub fn get_generator(resource_identity: ResourceIdentity) -> Option<Generator> {

        match resource_identity {
            ResourceIdentity::Calendar => {
                Some(
                    Generator::parse_secondary(
                        API_V1_METADATA,
                        Filter::PathStartsWith("/users/{user-id}/calendar"),
                        "calendar",
                        Some(&["calendar"])
                ))
            },
            ResourceIdentity::Events => {
                Some(
                    Generator::parse_secondary(
                        API_V1_METADATA,
                        Filter::PathStartsWith("/users"),
                        "events",
                        Some(&["events"]),
                    )
                )
            },
            ResourceIdentity::Me => {
                Some(Generator::parse(API_V1_METADATA, Some(&["me"])))
            }
            ResourceIdentity::Users => {
                Some(Generator::parse(API_V1_METADATA, Some(&["users"])))
            },
            _ => None
        }
    }
}
