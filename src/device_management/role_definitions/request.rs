// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    RoleDefinitionsApiClient,
    RoleDefinitionsIdApiClient,
    ResourceIdentity::RoleDefinitions
);

impl RoleDefinitionsApiClient {
    post!(
        doc: "Create new navigation property to roleDefinitions for deviceManagement",
        name: create_role_definitions,
        path: "/roleDefinitions",
        body: true
    );
    get!(
        doc: "Get roleDefinitions from deviceManagement",
        name: list_role_definitions,
        path: "/roleDefinitions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_role_definitions_count,
        path: "/roleDefinitions/$count"
    );
}

impl RoleDefinitionsIdApiClient {
    delete!(
        doc: "Delete navigation property roleDefinitions for deviceManagement",
        name: delete_role_definitions,
        path: "/roleDefinitions/{{RID}}"
    );
    get!(
        doc: "Get roleDefinitions from deviceManagement",
        name: get_role_definitions,
        path: "/roleDefinitions/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property roleDefinitions in deviceManagement",
        name: update_role_definitions,
        path: "/roleDefinitions/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to roleAssignments for deviceManagement",
        name: create_role_assignments,
        path: "/roleDefinitions/{{RID}}/roleAssignments",
        body: true
    );
    get!(
        doc: "Get roleAssignments from deviceManagement",
        name: list_role_assignments,
        path: "/roleDefinitions/{{RID}}/roleAssignments"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/roleDefinitions/{{RID}}/roleAssignments/$count"
    );
    delete!(
        doc: "Delete navigation property roleAssignments for deviceManagement",
        name: delete_role_assignments,
        path: "/roleDefinitions/{{RID}}/roleAssignments/{{id}}",
        params: role_assignment_id
    );
    get!(
        doc: "Get roleAssignments from deviceManagement",
        name: get_role_assignments,
        path: "/roleDefinitions/{{RID}}/roleAssignments/{{id}}",
        params: role_assignment_id
    );
    patch!(
        doc: "Update the navigation property roleAssignments in deviceManagement",
        name: update_role_assignments,
        path: "/roleDefinitions/{{RID}}/roleAssignments/{{id}}",
        body: true,
        params: role_assignment_id
    );
    get!(
        doc: "Get roleDefinition from deviceManagement",
        name: get_role_definition,
        path: "/roleDefinitions/{{RID}}/roleAssignments/{{id}}/roleDefinition",
        params: role_assignment_id
    );
}
