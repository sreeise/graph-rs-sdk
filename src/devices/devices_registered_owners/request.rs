// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    DevicesRegisteredOwnersApiClient,
    DevicesRegisteredOwnersIdApiClient,
    ResourceIdentity::DevicesRegisteredOwners
);

impl DevicesRegisteredOwnersApiClient {
    get!(
        doc: "List registeredOwners",
        name: list_registered_owners,
        path: "/registeredOwners"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_devices_registered_owners_count,
        path: "/registeredOwners/$count"
    );
    post!(
        doc: "Create registeredOwner",
        name: create_ref_registered_owners,
        path: "/registeredOwners/$ref",
        body: true
    );
    delete!(
        doc: "Delete registeredOwner",
        name: delete_ref_registered_owners,
        path: "/registeredOwners/$ref"
    );
    get!(
        doc: "List registeredOwners",
        name: list_ref_registered_owners,
        path: "/registeredOwners/$ref"
    );
    get!(
        doc: "Get the items of type microsoft.graph.appRoleAssignment in the microsoft.graph.directoryObject collection",
        name: as_app_role_assignment,
        path: "/registeredOwners/graph.appRoleAssignment"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_app_role_assignment_count,
        path: "/registeredOwners/graph.appRoleAssignment/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.endpoint in the microsoft.graph.directoryObject collection",
        name: as_endpoint,
        path: "/registeredOwners/graph.endpoint"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_endpoint_count,
        path: "/registeredOwners/graph.endpoint/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.servicePrincipal in the microsoft.graph.directoryObject collection",
        name: as_service_principal,
        path: "/registeredOwners/graph.servicePrincipal"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_service_principal_count,
        path: "/registeredOwners/graph.servicePrincipal/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.user in the microsoft.graph.directoryObject collection",
        name: as_user,
        path: "/registeredOwners/graph.user"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_count,
        path: "/registeredOwners/graph.user/$count"
    );
}

impl DevicesRegisteredOwnersIdApiClient {
    delete!(
        doc: "Delete registeredOwner",
        name: delete_ref_directory_object,
        path: "/registeredOwners/{{RID}}/$ref"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.appRoleAssignment",
        name: as_app_role_assignment,
        path: "/registeredOwners/{{RID}}/graph.appRoleAssignment"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.endpoint",
        name: as_endpoint,
        path: "/registeredOwners/{{RID}}/graph.endpoint"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.servicePrincipal",
        name: as_service_principal,
        path: "/registeredOwners/{{RID}}/graph.servicePrincipal"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.user",
        name: as_user,
        path: "/registeredOwners/{{RID}}/graph.user"
    );
}
