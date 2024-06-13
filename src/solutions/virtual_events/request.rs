// GENERATED CODE

use crate::api_default_imports::*;
use crate::solutions::*;

api_client!(VirtualEventsApiClient, ResourceIdentity::VirtualEvents);

impl VirtualEventsApiClient {
    api_client_link_id!(webinar, VirtualEventsWebinarsIdApiClient);
    api_client_link_id!(event, VirtualEventsEventsIdApiClient);
    api_client_link!(events, VirtualEventsEventsApiClient);
    api_client_link!(webinars, VirtualEventsWebinarsApiClient);

    delete!(
        doc: "Delete navigation property virtualEvents for solutions",
        name: delete_virtual_events,
        path: "/virtualEvents"
    );
    get!(
        doc: "Get virtualEvents from solutions",
        name: get_virtual_events,
        path: "/virtualEvents"
    );
    patch!(
        doc: "Update the navigation property virtualEvents in solutions",
        name: update_virtual_events,
        path: "/virtualEvents",
        body: true
    );
}
