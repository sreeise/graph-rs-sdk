// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    MemberOfApiClient,
    MemberOfIdApiClient,
    ResourceIdentity::MemberOf
);

impl MemberOfApiClient {
    get!(
        doc: "Get memberOf from users",
        name: list_member_of,
        path: "/memberOf"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_member_of_count,
        path: "/memberOf/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.group in the microsoft.graph.directoryObject collection",
        name: graph,
        path: "/memberOf/graph.group"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_group_count,
        path: "/memberOf/graph.group/$count"
    );
}

impl MemberOfIdApiClient {
    get!(
        doc: "Get memberOf from users",
        name: get_member_of,
        path: "/memberOf/{{RID}}"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.group",
        name: get_directory_object_item_as_group_type,
        path: "/memberOf/{{RID}}/graph.group"
    );
}
