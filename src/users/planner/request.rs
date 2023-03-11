// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(PlannerApiClient, ResourceIdentity::Planner);

impl PlannerApiClient {
    delete!(
        doc: "Delete navigation property planner for users",
        name: delete_planner,
        path: "/planner"
    );
    get!(
        doc: "Get planner from users",
        name: get_planner,
        path: "/planner"
    );
    patch!(
        doc: "Update the navigation property planner in users",
        name: update_planner,
        path: "/planner",
        body: true
    );
}
