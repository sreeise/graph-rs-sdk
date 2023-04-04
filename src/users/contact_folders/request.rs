// GENERATED CODE

use crate::api_default_imports::*;
use crate::extended_properties::*;
use crate::users::*;

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
    api_client_link!(contacts, ContactsApiClient);
    api_client_link!(extended_properties, ExtendedPropertiesApiClient);
    api_client_link_id!(child_folder, ChildFoldersIdApiClient);
    api_client_link_id!(contact, ContactsIdApiClient);
    api_client_link!(child_folders, ChildFoldersApiClient);

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
}
