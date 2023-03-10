// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    ContactsApiClient,
    ContactsIdApiClient,
    ResourceIdentity::Contacts
);

impl ContactsApiClient {
    post!(
        doc: "Create contact",
        name: create_contacts,
        path: "/contacts",
        body: true
    );
    get!(
        doc: "List contacts",
        name: list_contacts,
        path: "/contacts"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_contacts_count,
        path: "/contacts/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/contacts/delta()"
    );
}

impl ContactsIdApiClient {
    delete!(
        doc: "Delete navigation property contacts for users",
        name: delete_contacts,
        path: "/contacts/{{RID}}"
    );
    get!(
        doc: "Get contacts from users",
        name: get_contacts,
        path: "/contacts/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property contacts in users",
        name: update_contacts,
        path: "/contacts/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to extensions for users",
        name: create_extensions,
        path: "/contacts/{{RID}}/extensions",
        body: true
    );
    get!(
        doc: "Get extensions from users",
        name: list_extensions,
        path: "/contacts/{{RID}}/extensions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_extensions_count,
        path: "/contacts/{{RID}}/extensions/$count"
    );
    delete!(
        doc: "Delete navigation property extensions for users",
        name: delete_extensions,
        path: "/contacts/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    get!(
        doc: "Get extensions from users",
        name: get_extensions,
        path: "/contacts/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    patch!(
        doc: "Update the navigation property extensions in users",
        name: update_extensions,
        path: "/contacts/{{RID}}/extensions/{{id}}",
        body: true,
        params: extension_id
    );
    get!(
        doc: "Get photo from users",
        name: get_photo,
        path: "/contacts/{{RID}}/photo"
    );
    patch!(
        doc: "Update the navigation property photo in users",
        name: update_photo,
        path: "/contacts/{{RID}}/photo",
        body: true
    );
    get!(
        doc: "Get media content for the navigation property photo from users",
        name: get_photo_content,
        path: "/contacts/{{RID}}/photo/$value"
    );
    put!(
        doc: "Update media content for the navigation property photo in users",
        name: update_photo_content,
        path: "/contacts/{{RID}}/photo/$value",
        body: true
    );
}
