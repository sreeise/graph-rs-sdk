// GENERATED CODE

use crate::api_default_imports::*;
use crate::teams::*;

resource_api_client!(
    TeamsTagsApiClient,
    TeamsTagsIdApiClient,
    ResourceIdentity::TeamsTags
);

impl TeamsTagsApiClient {
    post!(
        doc: "Create teamworkTag",
        name: create_tags,
        path: "/tags",
        body: true
    );
    get!(
        doc: "List teamworkTags",
        name: list_tags,
        path: "/tags"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_tags_count,
        path: "/tags/$count"
    );
}

impl TeamsTagsIdApiClient {
    api_client_link!(member, TeamsMembersIdApiClient);
    api_client_link!(members, TeamsMembersApiClient);

    delete!(
        doc: "Delete navigation property tags for teams",
        name: delete_tags,
        path: "/tags/{{RID}}"
    );
    get!(
        doc: "Get tags from teams",
        name: get_tags,
        path: "/tags/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property tags in teams",
        name: update_tags,
        path: "/tags/{{RID}}",
        body: true
    );
}
