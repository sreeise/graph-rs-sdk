// GENERATED CODE

use crate::api_default_imports::*;
use crate::solutions::*;

api_client!(
    VirtualEventsWebinarsApiClient,
    VirtualEventsWebinarsIdApiClient,
    ResourceIdentity::VirtualEventsWebinars
);

impl VirtualEventsWebinarsApiClient {
    post!(
        doc: "Create new navigation property to webinars for solutions",
        name: create_webinars,
        path: "/webinars",
        body: true
    );
    get!(
        doc: "List webinars",
        name: list_webinars,
        path: "/webinars"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_webinars_count,
        path: "/webinars/$count"
    );
    get!(
        doc: "Invoke function getByUserIdAndRole",
        name: get_by_user_id_and_role,
        path: "/webinars/getByUserIdAndRole(userId='{{id}}',role='{{id2}}')",
        params: user_id, role
    );
    get!(
        doc: "Invoke function getByUserRole",
        name: get_by_user_role,
        path: "/webinars/getByUserRole(role='{{id}}')",
        params: role
    );
}

impl VirtualEventsWebinarsIdApiClient {
    api_client_link!(sessions, VirtualEventsSessionsApiClient);
    api_client_link_id!(session, VirtualEventsSessionsIdApiClient);

    delete!(
        doc: "Delete navigation property webinars for solutions",
        name: delete_webinars,
        path: "/webinars/{{RID}}"
    );
    get!(
        doc: "Get virtualEventWebinar",
        name: get_webinars,
        path: "/webinars/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property webinars in solutions",
        name: update_webinars,
        path: "/webinars/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to registrations for solutions",
        name: create_registrations,
        path: "/webinars/{{RID}}/registrations",
        body: true
    );
    get!(
        doc: "List virtualEventRegistrations",
        name: list_registrations,
        path: "/webinars/{{RID}}/registrations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_registrations_count,
        path: "/webinars/{{RID}}/registrations/$count"
    );
    delete!(
        doc: "Delete navigation property registrations for solutions",
        name: delete_registrations,
        path: "/webinars/{{RID}}/registrations/{{id}}",
        params: virtual_event_registration_id
    );
    get!(
        doc: "Get virtualEventRegistration",
        name: get_registrations,
        path: "/webinars/{{RID}}/registrations/{{id}}",
        params: virtual_event_registration_id
    );
    patch!(
        doc: "Update the navigation property registrations in solutions",
        name: update_registrations,
        path: "/webinars/{{RID}}/registrations/{{id}}",
        body: true,
        params: virtual_event_registration_id
    );
}
