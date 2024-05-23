// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;

api_client!(OnenoteApiClient, ResourceIdentity::Onenote);

impl OnenoteApiClient {
    api_client_link!(sections, OnenoteSectionsApiClient);
    api_client_link!(section_groups, OnenoteSectionGroupsApiClient);
    api_client_link_id!(section_group, OnenoteSectionGroupsIdApiClient);
    api_client_link!(pages, OnenotePagesApiClient);
    api_client_link!(notebooks, OnenoteNotebooksApiClient);
    api_client_link_id!(notebook, OnenoteNotebooksIdApiClient);
    api_client_link_id!(section, OnenoteSectionsIdApiClient);
    api_client_link_id!(page, OnenotePagesIdApiClient);

    delete!(
        doc: "Delete navigation property onenote for users",
        name: delete_onenote,
        path: "/onenote"
    );
    get!(
        doc: "Get onenote from users",
        name: get_onenote,
        path: "/onenote"
    );
    patch!(
        doc: "Update the navigation property onenote in users",
        name: update_onenote,
        path: "/onenote",
        body: true
    );
    post!(
        doc: "Create new navigation property to operations for users",
        name: create_operations,
        path: "/onenote/operations",
        body: true
    );
    get!(
        doc: "Get operations from users",
        name: list_operations,
        path: "/onenote/operations"
    );
    get!(
        doc: "Get the number of the resource",
        name: operations_adfa,
        path: "/onenote/operations/$count"
    );
    delete!(
        doc: "Delete navigation property operations for users",
        name: delete_operations,
        path: "/onenote/operations/{{id}}",
        params: onenote_operation_id
    );
    get!(
        doc: "Get operations from users",
        name: get_operations,
        path: "/onenote/operations/{{id}}",
        params: onenote_operation_id
    );
    patch!(
        doc: "Update the navigation property operations in users",
        name: update_operations,
        path: "/onenote/operations/{{id}}",
        body: true,
        params: onenote_operation_id
    );
    post!(
        doc: "Create new navigation property to resources for users",
        name: create_resources,
        path: "/onenote/resources",
        body: true
    );
    get!(
        doc: "Get resources from users",
        name: list_resources,
        path: "/onenote/resources"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_resources_count,
        path: "/onenote/resources/$count"
    );
    delete!(
        doc: "Delete navigation property resources for users",
        name: delete_resources,
        path: "/onenote/resources/{{id}}",
        params: onenote_resource_id
    );
    get!(
        doc: "Get resources from users",
        name: get_resources,
        path: "/onenote/resources/{{id}}",
        params: onenote_resource_id
    );
    patch!(
        doc: "Update the navigation property resources in users",
        name: update_resources,
        path: "/onenote/resources/{{id}}",
        body: true,
        params: onenote_resource_id
    );
    get!(
        doc: "Get content for the navigation property resources from users",
        name: get_resources_content,
        path: "/onenote/resources/{{id}}/content",
        params: onenote_resource_id
    );
    put!(
        doc: "Update content for the navigation property resources in users",
        name: update_resources_content,
        path: "/onenote/resources/{{id}}/content",
        body: true,
        params: onenote_resource_id
    );
}
