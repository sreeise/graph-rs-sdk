// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    ExtensionsApiClient,
    ExtensionsIdApiClient,
    ResourceIdentity::Extensions
);

impl ExtensionsApiClient {
    post!(
        doc: "Create new navigation property to extensions for users",
        name: create_extensions,
        path: "/extensions",
        body: true
    );
    get!(
        doc: "Get extensions from users",
        name: list_extensions,
        path: "/extensions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_extensions_count,
        path: "/extensions/$count"
    );
}

impl ExtensionsIdApiClient {
    delete!(
        doc: "Delete navigation property extensions for users",
        name: delete_extensions,
        path: "/extensions/{{RID}}"
    );
    get!(
        doc: "Get extensions from users",
        name: get_extensions,
        path: "/extensions/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property extensions in users",
        name: update_extensions,
        path: "/extensions/{{RID}}",
        body: true
    );
}
