// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    DirectoryRoleTemplatesApiClient,
    DirectoryRoleTemplatesIdApiClient,
    ResourceIdentity::DirectoryRoleTemplates
);

impl DirectoryRoleTemplatesApiClient {
    post!(
        doc: "Add new entity to directoryRoleTemplates",
        name: create_directory_role_template,
        path: "/directoryRoleTemplates",
        body: true
    );
    get!(
        doc: "List directoryRoleTemplates",
        name: list_directory_role_template,
        path: "/directoryRoleTemplates"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_directory_role_templates_count,
        path: "/directoryRoleTemplates/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/directoryRoleTemplates/delta()"
    );
    post!(
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        path: "/directoryRoleTemplates/getAvailableExtensionProperties",
        body: true
    );
    post!(
        doc: "Invoke action getByIds",
        name: get_by_ids,
        path: "/directoryRoleTemplates/getByIds",
        body: true
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/directoryRoleTemplates/validateProperties",
        body: true
    );
}

impl DirectoryRoleTemplatesIdApiClient {
    delete!(
        doc: "Delete entity from directoryRoleTemplates",
        name: delete_directory_role_template,
        path: "/directoryRoleTemplates/{{RID}}"
    );
    get!(
        doc: "Get directoryRoleTemplate",
        name: get_directory_role_template,
        path: "/directoryRoleTemplates/{{RID}}"
    );
    patch!(
        doc: "Update entity in directoryRoleTemplates",
        name: update_directory_role_template,
        path: "/directoryRoleTemplates/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        path: "/directoryRoleTemplates/{{RID}}/checkMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        path: "/directoryRoleTemplates/{{RID}}/checkMemberObjects",
        body: true
    );
    post!(
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        path: "/directoryRoleTemplates/{{RID}}/getMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        path: "/directoryRoleTemplates/{{RID}}/getMemberObjects",
        body: true
    );
    post!(
        doc: "Invoke action restore",
        name: restore,
        path: "/directoryRoleTemplates/{{RID}}/restore"
    );
}
