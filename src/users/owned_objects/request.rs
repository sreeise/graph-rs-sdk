// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    OwnedObjectsApiClient,
    OwnedObjectsIdApiClient,
    ResourceIdentity::OwnedObjects
);

impl OwnedObjectsApiClient {
    get!(
        doc: "Get ownedObjects from users",
        name: list_owned_objects,
        path: "/ownedObjects"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_owned_objects_count,
        path: "/ownedObjects/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.application in the microsoft.graph.directoryObject collection",
        name: application_abca,
        path: "/ownedObjects/graph.application"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_application_count,
        path: "/ownedObjects/graph.application/$count"
    );

    get!(
        doc: "Get the number of the resource",
        name: get_service_principal_count,
        path: "/ownedObjects/graph.servicePrincipal/$count"
    );
}

impl OwnedObjectsIdApiClient {
    get!(
        doc: "Get ownedObjects from users",
        name: get_owned_objects,
        path: "/ownedObjects/{{RID}}"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.application",
        name: application_abca,
        path: "/ownedObjects/{{RID}}/graph.application"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.group",
        name: get_directory_object_item_as_group_type,
        path: "/ownedObjects/{{RID}}/graph.group"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.servicePrincipal",
        name: get_directory_object_item_as_service_principal_type,
        path: "/ownedObjects/{{RID}}/graph.servicePrincipal"
    );
}
