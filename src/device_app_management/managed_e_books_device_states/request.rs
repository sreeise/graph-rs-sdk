// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    ManagedEBooksDeviceStatesApiClient,
    ManagedEBooksDeviceStatesIdApiClient,
    ResourceIdentity::ManagedEBooksDeviceStates
);

impl ManagedEBooksDeviceStatesApiClient {
    post!(
        doc: "Create new navigation property to deviceStates for deviceAppManagement",
        name: create_device_states,
        path: "/deviceStates",
        body: true
    );
    get!(
        doc: "Get deviceStates from deviceAppManagement",
        name: list_device_states,
        path: "/deviceStates"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_states_count,
        path: "/deviceStates/$count"
    );
}

impl ManagedEBooksDeviceStatesIdApiClient {
    delete!(
        doc: "Delete navigation property deviceStates for deviceAppManagement",
        name: delete_device_states,
        path: "/deviceStates/{{RID}}"
    );
    get!(
        doc: "Get deviceStates from deviceAppManagement",
        name: get_device_states,
        path: "/deviceStates/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property deviceStates in deviceAppManagement",
        name: update_device_states,
        path: "/deviceStates/{{RID}}",
        body: true
    );
}
