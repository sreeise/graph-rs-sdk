// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
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
        name: as_group,
        path: "/memberOf/graph.group"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_group_count,
        path: "/memberOf/graph.group/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.administrativeUnit in the microsoft.graph.directoryObject collection",
        name: as_administrative_unit,
        path: "/memberOf/graph.administrativeUnit"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_administrative_unit_count,
        path: "/memberOf/graph.administrativeUnit/$count"
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
        name: as_group,
        path: "/memberOf/{{RID}}/graph.group"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.administrativeUnit",
        name: as_administrative_unit,
        path: "/memberOf/{{id}}/graph.administrativeUnit",
        params: directory_object_id
    );
}
