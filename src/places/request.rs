// GENERATED CODE

use crate::api_default_imports::*;

api_client!(PlacesApiClient, PlacesIdApiClient, ResourceIdentity::Places);

impl PlacesApiClient {
    get!(
        doc: "Get the number of the resource",
        name: get_places_count,
        path: "/places/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.room in the microsoft.graph.place collection",
        name: graph,
        path: "/places/graph.room"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_room_count,
        path: "/places/graph.room/$count"
    );
}

impl PlacesIdApiClient {
    delete!(
        doc: "Delete entity from places",
        name: delete_place,
        path: "/places/{{RID}}"
    );
    patch!(
        doc: "Update place",
        name: update_place,
        path: "/places/{{RID}}",
        body: true
    );
    get!(
        doc: "Get the item of type microsoft.graph.place as microsoft.graph.room",
        name: graph,
        path: "/places/{{RID}}/graph.room"
    );
}
