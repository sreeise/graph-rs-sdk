// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    ManagedEBooksUserStateSummaryApiClient,
    ManagedEBooksUserStateSummaryIdApiClient,
    ResourceIdentity::ManagedEBooksUserStateSummary
);

impl ManagedEBooksUserStateSummaryApiClient {
    post!(
        doc: "Create new navigation property to userStateSummary for deviceAppManagement",
        name: create_user_state_summary,
        path: "/userStateSummary",
        body: true
    );
    get!(
        doc: "Get userStateSummary from deviceAppManagement",
        name: list_user_state_summary,
        path: "/userStateSummary"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_state_summary_count,
        path: "/userStateSummary/$count"
    );
}

impl ManagedEBooksUserStateSummaryIdApiClient {
    delete!(
        doc: "Delete navigation property userStateSummary for deviceAppManagement",
        name: delete_user_state_summary,
        path: "/userStateSummary/{{RID}}"
    );
    get!(
        doc: "Get userStateSummary from deviceAppManagement",
        name: get_user_state_summary,
        path: "/userStateSummary/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property userStateSummary in deviceAppManagement",
        name: update_user_state_summary,
        path: "/userStateSummary/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to deviceStates for deviceAppManagement",
        name: create_device_states,
        path: "/userStateSummary/{{RID}}/deviceStates",
        body: true
    );
    get!(
        doc: "Get deviceStates from deviceAppManagement",
        name: list_device_states,
        path: "/userStateSummary/{{RID}}/deviceStates"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_states_count,
        path: "/userStateSummary/{{RID}}/deviceStates/$count"
    );
    delete!(
        doc: "Delete navigation property deviceStates for deviceAppManagement",
        name: delete_device_states,
        path: "/userStateSummary/{{RID}}/deviceStates/{{id}}",
        params: device_install_state_id
    );
    get!(
        doc: "Get deviceStates from deviceAppManagement",
        name: get_device_states,
        path: "/userStateSummary/{{RID}}/deviceStates/{{id}}",
        params: device_install_state_id
    );
    patch!(
        doc: "Update the navigation property deviceStates in deviceAppManagement",
        name: update_device_states,
        path: "/userStateSummary/{{RID}}/deviceStates/{{id}}",
        body: true,
        params: device_install_state_id
    );
}
