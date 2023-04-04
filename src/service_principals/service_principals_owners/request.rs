// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    ServicePrincipalsOwnersApiClient,
    ServicePrincipalsOwnersIdApiClient,
    ResourceIdentity::ServicePrincipalsOwners
);

impl ServicePrincipalsOwnersApiClient {
    get!(
        doc: "Get owners from servicePrincipals",
        name: list_owners,
        path: "/owners"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_owners_count,
        path: "/owners/$count"
    );
    post!(
        doc: "Create new navigation property ref to owners for servicePrincipals",
        name: create_ref_owners,
        path: "/owners/$ref",
        body: true
    );
    get!(
        doc: "Get ref of owners from servicePrincipals",
        name: list_ref_owners,
        path: "/owners/$ref"
    );
    get!(
        doc: "Get the items of type microsoft.graph.appRoleAssignment in the microsoft.graph.directoryObject collection",
        name: get_app_role_assignments,
        path: "/owners/graph.appRoleAssignment"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_app_role_assignments_count,
        path: "/owners/graph.appRoleAssignment/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.endpoint in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_endpoint_type,
        path: "/owners/graph.endpoint"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_endpoint_count,
        path: "/owners/graph.endpoint/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.servicePrincipal in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_service_principal_type,
        path: "/owners/graph.servicePrincipal"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_service_principal_count,
        path: "/owners/graph.servicePrincipal/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.user in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_user_type,
        path: "/owners/graph.user"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_count,
        path: "/owners/graph.user/$count"
    );
}

impl ServicePrincipalsOwnersIdApiClient {
    delete!(
        doc: "Delete ref of navigation property owners for servicePrincipals",
        name: delete_ref_owners,
        path: "/owners/{{RID}}/$ref"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.appRoleAssignment",
        name: get_app_role_assignments,
        path: "/owners/{{RID}}/graph.appRoleAssignment"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.endpoint",
        name: get_directory_object_item_as_endpoint_type,
        path: "/owners/{{RID}}/graph.endpoint"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.servicePrincipal",
        name: get_directory_object_item_as_service_principal_type,
        path: "/owners/{{RID}}/graph.servicePrincipal"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.user",
        name: get_directory_object_item_as_user_type,
        path: "/owners/{{RID}}/graph.user"
    );
}
