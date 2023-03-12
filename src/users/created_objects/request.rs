// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    CreatedObjectsApiClient,
    CreatedObjectsIdApiClient,
    ResourceIdentity::CreatedObjects
);

impl CreatedObjectsApiClient {
    get!(
        doc: "List createdObjects",
        name: list_created_objects,
        path: "/createdObjects"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_created_objects_count,
        path: "/createdObjects/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.servicePrincipal in the microsoft.graph.directoryObject collection",
        name: graph,
        path: "/createdObjects/graph.servicePrincipal"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_service_principal_count,
        path: "/createdObjects/graph.servicePrincipal/$count"
    );
}

impl CreatedObjectsIdApiClient {
    get!(
        doc: "Get createdObjects from users",
        name: get_created_objects,
        path: "/createdObjects/{{RID}}"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.servicePrincipal",
        name: get_directory_object_item_as_service_principal_type,
        path: "/createdObjects/{{RID}}/graph.servicePrincipal"
    );
}
