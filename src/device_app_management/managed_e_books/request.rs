// GENERATED CODE

use crate::api_default_imports::*;
use crate::device_app_management::*;

resource_api_client!(
    ManagedEBooksApiClient,
    ManagedEBooksIdApiClient,
    ResourceIdentity::ManagedEBooks
);

impl ManagedEBooksApiClient {
    api_client_link!(device_states, ManagedEBooksDeviceStatesApiClient);
    api_client_link!(user_state_summary, ManagedEBooksUserStateSummaryApiClient);
    api_client_link_id!(device_state, ManagedEBooksDeviceStatesIdApiClient);
    api_client_link_id!(
        user_state_summary_id,
        ManagedEBooksUserStateSummaryIdApiClient
    );

    post!(
        doc: "Create new navigation property to managedEBooks for deviceAppManagement",
        name: create_managed_e_books,
        path: "/managedEBooks",
        body: true
    );
    get!(
        doc: "Get managedEBooks from deviceAppManagement",
        name: list_managed_e_books,
        path: "/managedEBooks"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_managed_e_books_count,
        path: "/managedEBooks/$count"
    );
}

impl ManagedEBooksIdApiClient {
    api_client_link_id!(device_state, ManagedEBooksDeviceStatesIdApiClient);
    api_client_link!(user_state_summary, ManagedEBooksUserStateSummaryApiClient);
    api_client_link!(device_states, ManagedEBooksDeviceStatesApiClient);
    api_client_link_id!(
        user_state_summary_id,
        ManagedEBooksUserStateSummaryIdApiClient
    );

    delete!(
        doc: "Delete navigation property managedEBooks for deviceAppManagement",
        name: delete_managed_e_books,
        path: "/managedEBooks/{{RID}}"
    );
    get!(
        doc: "Get managedEBooks from deviceAppManagement",
        name: get_managed_e_books,
        path: "/managedEBooks/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property managedEBooks in deviceAppManagement",
        name: update_managed_e_books,
        path: "/managedEBooks/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action assign",
        name: assign,
        path: "/managedEBooks/{{RID}}/assign",
        body: true
    );
    post!(
        doc: "Create new navigation property to assignments for deviceAppManagement",
        name: create_assignments,
        path: "/managedEBooks/{{RID}}/assignments",
        body: true
    );
    get!(
        doc: "Get assignments from deviceAppManagement",
        name: list_assignments,
        path: "/managedEBooks/{{RID}}/assignments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_assignments_count,
        path: "/managedEBooks/{{RID}}/assignments/$count"
    );
    delete!(
        doc: "Delete navigation property assignments for deviceAppManagement",
        name: delete_assignments,
        path: "/managedEBooks/{{RID}}/assignments/{{id}}",
        params: managed_e_book_assignment_id
    );
    get!(
        doc: "Get assignments from deviceAppManagement",
        name: get_assignments,
        path: "/managedEBooks/{{RID}}/assignments/{{id}}",
        params: managed_e_book_assignment_id
    );
    patch!(
        doc: "Update the navigation property assignments in deviceAppManagement",
        name: update_assignments,
        path: "/managedEBooks/{{RID}}/assignments/{{id}}",
        body: true,
        params: managed_e_book_assignment_id
    );
    delete!(
        doc: "Delete navigation property installSummary for deviceAppManagement",
        name: delete_install_summary,
        path: "/managedEBooks/{{RID}}/installSummary"
    );
    get!(
        doc: "Get installSummary from deviceAppManagement",
        name: get_install_summary,
        path: "/managedEBooks/{{RID}}/installSummary"
    );
    patch!(
        doc: "Update the navigation property installSummary in deviceAppManagement",
        name: update_install_summary,
        path: "/managedEBooks/{{RID}}/installSummary",
        body: true
    );
}
