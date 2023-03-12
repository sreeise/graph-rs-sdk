// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    FollowedSitesApiClient,
    FollowedSitesIdApiClient,
    ResourceIdentity::FollowedSites
);

impl FollowedSitesApiClient {
    get!(
        doc: "List followed sites",
        name: list_followed_sites,
        path: "/followedSites"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_followed_sites_count,
        path: "/followedSites/$count"
    );
}

impl FollowedSitesIdApiClient {
    get!(
        doc: "Get followedSites from users",
        name: get_followed_sites,
        path: "/followedSites/{{RID}}"
    );
}
