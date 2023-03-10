// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    ContactFoldersApiClient,
    ContactFoldersIdApiClient,
    ResourceIdentity::ContactFolders
);

impl ContactFoldersApiClient {
    post!(
        doc: "Create ContactFolder",
        name: create_contact_folders,
        path: "/contactFolders",
        body: true
    );
    get!(
        doc: "List contactFolders",
        name: list_contact_folders,
        path: "/contactFolders"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_contact_folders_count,
        path: "/contactFolders/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/contactFolders/delta()"
    );
}

impl ContactFoldersIdApiClient {
    delete!(
        doc: "Delete navigation property contactFolders for users",
        name: delete_contact_folders,
        path: "/contactFolders/{{RID}}"
    );
    get!(
        doc: "Get contactFolders from users",
        name: get_contact_folders,
        path: "/contactFolders/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property contactFolders in users",
        name: update_contact_folders,
        path: "/contactFolders/{{RID}}",
        body: true
    );
    post!(
        doc: "Create ContactFolder",
        name: create_child_folders,
        path: "/contactFolders/{{RID}}/childFolders",
        body: true
    );
    get!(
        doc: "List childFolders",
        name: list_child_folders,
        path: "/contactFolders/{{RID}}/childFolders"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_child_folders_count,
        path: "/contactFolders/{{RID}}/childFolders/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/contactFolders/{{RID}}/childFolders/delta()"
    );
    delete!(
        doc: "Delete navigation property childFolders for users",
        name: delete_child_folders,
        path: "/contactFolders/{{RID}}/childFolders/{{id}}",
        params: contact_folder_id_1
    );
    get!(
        doc: "Get childFolders from users",
        name: get_child_folders,
        path: "/contactFolders/{{RID}}/childFolders/{{id}}",
        params: contact_folder_id_1
    );
    patch!(
        doc: "Update the navigation property childFolders in users",
        name: update_child_folders,
        path: "/contactFolders/{{RID}}/childFolders/{{id}}",
        body: true,
        params: contact_folder_id_1
    );
}
