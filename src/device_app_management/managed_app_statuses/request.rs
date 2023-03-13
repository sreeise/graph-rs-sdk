// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    ManagedAppStatusesApiClient,
    ManagedAppStatusesIdApiClient,
    ResourceIdentity::ManagedAppStatuses
);

impl ManagedAppStatusesApiClient {
    post!(
        doc: "Create new navigation property to managedAppStatuses for deviceAppManagement",
        name: create_managed_app_statuses,
        path: "/managedAppStatuses",
        body: true
    );
    get!(
        doc: "Get managedAppStatuses from deviceAppManagement",
        name: list_managed_app_statuses,
        path: "/managedAppStatuses"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_managed_app_statuses_count,
        path: "/managedAppStatuses/$count"
    );
}

impl ManagedAppStatusesIdApiClient {
    delete!(
        doc: "Delete navigation property managedAppStatuses for deviceAppManagement",
        name: delete_managed_app_statuses,
        path: "/managedAppStatuses/{{RID}}"
    );
    get!(
        doc: "Get managedAppStatuses from deviceAppManagement",
        name: get_managed_app_statuses,
        path: "/managedAppStatuses/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property managedAppStatuses in deviceAppManagement",
        name: update_managed_app_statuses,
        path: "/managedAppStatuses/{{RID}}",
        body: true
    );
}
