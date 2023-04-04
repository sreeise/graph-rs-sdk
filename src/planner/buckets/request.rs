// GENERATED CODE

use crate::api_default_imports::*;
use crate::planner::*;

resource_api_client!(
    BucketsApiClient,
    BucketsIdApiClient,
    ResourceIdentity::Buckets
);

impl BucketsApiClient {
    post!(
        doc: "Create plannerBucket",
        name: create_buckets,
        path: "/buckets",
        body: true
    );
    get!(
        doc: "List buckets",
        name: list_buckets,
        path: "/buckets"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/buckets/$count"
    );
}

impl BucketsIdApiClient {
    api_client_link_id!(task, PlannerTasksIdApiClient);
    api_client_link!(tasks, PlannerTasksApiClient);

    delete!(
        doc: "Delete navigation property buckets for planner",
        name: delete_buckets,
        path: "/buckets/{{RID}}"
    );
    get!(
        doc: "Get buckets from planner",
        name: get_buckets,
        path: "/buckets/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property buckets in planner",
        name: update_buckets,
        path: "/buckets/{{RID}}",
        body: true
    );
}
