// GENERATED CODE

use crate::api_default_imports::*;
use crate::planner::*;

resource_api_client!(PlansApiClient, PlansIdApiClient, ResourceIdentity::Plans);

impl PlansApiClient {
    post!(
        doc: "Create plannerPlan",
        name: create_plans,
        path: "/plans",
        body: true
    );
    get!(
        doc: "List plans",
        name: list_plans,
        path: "/plans"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_plans_count,
        path: "/plans/$count"
    );
}

impl PlansIdApiClient {
    api_client_link_id!(task, PlannerTasksIdApiClient);
    api_client_link!(plans, PlansApiClient);
    api_client_link_id!(plan, PlansIdApiClient);
    api_client_link!(tasks, PlannerTasksApiClient);

    delete!(
        doc: "Delete navigation property plans for planner",
        name: delete_plans,
        path: "/plans/{{RID}}"
    );
    get!(
        doc: "Get plans from planner",
        name: get_plans,
        path: "/plans/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property plans in planner",
        name: update_plans,
        path: "/plans/{{RID}}",
        body: true
    );
    delete!(
        doc: "Delete navigation property details for planner",
        name: delete_details,
        path: "/plans/{{RID}}/details"
    );
    get!(
        doc: "Get plannerPlanDetails",
        name: get_details,
        path: "/plans/{{RID}}/details"
    );
    patch!(
        doc: "Update the navigation property details in planner",
        name: update_details,
        path: "/plans/{{RID}}/details",
        body: true
    );
}
