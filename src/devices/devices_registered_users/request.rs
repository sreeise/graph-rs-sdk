// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    DevicesRegisteredUsersApiClient,
    DevicesRegisteredUsersIdApiClient,
    ResourceIdentity::DevicesRegisteredUsers
);

impl DevicesRegisteredUsersApiClient {
    get!(
        doc: "List registeredUsers",
        name: list_registered_users,
        path: "/registeredUsers"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_devices_registered_owners_count,
        path: "/registeredUsers/$count"
    );
    post!(
        doc: "Create registeredUser",
        name: create_ref_registered_users,
        path: "/registeredUsers/$ref",
        body: true
    );
    delete!(
        doc: "Delete registeredUser",
        name: delete_ref_registered_users,
        path: "/registeredUsers/$ref"
    );
    get!(
        doc: "List registeredUsers",
        name: list_ref_registered_users,
        path: "/registeredUsers/$ref"
    );
    get!(
        doc: "Get the items of type microsoft.graph.appRoleAssignment in the microsoft.graph.directoryObject collection",
        name: as_app_role_assignment,
        path: "/registeredUsers/graph.appRoleAssignment"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_app_role_assignment_count,
        path: "/registeredUsers/graph.appRoleAssignment/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.endpoint in the microsoft.graph.directoryObject collection",
        name: as_endpoint,
        path: "/registeredUsers/graph.endpoint"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_endpoint_count,
        path: "/registeredUsers/graph.endpoint/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.servicePrincipal in the microsoft.graph.directoryObject collection",
        name: as_service_principal,
        path: "/registeredUsers/graph.servicePrincipal"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_service_principal_count,
        path: "/registeredUsers/graph.servicePrincipal/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.user in the microsoft.graph.directoryObject collection",
        name: as_user,
        path: "/registeredUsers/graph.user"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_count,
        path: "/registeredUsers/graph.user/$count"
    );
}

impl DevicesRegisteredUsersIdApiClient {
    delete!(
        doc: "Delete registeredUser",
        name: delete_ref_directory_object,
        path: "/registeredUsers/{{RID}}/$ref"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.appRoleAssignment",
        name: as_app_role_assignment,
        path: "/registeredUsers/{{RID}}/graph.appRoleAssignment"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.endpoint",
        name: as_endpoint,
        path: "/registeredUsers/{{RID}}/graph.endpoint"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.servicePrincipal",
        name: as_service_principal,
        path: "/registeredUsers/{{RID}}/graph.servicePrincipal"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.user",
        name: as_user,
        path: "/registeredUsers/{{RID}}/graph.user"
    );
}
