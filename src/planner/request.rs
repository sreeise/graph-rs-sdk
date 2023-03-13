// GENERATED CODE

use crate::api_default_imports::*;
use crate::planner::*;

resource_api_client!(PlannerApiClient, ResourceIdentity::Planner);

impl PlannerApiClient {
    api_client_link_id!(bucket, BucketsIdApiClient);
    api_client_link!(tasks, PlannerTasksApiClient);
    api_client_link!(plans, PlansApiClient);
    api_client_link_id!(plan, PlansIdApiClient);
    api_client_link_id!(task, PlannerTasksIdApiClient);
    api_client_link!(buckets, BucketsApiClient);

    get!(
        doc: "Get planner",
        name: get_planner,
        path: "/planner"
    );
    patch!(
        doc: "Update planner",
        name: update_planner,
        path: "/planner",
        body: true
    );
}
