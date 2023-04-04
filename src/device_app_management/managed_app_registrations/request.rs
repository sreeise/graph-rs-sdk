// GENERATED CODE

use crate::api_default_imports::*;
use crate::device_app_management::*;

resource_api_client!(
    ManagedAppRegistrationsApiClient,
    ManagedAppRegistrationsIdApiClient,
    ResourceIdentity::ManagedAppRegistrations
);

impl ManagedAppRegistrationsApiClient {
    api_client_link!(
        applied_policies,
        ManagedAppRegistrationsAppliedPoliciesApiClient
    );
    api_client_link_id!(
        intended_policies_id,
        ManagedAppRegistrationsIntendedPoliciesIdApiClient
    );
    api_client_link!(
        intended_policies,
        ManagedAppRegistrationsIntendedPoliciesApiClient
    );
    api_client_link_id!(
        applied_policies_id,
        ManagedAppRegistrationsAppliedPoliciesIdApiClient
    );

    post!(
        doc: "Create new navigation property to managedAppRegistrations for deviceAppManagement",
        name: create_managed_app_registrations,
        path: "/managedAppRegistrations",
        body: true
    );
    get!(
        doc: "Get managedAppRegistrations from deviceAppManagement",
        name: list_managed_app_registrations,
        path: "/managedAppRegistrations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_managed_app_registrations_count,
        path: "/managedAppRegistrations/$count"
    );
    get!(
        doc: "Invoke function getUserIdsWithFlaggedAppRegistration",
        name: get_user_ids_with_flagged_app_registration,
        path: "/managedAppRegistrations/getUserIdsWithFlaggedAppRegistration()"
    );
}

impl ManagedAppRegistrationsIdApiClient {
    api_client_link_id!(
        intended_policies_id,
        ManagedAppRegistrationsIntendedPoliciesIdApiClient
    );
    api_client_link!(
        applied_policies,
        ManagedAppRegistrationsAppliedPoliciesApiClient
    );
    api_client_link_id!(
        applied_policies_id,
        ManagedAppRegistrationsAppliedPoliciesIdApiClient
    );
    api_client_link!(
        intended_policies,
        ManagedAppRegistrationsIntendedPoliciesApiClient
    );

    delete!(
        doc: "Delete navigation property managedAppRegistrations for deviceAppManagement",
        name: delete_managed_app_registrations,
        path: "/managedAppRegistrations/{{RID}}"
    );
    get!(
        doc: "Get managedAppRegistrations from deviceAppManagement",
        name: get_managed_app_registrations,
        path: "/managedAppRegistrations/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property managedAppRegistrations in deviceAppManagement",
        name: update_managed_app_registrations,
        path: "/managedAppRegistrations/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to operations for deviceAppManagement",
        name: create_operations,
        path: "/managedAppRegistrations/{{RID}}/operations",
        body: true
    );
    get!(
        doc: "Get operations from deviceAppManagement",
        name: list_operations,
        path: "/managedAppRegistrations/{{RID}}/operations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_operations_count,
        path: "/managedAppRegistrations/{{RID}}/operations/$count"
    );
    delete!(
        doc: "Delete navigation property operations for deviceAppManagement",
        name: delete_operations,
        path: "/managedAppRegistrations/{{RID}}/operations/{{id}}",
        params: managed_app_operation_id
    );
    get!(
        doc: "Get operations from deviceAppManagement",
        name: get_operations,
        path: "/managedAppRegistrations/{{RID}}/operations/{{id}}",
        params: managed_app_operation_id
    );
    patch!(
        doc: "Update the navigation property operations in deviceAppManagement",
        name: update_operations,
        path: "/managedAppRegistrations/{{RID}}/operations/{{id}}",
        body: true,
        params: managed_app_operation_id
    );
}
