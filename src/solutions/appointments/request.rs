// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    AppointmentsApiClient,
    AppointmentsIdApiClient,
    ResourceIdentity::Appointments
);

impl AppointmentsApiClient {
    post!(
        doc: "Create new navigation property to appointments for solutions",
        name: create_appointments,
        path: "/appointments",
        body: true
    );
    get!(
        doc: "Get appointments from solutions",
        name: list_appointments,
        path: "/appointments"
    );
    get!(
        doc: "Get the number of the resource",
        name: appointments,
        path: "/appointments/$count"
    );
}

impl AppointmentsIdApiClient {
    delete!(
        doc: "Delete navigation property appointments for solutions",
        name: delete_appointments,
        path: "/appointments/{{RID}}"
    );
    get!(
        doc: "Get appointments from solutions",
        name: get_appointments,
        path: "/appointments/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property appointments in solutions",
        name: update_appointments,
        path: "/appointments/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action cancel",
        name: cancel,
        path: "/appointments/{{RID}}/cancel",
        body: true
    );
}
