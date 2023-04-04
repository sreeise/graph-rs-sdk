// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    DirectoryObjectsApiClient,
    DirectoryObjectsIdApiClient,
    ResourceIdentity::DirectoryObjects
);

impl DirectoryObjectsApiClient {
    post!(
        doc: "Add new entity to directoryObjects",
        name: create_directory_object,
        path: "/directoryObjects",
        body: true
    );
    get!(
        doc: "Get entities from directoryObjects",
        name: list_directory_object,
        path: "/directoryObjects"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_directory_objects_count,
        path: "/directoryObjects/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/directoryObjects/delta()"
    );
    post!(
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        path: "/directoryObjects/getAvailableExtensionProperties",
        body: true
    );
    post!(
        doc: "Invoke action getByIds",
        name: get_by_ids,
        path: "/directoryObjects/getByIds",
        body: true
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/directoryObjects/validateProperties",
        body: true
    );
}

impl DirectoryObjectsIdApiClient {
    delete!(
        doc: "Delete directoryObject",
        name: delete_directory_object,
        path: "/directoryObjects/{{RID}}"
    );
    get!(
        doc: "Get directoryObject",
        name: get_directory_object,
        path: "/directoryObjects/{{RID}}"
    );
    patch!(
        doc: "Update entity in directoryObjects",
        name: update_directory_object,
        path: "/directoryObjects/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        path: "/directoryObjects/{{RID}}/checkMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        path: "/directoryObjects/{{RID}}/checkMemberObjects",
        body: true
    );
    post!(
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        path: "/directoryObjects/{{RID}}/getMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        path: "/directoryObjects/{{RID}}/getMemberObjects",
        body: true
    );
    post!(
        doc: "Invoke action restore",
        name: restore,
        path: "/directoryObjects/{{RID}}/restore"
    );
}
