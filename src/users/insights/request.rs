// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(InsightsApiClient, ResourceIdentity::Insights);

impl InsightsApiClient {
    delete!(
        doc: "Delete navigation property insights for users",
        name: delete_insights,
        path: "/insights"
    );
    get!(
        doc: "Get insights from users",
        name: get_insights,
        path: "/insights"
    );
    patch!(
        doc: "Update the navigation property insights in users",
        name: update_insights,
        path: "/insights",
        body: true
    );
    post!(
        doc: "Create new navigation property to shared for users",
        name: create_shared,
        path: "/insights/shared",
        body: true
    );
    get!(
        doc: "Get shared from users",
        name: list_shared,
        path: "/insights/shared"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_shared_count,
        path: "/insights/shared/$count"
    );
    delete!(
        doc: "Delete navigation property shared for users",
        name: delete_shared,
        path: "/insights/shared/{{id}}",
        params: shared_insight_id
    );
    get!(
        doc: "Get shared from users",
        name: get_shared,
        path: "/insights/shared/{{id}}",
        params: shared_insight_id
    );
    patch!(
        doc: "Update the navigation property shared in users",
        name: update_shared,
        path: "/insights/shared/{{id}}",
        body: true,
        params: shared_insight_id
    );
    get!(
        doc: "Get lastSharedMethod from users",
        name: get_last_shared_method,
        path: "/insights/shared/{{id}}/lastSharedMethod",
        params: shared_insight_id
    );
    get!(
        doc: "Get resource from users",
        name: get_shared_resource,
        path: "/insights/shared/{{id}}/resource",
        params: shared_insight_id
    );
    post!(
        doc: "Create new navigation property to trending for users",
        name: create_trending,
        path: "/insights/trending",
        body: true
    );
    get!(
        doc: "Get trending from users",
        name: list_trending,
        path: "/insights/trending"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_trending_count,
        path: "/insights/trending/$count"
    );
    delete!(
        doc: "Delete navigation property trending for users",
        name: delete_trending,
        path: "/insights/trending/{{id}}",
        params: trending_id
    );
    get!(
        doc: "Get trending from users",
        name: get_trending,
        path: "/insights/trending/{{id}}",
        params: trending_id
    );
    patch!(
        doc: "Update the navigation property trending in users",
        name: update_trending,
        path: "/insights/trending/{{id}}",
        body: true,
        params: trending_id
    );
    get!(
        doc: "Get resource from users",
        name: get_trending_resource,
        path: "/insights/trending/{{id}}/resource",
        params: trending_id
    );
    post!(
        doc: "Create new navigation property to used for users",
        name: create_used,
        path: "/insights/used",
        body: true
    );
    get!(
        doc: "List used",
        name: list_used,
        path: "/insights/used"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_used_count,
        path: "/insights/used/$count"
    );
    delete!(
        doc: "Delete navigation property used for users",
        name: delete_used,
        path: "/insights/used/{{id}}",
        params: used_insight_id
    );
    get!(
        doc: "Get used from users",
        name: get_used,
        path: "/insights/used/{{id}}",
        params: used_insight_id
    );
    patch!(
        doc: "Update the navigation property used in users",
        name: update_used,
        path: "/insights/used/{{id}}",
        body: true,
        params: used_insight_id
    );
    get!(
        doc: "Get resource from users",
        name: get_used_resource,
        path: "/insights/used/{{id}}/resource",
        params: used_insight_id
    );
}
