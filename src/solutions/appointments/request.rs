// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    AppointmentsApiClient,
    AppointmentsIdApiClient,
    ResourceIdentity::Appointments
);

impl AppointmentsApiClient {
    post!(
        doc: "Create bookingAppointment",
        name: create_appointments,
        path: "/appointments",
        body: true
    );
    get!(
        doc: "List appointments",
        name: list_appointments,
        path: "/appointments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_appointments_count,
        path: "/appointments/$count"
    );
}

impl AppointmentsIdApiClient {
    delete!(
        doc: "Delete bookingAppointment",
        name: delete_appointments,
        path: "/appointments/{{RID}}"
    );
    get!(
        doc: "Get bookingAppointment",
        name: get_appointments,
        path: "/appointments/{{RID}}"
    );
    patch!(
        doc: "Update bookingAppointment",
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
