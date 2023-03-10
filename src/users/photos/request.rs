// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(PhotosApiClient, PhotosIdApiClient, ResourceIdentity::Photos);

impl PhotosApiClient {
    get!(
        doc: "Get photos from users",
        name: list_photos,
        path: "/photos"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/photos/$count"
    );
}

impl PhotosIdApiClient {
    get!(
        doc: "Get photos from users",
        name: get_photos,
        path: "/photos/{{RID}}"
    );
    get!(
        doc: "Get media content for the navigation property photos from users",
        name: get_photos_content,
        path: "/photos/{{RID}}/$value"
    );
    put!(
        doc: "Update media content for the navigation property photos in users",
        name: update_photos_content,
        path: "/photos/{{RID}}/$value",
        body: true
    );
}
