// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    ManagedAppRegistrationsApiClient,
    ManagedAppRegistrationsIdApiClient,
    ResourceIdentity::ManagedAppRegistrations
);

impl ManagedAppRegistrationsApiClient {
    get!(
        doc: "Get managedAppRegistrations from users",
        name: list_managed_app_registrations,
        path: "/managedAppRegistrations"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/managedAppRegistrations/$count"
    );
}

impl ManagedAppRegistrationsIdApiClient {
    get!(
        doc: "Get managedAppRegistrations from users",
        name: get_managed_app_registrations,
        path: "/managedAppRegistrations/{{RID}}"
    );
}
